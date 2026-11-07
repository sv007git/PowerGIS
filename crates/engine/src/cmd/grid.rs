//! Analyze › Grid Intelligence: electrical distribution grid fault detection,
//! impact analysis, risk prediction, power flow, and crew dispatch (GridSense).

use powergis_analysis::grid::{
    CrewStatus, FaultType, MeterStatus, MeterTelemetry, UserRole, all_grid_datasets, assess_asset_risk, assign_crew,
    calculate_hotspots, calculate_impact, create_sample_network, ingest_telemetry_batch, restore_fault,
    simulate_power_flow, trigger_fault,
};
use powergis_map::{Layer, LayerGroup};
use serde_json::{Value, json};

use super::{f, ps, uint};
use crate::project_log::now_ms;
use crate::{CmdResult, CommandSpec as C, Session};

pub fn commands() -> Vec<C> {
    vec![
        C::new("grid.sample", "Load Sample Grid", "Analyze › Grid Intelligence", sample)
            .params(r#"{}"#),
        C::new("grid.createLayers", "Create Grid Layers", "Analyze › Grid Intelligence", create_layers)
            .params(r#"{"role"?: "ADMIN" (default) | "DISPATCHER" | "FIELD_ENGINEER" | "GUEST"}"#),
        C::new("grid.status", "Grid Operational Status", "Analyze › Grid Intelligence", status)
            .view()
            .params(r#"{}"#),
        C::new("grid.analyzeImpact", "Analyze Fault Impact", "Analyze › Grid Intelligence", analyze_impact)
            .view()
            .params(r#"{"assetId": "T-104" | string, "faultType"?: "TRANSFORMER_FAILURE" | "LINE_BREAK" | "THREE_PHASE" | "SINGLE_PHASE_GROUND" | "VEGETATION_INTERFERENCE"}"#),
        C::new("grid.triggerFault", "Trigger Fault", "Analyze › Grid Intelligence", run_trigger_fault)
            .params(r#"{"assetId": "T-104" | string, "faultType"?: "TRANSFORMER_FAILURE" (default) | "LINE_BREAK" | "THREE_PHASE" | "SINGLE_PHASE_GROUND" | "VEGETATION_INTERFERENCE"}"#),
        C::new("grid.restoreFault", "Restore Fault", "Analyze › Grid Intelligence", run_restore_fault)
            .params(r#"{"faultNumber": "FLT-0001" | string}"#),
        C::new("grid.assessRisk", "Assess Asset Risk", "Analyze › Grid Intelligence", assess_risk)
            .view()
            .params(r#"{"assetId": "T-104" | string, "currentYear"?: number}"#),
        C::new("grid.hotspots", "Risk Hotspots", "Analyze › Grid Intelligence", hotspots)
            .view()
            .params(r#"{"metric"?: "customer_impact" (default) | "risk_score" | "revenue_exposure", "currentYear"?: number}"#),
        C::new("grid.powerFlow", "Power Flow Simulation", "Analyze › Grid Intelligence", power_flow)
            .view()
            .params(r#"{"substationId"?: "SUB-01" | string}"#),
        C::new("grid.assignCrew", "Assign Response Crew", "Analyze › Grid Intelligence", run_assign_crew)
            .params(r#"{"faultNumber": "FLT-0001" | string, "crewCode"?: "CREW-ALPHA" | string (empty: auto-select best)}"#),
        C::new("grid.ingestTelemetry", "Ingest SCADA Telemetry", "Analyze › Grid Intelligence", ingest_telemetry)
            .params(r#"{"readings": [{"meterNumber": string, "voltage": number, "current"?: number, "power"?: number, "powerFactor"?: number}]}"#),
        C::new("grid.networkTree", "Digital Twin Network Tree", "Analyze › Grid Intelligence", network_tree)
            .view()
            .params(r#"{"rootId"?: "SUB-01" (default), "role"?: "ADMIN" | "DISPATCHER" | "FIELD_ENGINEER" | "GUEST"}"#),
    ]
}

/// Parses user role from parameters.
fn parse_role(p: &Value) -> UserRole {
    match ps(p, "role").unwrap_or("ADMIN").to_ascii_uppercase().as_str() {
        "GUEST" => UserRole::Guest,
        "FIELD_ENGINEER" | "FIELD" => UserRole::FieldEngineer,
        "DISPATCHER" => UserRole::Dispatcher,
        _ => UserRole::Admin,
    }
}

/// Loads or resets the GridSense sample network in session memory.
fn sample(s: &mut Session, _p: &Value) -> CmdResult {
    s.grid = Some(create_sample_network());
    let net = s.grid.as_ref().ok_or_else(|| "failed to initialize grid network".to_string())?;

    Ok(json!({
        "status": "LOADED",
        "nodes": net.nodes.len(),
        "edges": net.edges.len(),
        "meters": net.meters.len(),
        "customers": net.customers.len(),
        "crews": net.crews.len(),
        "message": "Loaded reference Juru/Goromonzi 132kV/33kV/11kV electrical distribution network",
    }))
}

const GRID_LAYER_NAMES: &[&str] = &[
    "Grid Substations",
    "Grid Feeders",
    "Power Lines & Segments",
    "Distribution Transformers",
    "Distribution Poles",
    "Smart Meters",
    "Grid Faults",
    "Response Crews",
];

/// Generates all 8 grid vector datasets and adds them as live layers to the active map.
/// Replaces any existing grid layers so they populate only once without duplication,
/// and organizes them under a dedicated "Grid Intelligence" LayerGroup.
fn create_layers(s: &mut Session, p: &Value) -> CmdResult {
    let role = parse_role(p);
    let net = s.grid_mut().clone();

    let datasets = all_grid_datasets(&net, role);
    let mut layer_ids = Vec::new();
    let mut added_layer_ids = Vec::new();

    let map = s.map_mut();

    // 1. Clean up existing grid layers and group so layers never duplicate
    map.layers.retain(|l| !GRID_LAYER_NAMES.contains(&l.name.as_str()));
    map.groups.retain(|g| g.name != "Grid Intelligence");

    // 2. Add fresh grid layers
    for ds in datasets {
        let name = ds.name.clone();
        let layer_id = map.add_layer(Layer::new(0, ds));
        layer_ids.push(layer_id);
        added_layer_ids.push(json!({
            "id": layer_id,
            "name": name,
        }));
    }

    // 3. Group grid layers together in the Layers pane
    let mut grid_group = LayerGroup::new("Grid Intelligence");
    grid_group.children = layer_ids;
    grid_group.expanded = true;
    let group_id = map.add_group(grid_group);

    Ok(json!({
        "layersAdded": added_layer_ids.len(),
        "groupId": group_id,
        "groupName": "Grid Intelligence",
        "layers": added_layer_ids,
        "role": format!("{:?}", role),
    }))
}

/// Returns real-time KPI metrics for control room dashboard.
fn status(s: &mut Session, _p: &Value) -> CmdResult {
    let net = s.grid_mut();

    let active_faults: Vec<_> = net.faults.values().filter(|f| f.status == "ACTIVE").collect();
    let total_affected_meters: usize = active_faults.iter().map(|f| f.total_affected_meters).sum();
    let critical_affected: usize = active_faults.iter().map(|f| f.critical_customers_affected).sum();
    let total_revenue_loss: f64 = active_faults.iter().map(|f| f.estimated_revenue_loss_per_day).sum();
    let available_crews = net.crews.values().filter(|c| c.status == CrewStatus::Available).count();
    let active_meters = net.meters.values().filter(|m| m.status == MeterStatus::Active).count();
    let outage_meters = net.meters.values().filter(|m| m.status == MeterStatus::Outage).count();

    Ok(json!({
        "activeFaultsCount": active_faults.len(),
        "totalAffectedMeters": total_affected_meters,
        "criticalCustomersImpacted": critical_affected,
        "totalDailyRevenueLoss": (total_revenue_loss * 100.0).round() / 100.0,
        "availableCrews": available_crews,
        "totalCrews": net.crews.len(),
        "activeMeters": active_meters,
        "outageMeters": outage_meters,
        "totalNodes": net.nodes.len(),
    }))
}

/// What-if fault impact analysis without mutating live state.
fn analyze_impact(s: &mut Session, p: &Value) -> CmdResult {
    let asset_id = ps(p, "assetId").ok_or_else(|| "assetId parameter is required".to_string())?;
    let fault_type = ps(p, "faultType")
        .map(FaultType::from_str_lossy)
        .unwrap_or(FaultType::TransformerFailure);

    let net = s.grid_mut();
    let summary = calculate_impact(net, asset_id, fault_type)?;

    serde_json::to_value(&summary).map_err(|e| format!("serialization error: {e}"))
}

/// Triggers a live or simulated fault on a grid asset.
fn run_trigger_fault(s: &mut Session, p: &Value) -> CmdResult {
    let asset_id = ps(p, "assetId").ok_or_else(|| "assetId parameter is required".to_string())?;
    let fault_type = ps(p, "faultType")
        .map(FaultType::from_str_lossy)
        .unwrap_or(FaultType::TransformerFailure);

    let ts = uint(p, "timestamp").unwrap_or_else(now_ms);
    let net = s.grid_mut();

    let fault_record = trigger_fault(net, asset_id, fault_type, ts)?;

    serde_json::to_value(&fault_record).map_err(|e| format!("serialization error: {e}"))
}

/// Restores a registered fault and reconnects affected customers.
fn run_restore_fault(s: &mut Session, p: &Value) -> CmdResult {
    let fault_number = ps(p, "faultNumber").ok_or_else(|| "faultNumber parameter is required".to_string())?;
    let ts = uint(p, "timestamp").unwrap_or_else(now_ms);

    let net = s.grid_mut();
    let record = restore_fault(net, fault_number, ts)?;

    serde_json::to_value(&record).map_err(|e| format!("serialization error: {e}"))
}

/// Evaluates predictive failure risk on an asset.
fn assess_risk(s: &mut Session, p: &Value) -> CmdResult {
    let asset_id = ps(p, "assetId").ok_or_else(|| "assetId parameter is required".to_string())?;
    let current_year = uint(p, "currentYear").unwrap_or(2026) as u32;

    let net = s.grid_mut();
    let assessment = assess_asset_risk(net, asset_id, current_year)?;

    serde_json::to_value(&assessment).map_err(|e| format!("serialization error: {e}"))
}

/// Computes geographic concentration hotspots across the grid.
fn hotspots(s: &mut Session, p: &Value) -> CmdResult {
    let metric = ps(p, "metric").unwrap_or("customer_impact");
    let current_year = uint(p, "currentYear").unwrap_or(2026) as u32;

    let net = s.grid_mut();
    let list = calculate_hotspots(net, metric, current_year);

    serde_json::to_value(&list).map_err(|e| format!("serialization error: {e}"))
}

/// Runs advanced radial power flow simulation (Roadmap Feature).
fn power_flow(s: &mut Session, p: &Value) -> CmdResult {
    let sub_id = ps(p, "substationId");
    let net = s.grid_mut();

    let summary = simulate_power_flow(net, sub_id)?;
    serde_json::to_value(&summary).map_err(|e| format!("serialization error: {e}"))
}

/// Dispatches response crew using spatial proximity and skill matching (Roadmap Feature).
fn run_assign_crew(s: &mut Session, p: &Value) -> CmdResult {
    let fault_number = ps(p, "faultNumber").ok_or_else(|| "faultNumber parameter is required".to_string())?;
    let crew_code = ps(p, "crewCode");
    let ts = uint(p, "timestamp").unwrap_or_else(now_ms);

    let net = s.grid_mut();
    let assignment = assign_crew(net, crew_code, fault_number, ts)?;

    serde_json::to_value(&assignment).map_err(|e| format!("serialization error: {e}"))
}

/// Ingests SCADA telemetry readings and returns automated anomaly detections (Roadmap Feature).
fn ingest_telemetry(s: &mut Session, p: &Value) -> CmdResult {
    let readings_arr = p
        .get("readings")
        .and_then(Value::as_array)
        .ok_or_else(|| "readings array required".to_string())?;

    let mut telemetry_list = Vec::with_capacity(readings_arr.len());
    let default_ts = now_ms();

    for item in readings_arr {
        let meter_number = ps(item, "meterNumber").unwrap_or("").to_string();
        if meter_number.is_empty() {
            continue;
        }

        let voltage_v = f(item, "voltage").unwrap_or(230.0);
        let current_a = f(item, "current").unwrap_or(5.0);
        let active_power_kw = f(item, "power").unwrap_or(voltage_v * current_a / 1000.0);
        let power_factor = f(item, "powerFactor").unwrap_or(0.95);
        let timestamp_ms = uint(item, "timestamp").unwrap_or(default_ts);

        telemetry_list.push(MeterTelemetry {
            meter_number,
            voltage_v,
            current_a,
            active_power_kw,
            energy_kwh: 100.0,
            power_factor,
            timestamp_ms,
        });
    }

    let net = s.grid_mut();
    let anomalies = ingest_telemetry_batch(net, telemetry_list);

    serde_json::to_value(&anomalies).map_err(|e| format!("serialization error: {e}"))
}

/// Generates hierarchical Digital Twin network tree.
fn network_tree(s: &mut Session, p: &Value) -> CmdResult {
    let root_id = ps(p, "rootId").unwrap_or("SUB-01");
    let role = parse_role(p);

    let net = s.grid_mut();
    let tree = net
        .build_network_tree(root_id, role)
        .ok_or_else(|| format!("failed to build network tree from root '{root_id}'"))?;

    Ok(tree)
}
