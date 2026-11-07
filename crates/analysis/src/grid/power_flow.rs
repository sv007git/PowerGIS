//! Advanced radial distribution power flow simulation (Roadmap Feature).
//!
//! Models voltage drop profiles along feeders, transformer loading percentages,
//! technical line losses (I²R), and flags ANSI/IEC voltage standard violations.

use super::model::{GridAssetType, PowerFlowSummary, TransformerLoading, VoltageViolation};
use super::topology::ElectricalNetwork;

/// Executes a radial power flow analysis starting from `substation_id` (or the first active substation).
pub fn simulate_power_flow(network: &ElectricalNetwork, substation_id: Option<&str>) -> Result<PowerFlowSummary, String> {
    let sub_id = match substation_id {
        Some(id) => id.to_string(),
        None => {
            network
                .nodes
                .values()
                .find(|n| n.asset_type == GridAssetType::Substation)
                .map(|n| n.asset_id.clone())
                .ok_or_else(|| "no substation found in electrical network to run power flow".to_string())?
        }
    };

    let sub_node = network
        .nodes
        .get(&sub_id)
        .ok_or_else(|| format!("substation '{sub_id}' not found"))?;

    let base_kv = if sub_node.voltage_kv > 0.0 { sub_node.voltage_kv } else { 33.0 };
    let mut total_load_kw = 0.0f64;
    let mut total_losses_kw = 0.0f64;

    let mut transformer_loadings = Vec::new();
    let mut voltage_violations = Vec::new();

    // 1. Calculate Transformer Loadings & Connected Customer Loads
    for node in network.nodes.values() {
        if node.asset_type != GridAssetType::Transformer {
            continue;
        }

        let meters = network.get_connected_meters(&node.asset_id);
        let mut node_kwh_day = 0.0f64;
        for m in &meters {
            if let Some(c) = network.customers.get(&m.customer_number) {
                node_kwh_day += c.avg_consumption_kwh_day;
            } else {
                node_kwh_day += 15.0; // Default residential
            }
        }

        // Peak coincidence load factor: assume peak load is ~2.5x average hourly load
        // Avg kW = kWh_day / 24, Peak kW = Avg kW * 2.5
        let peak_kw = (node_kwh_day / 24.0) * 2.5;
        let power_factor = 0.92f64;
        let load_kva = if power_factor > 0.0 { peak_kw / power_factor } else { peak_kw };
        total_load_kw += peak_kw;

        let rated_kva = if node.capacity_kva > 0.0 { node.capacity_kva } else { 315.0 };
        let util_pct = ((load_kva / rated_kva) * 100.0 * 10.0).round() / 10.0;
        let is_overloaded = util_pct > 100.0;

        transformer_loadings.push(TransformerLoading {
            transformer_id: node.asset_id.clone(),
            name: node.name.clone(),
            rated_capacity_kva: rated_kva,
            connected_load_kva: (load_kva * 10.0).round() / 10.0,
            utilization_pct: util_pct,
            is_overloaded,
        });
    }

    transformer_loadings.sort_by(|a, b| b.utilization_pct.partial_cmp(&a.utilization_pct).unwrap_or(std::cmp::Ordering::Equal));

    // 2. Line Losses & Voltage Drop along Feeders and Line Segments
    for edge in &network.edges {
        let downstream_assets = network.get_downstream_asset_ids(&edge.to_node, 12);
        let mut downstream_load_kw = 0.0f64;

        for asset in &downstream_assets {
            let meters = network.get_connected_meters(asset);
            for m in meters {
                let daily = network
                    .customers
                    .get(&m.customer_number)
                    .map(|c| c.avg_consumption_kwh_day)
                    .unwrap_or(15.0);
                downstream_load_kw += (daily / 24.0) * 2.5;
            }
        }

        let line_kv = base_kv;
        // 3-phase current: I = P / (sqrt(3) * V_LL * PF)
        let sqrt3 = 1.7320508f64;
        let pf = 0.92f64;
        let current_a = if line_kv > 0.0 {
            (downstream_load_kw * 1000.0) / (sqrt3 * line_kv * 1000.0 * pf)
        } else {
            0.0
        };

        // Resistance in ohms
        let resistance = if edge.resistance_ohm > 0.0 {
            edge.resistance_ohm
        } else {
            // Default ~0.3 ohm/km for standard ACSR overhead conductor
            (edge.length_m / 1000.0) * 0.35
        };

        // Technical Line Loss: 3 * I² * R in kW
        let line_loss_kw = (3.0 * current_a.powi(2) * resistance) / 1000.0;
        total_losses_kw += line_loss_kw;

        // Voltage drop: delta_V = sqrt(3) * I * R
        let delta_v_volts = sqrt3 * current_a * resistance;
        let nominal_v = line_kv * 1000.0;
        let actual_v = (nominal_v - delta_v_volts).max(0.0);
        let pu_v = if nominal_v > 0.0 { actual_v / nominal_v } else { 1.0 };
        let drop_pct = ((nominal_v - actual_v) / nominal_v) * 100.0;

        if pu_v < 0.95 {
            let violation_tier = if pu_v < 0.90 { "CRITICAL_UNDER_VOLTAGE" } else { "WARNING_UNDER_VOLTAGE" };
            voltage_violations.push(VoltageViolation {
                node_id: edge.to_node.clone(),
                name: network.nodes.get(&edge.to_node).map(|n| n.name.clone()).unwrap_or_else(|| edge.to_node.clone()),
                nominal_kv: line_kv,
                actual_kv: (actual_v / 1000.0 * 100.0).round() / 100.0,
                per_unit_voltage: (pu_v * 1000.0).round() / 1000.0,
                drop_pct: (drop_pct * 10.0).round() / 10.0,
                violation_tier: violation_tier.to_string(),
            });
        }
    }

    let overall_efficiency_pct = if total_load_kw + total_losses_kw > 0.0 {
        ((total_load_kw / (total_load_kw + total_losses_kw)) * 100.0 * 10.0).round() / 10.0
    } else {
        100.0
    };

    Ok(PowerFlowSummary {
        substation_id: sub_id,
        total_generation_kwh: ((total_load_kw + total_losses_kw) * 10.0).round() / 10.0,
        total_load_kw: (total_load_kw * 10.0).round() / 10.0,
        total_losses_kw: (total_losses_kw * 10.0).round() / 10.0,
        overall_efficiency_pct,
        transformer_loading: transformer_loadings,
        voltage_violations,
    })
}
