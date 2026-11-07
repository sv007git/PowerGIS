//! Integration tests for GridSense electrical distribution grid intelligence in PowerGIS.

use rustgis_engine::{Project, Session};
use serde_json::json;

fn session() -> Session {
    Session::new(Project::default())
}

#[test]
fn grid_sample_initialization_command() {
    let mut s = session();
    let res = s.run("grid.sample", &json!({})).unwrap();

    assert_eq!(res["status"], "LOADED");
    assert!(res["nodes"].as_u64().unwrap() >= 20);
    assert!(res["meters"].as_u64().unwrap() >= 15);
    assert!(res["crews"].as_u64().unwrap() >= 3);
}

#[test]
fn grid_create_layers_on_map() {
    let mut s = session();
    let res = s.run("grid.createLayers", &json!({"role": "DISPATCHER"})).unwrap();

    assert_eq!(res["layersAdded"], 8);
    assert_eq!(s.map().layers.len(), 8);

    let layer_names: Vec<&str> = s.map().layers.iter().map(|l| l.name.as_str()).collect();
    assert!(layer_names.contains(&"Grid Substations"));
    assert!(layer_names.contains(&"Grid Feeders"));
    assert!(layer_names.contains(&"Power Lines & Segments"));
    assert!(layer_names.contains(&"Distribution Transformers"));
    assert!(layer_names.contains(&"Distribution Poles"));
    assert!(layer_names.contains(&"Smart Meters"));
    assert!(layer_names.contains(&"Grid Faults"));
    assert!(layer_names.contains(&"Response Crews"));
}

#[test]
fn grid_status_kpis() {
    let mut s = session();
    let res = s.run("grid.status", &json!({})).unwrap();

    assert_eq!(res["activeFaultsCount"], 0);
    assert!(res["totalNodes"].as_u64().unwrap() >= 20);
    assert!(res["activeMeters"].as_u64().unwrap() >= 15);
    assert_eq!(res["outageMeters"], 0);
    assert!(res["availableCrews"].as_u64().unwrap() >= 3);
}

#[test]
fn grid_analyze_impact_what_if() {
    let mut s = session();
    let res = s.run("grid.analyzeImpact", &json!({"assetId": "T-104"})).unwrap();

    assert_eq!(res["asset_id"], "T-104");
    assert!(res["affected_meters_count"].as_u64().unwrap() >= 10);
    assert_eq!(res["critical_customers_affected"], 2); // Hospital + Police station
    assert_eq!(res["priority"], "CRITICAL");
    assert!(res["priority_score"].as_f64().unwrap() >= 80.0);
    assert!(res["estimated_revenue_loss_per_day"].as_f64().unwrap() > 50.0);

    // Verify what-if did not mutate the state
    let status_res = s.run("grid.status", &json!({})).unwrap();
    assert_eq!(status_res["activeFaultsCount"], 0);
}

#[test]
fn grid_fault_lifecycle_trigger_assign_restore() {
    let mut s = session();

    // 1. Trigger Fault on T-104
    let fault = s
        .run(
            "grid.triggerFault",
            &json!({
                "assetId": "T-104",
                "faultType": "TRANSFORMER_FAILURE"
            }),
        )
        .unwrap();

    let fault_num = fault["fault_number"].as_str().unwrap();
    assert_eq!(fault["status"], "ACTIVE");
    assert_eq!(fault["priority"], "CRITICAL");

    // Check status: 1 active fault, meters in outage
    let status1 = s.run("grid.status", &json!({})).unwrap();
    assert_eq!(status1["activeFaultsCount"], 1);
    assert!(status1["outageMeters"].as_u64().unwrap() >= 10);

    // 2. Dispatch Crew
    let assign = s
        .run(
            "grid.assignCrew",
            &json!({
                "faultNumber": fault_num
            }),
        )
        .unwrap();

    assert_eq!(assign["status"], "ASSIGNED");
    assert_eq!(assign["crew_code"], "CREW-ALPHA"); // Auto-matched Rapid Response Alpha
    assert!(assign["distance_km"].as_f64().unwrap() > 0.0);
    assert!(assign["estimated_restoration_minutes"].as_f64().unwrap() > 60.0);

    // 3. Restore Fault
    let restore = s
        .run(
            "grid.restoreFault",
            &json!({
                "faultNumber": fault_num
            }),
        )
        .unwrap();

    assert_eq!(restore["status"], "RESTORED");

    // Status: 0 active faults, all meters restored
    let status2 = s.run("grid.status", &json!({})).unwrap();
    assert_eq!(status2["activeFaultsCount"], 0);
    assert_eq!(status2["outageMeters"], 0);
    assert!(status2["availableCrews"].as_u64().unwrap() >= 3);
}

#[test]
fn grid_risk_assessment_and_hotspots() {
    let mut s = session();

    // Assess risk on T-104
    let risk = s
        .run(
            "grid.assessRisk",
            &json!({
                "assetId": "T-104",
                "currentYear": 2026
            }),
        )
        .unwrap();

    assert_eq!(risk["asset_id"], "T-104");
    assert!(risk["risk_score"].as_f64().unwrap() > 30.0);
    assert!(risk["failure_probability_pct"].as_f64().unwrap() > 10.0);
    assert!(!risk["recommendations"].as_array().unwrap().is_empty());

    // Calculate hotspots
    let hotspots = s.run("grid.hotspots", &json!({"metric": "customer_impact"})).unwrap();
    let arr = hotspots.as_array().unwrap();
    assert!(!arr.is_empty());
    assert!(arr[0]["intensity_score"].as_f64().unwrap() > 0.0);
}

#[test]
fn grid_power_flow_simulation() {
    let mut s = session();
    let flow = s.run("grid.powerFlow", &json!({"substationId": "SUB-01"})).unwrap();

    assert_eq!(flow["substation_id"], "SUB-01");
    assert!(flow["total_load_kw"].as_f64().unwrap() > 0.0);
    assert!(flow["overall_efficiency_pct"].as_f64().unwrap() > 80.0);
    assert!(!flow["transformer_loading"].as_array().unwrap().is_empty());
}

#[test]
fn grid_scada_telemetry_stream() {
    let mut s = session();
    let telemetry_input = json!({
        "readings": [
            {
                "meterNumber": "MTR-HOSP-01",
                "voltage": 195.0,
                "current": 14.2,
                "powerFactor": 0.96
            },
            {
                "meterNumber": "MTR-RES-004",
                "voltage": 0.0,
                "current": 0.0,
                "powerFactor": 1.0
            }
        ]
    });

    let res = s.run("grid.ingestTelemetry", &telemetry_input).unwrap();
    let anomalies = res.as_array().unwrap();

    assert_eq!(anomalies.len(), 2);
    assert!(anomalies.iter().any(|a| a["anomaly_type"] == "UNDER_VOLTAGE"));
    assert!(anomalies.iter().any(|a| a["anomaly_type"] == "OUTAGE_DETECTED"));
}

#[test]
fn grid_digital_twin_tree_with_masking() {
    let mut s = session();

    // 1. Unmasked for DISPATCHER
    let tree_disp = s.run("grid.networkTree", &json!({"rootId": "SUB-01", "role": "DISPATCHER"})).unwrap();
    assert!(tree_disp.to_string().contains("Juru General Hospital"));

    // 2. Masked for FIELD_ENGINEER
    let tree_eng = s.run("grid.networkTree", &json!({"rootId": "SUB-01", "role": "FIELD_ENGINEER"})).unwrap();
    assert!(!tree_eng.to_string().contains("Juru General Hospital"));
    assert!(tree_eng.to_string().contains("J*** G****** H*******"));
}
