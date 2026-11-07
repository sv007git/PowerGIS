//! Fault impact calculation and dynamic priority scoring algorithms.
//!
//! Evaluates customer counts by category, critical facility impact, daily revenue loss,
//! and computes a dynamic priority score (0-100) with explanatory justification.

use serde::{Deserialize, Serialize};

use super::model::{
    CustomerCategory, FaultPriority, FaultRecord, FaultType, GridAssetStatus, GridAssetType, MeterStatus,
};
use super::topology::ElectricalNetwork;

/// Detailed breakdown and analysis of an asset fault condition.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FaultImpactSummary {
    pub asset_id: String,
    pub asset_name: String,
    pub asset_type: GridAssetType,
    pub affected_meters_count: usize,
    pub residential_affected: usize,
    pub commercial_affected: usize,
    pub industrial_affected: usize,
    pub agricultural_affected: usize,
    pub government_affected: usize,
    pub other_affected: usize,
    pub critical_customers_affected: usize,
    pub critical_reasons: Vec<String>,
    pub estimated_revenue_loss_per_day: f64,
    pub priority_score: f64,
    pub priority: FaultPriority,
    pub priority_explanation: String,
    pub affected_meter_ids: Vec<String>,
}

/// Calculates the full impact summary for a hypothetical or actual failure on `asset_id`.
pub fn calculate_impact(
    network: &ElectricalNetwork,
    asset_id: &str,
    _fault_type: FaultType,
) -> Result<FaultImpactSummary, String> {
    let node = network
        .nodes
        .get(asset_id)
        .ok_or_else(|| format!("grid asset '{asset_id}' not found in electrical network"))?;

    let meters = network.get_connected_meters(asset_id);

    let mut residential = 0usize;
    let mut commercial = 0usize;
    let mut industrial = 0usize;
    let mut agricultural = 0usize;
    let mut government = 0usize;
    let mut other = 0usize;
    let mut critical_count = 0usize;
    let mut critical_reasons = Vec::new();
    let mut daily_revenue_loss = 0.0f64;
    let mut meter_ids = Vec::with_capacity(meters.len());

    for m in &meters {
        meter_ids.push(m.meter_number.clone());
        if let Some(cust) = network.customers.get(&m.customer_number) {
            match cust.category {
                CustomerCategory::Residential => residential += 1,
                CustomerCategory::Commercial => commercial += 1,
                CustomerCategory::Industrial => industrial += 1,
                CustomerCategory::Agricultural => agricultural += 1,
                CustomerCategory::Government => government += 1,
                CustomerCategory::Other => other += 1,
            }

            if cust.is_critical {
                critical_count += 1;
                if let Some(reason) = &cust.critical_reason {
                    if !critical_reasons.iter().any(|r| r == reason) {
                        critical_reasons.push(format!("{}: {}", cust.name, reason));
                    }
                }
            }

            let daily_loss = cust.avg_consumption_kwh_day * cust.default_tariff_rate;
            if daily_loss.is_finite() && daily_loss > 0.0 {
                daily_revenue_loss += daily_loss;
            }
        } else {
            other += 1;
        }
    }

    // Dynamic Priority Scoring Algorithm
    // 1. Customer volume impact (max 30 points)
    let volume_score = (meters.len() as f64 * 0.5).min(30.0);

    // 2. Critical customer impact (25 points per critical facility)
    let critical_score = critical_count as f64 * 25.0;

    // 3. Voltage level weighting (up to 30 points)
    let voltage_score = if node.voltage_kv >= 132.0 {
        30.0
    } else if node.voltage_kv >= 33.0 {
        20.0
    } else if node.voltage_kv >= 11.0 {
        10.0
    } else {
        5.0
    };

    // 4. Financial exposure ($1 per $100 daily loss, max 20 points)
    let financial_score = (daily_revenue_loss / 100.0).min(20.0);

    let raw_score = volume_score + critical_score + voltage_score + financial_score;
    let priority_score = (raw_score * 10.0).round() / 10.0;
    let clamped_score = priority_score.clamp(0.0, 100.0);

    let priority = if clamped_score >= 80.0 || critical_count >= 1 {
        FaultPriority::Critical
    } else if clamped_score >= 60.0 {
        FaultPriority::High
    } else if clamped_score >= 35.0 {
        FaultPriority::Medium
    } else {
        FaultPriority::Low
    };

    let explanation = format!(
        "Priority {} (Score: {:.1}/100) based on {} affected meters, {} critical facilities, ${:.2}/day revenue exposure on {} {}.",
        priority.as_str(),
        clamped_score,
        meters.len(),
        critical_count,
        daily_revenue_loss,
        node.asset_type.as_str(),
        node.asset_id
    );

    Ok(FaultImpactSummary {
        asset_id: node.asset_id.clone(),
        asset_name: node.name.clone(),
        asset_type: node.asset_type,
        affected_meters_count: meters.len(),
        residential_affected: residential,
        commercial_affected: commercial,
        industrial_affected: industrial,
        agricultural_affected: agricultural,
        government_affected: government,
        other_affected: other,
        critical_customers_affected: critical_count,
        critical_reasons,
        estimated_revenue_loss_per_day: (daily_revenue_loss * 100.0).round() / 100.0,
        priority_score: clamped_score,
        priority,
        priority_explanation: explanation,
        affected_meter_ids: meter_ids,
    })
}

/// Triggers a fault on `asset_id`: creates fault record, updates node to FAULTED, and sets
/// all impacted meters to OUTAGE.
pub fn trigger_fault(
    network: &mut ElectricalNetwork,
    asset_id: &str,
    fault_type: FaultType,
    now_ms: u64,
) -> Result<FaultRecord, String> {
    let summary = calculate_impact(network, asset_id, fault_type)?;

    let node = network
        .nodes
        .get_mut(asset_id)
        .ok_or_else(|| format!("asset '{asset_id}' not found"))?;

    node.status = GridAssetStatus::Faulted;
    let node_coord = node.coord;
    let node_name = node.name.clone();
    let node_type = node.asset_type;

    // Set all affected meters to OUTAGE
    for meter_id in &summary.affected_meter_ids {
        if let Some(meter) = network.meters.get_mut(meter_id) {
            meter.status = MeterStatus::Outage;
        }
    }

    let fault_count = network.faults.len() + 1;
    let fault_number = format!("FLT-{:04}", fault_count);

    let fault_record = FaultRecord {
        fault_number: fault_number.clone(),
        asset_id: asset_id.to_string(),
        asset_type: node_type,
        asset_name: node_name,
        fault_type,
        status: "ACTIVE".to_string(),
        priority: summary.priority,
        priority_score: summary.priority_score,
        priority_explanation: summary.priority_explanation,
        total_affected_meters: summary.affected_meters_count,
        residential_affected: summary.residential_affected,
        commercial_affected: summary.commercial_affected,
        industrial_affected: summary.industrial_affected,
        agricultural_affected: summary.agricultural_affected,
        government_affected: summary.government_affected,
        other_affected: summary.other_affected,
        critical_customers_affected: summary.critical_customers_affected,
        estimated_revenue_loss_per_day: summary.estimated_revenue_loss_per_day,
        coord: node_coord,
        created_at_ms: now_ms,
        restored_at_ms: None,
    };

    network.faults.insert(fault_number, fault_record.clone());

    Ok(fault_record)
}

/// Restores a registered fault: marks fault RESTORED, node ACTIVE, and restores all connected meters.
pub fn restore_fault(
    network: &mut ElectricalNetwork,
    fault_number: &str,
    now_ms: u64,
) -> Result<FaultRecord, String> {
    let fault = network
        .faults
        .get_mut(fault_number)
        .ok_or_else(|| format!("fault '{fault_number}' not found"))?;

    fault.status = "RESTORED".to_string();
    fault.restored_at_ms = Some(now_ms);
    let asset_id = fault.asset_id.clone();
    let record_clone = fault.clone();

    // Restore node status
    if let Some(node) = network.nodes.get_mut(&asset_id) {
        node.status = GridAssetStatus::Active;
    }

    // Restore all connected meters
    let connected = network.get_connected_meters(&asset_id);
    let meter_numbers: Vec<String> = connected.iter().map(|m| m.meter_number.clone()).collect();
    for num in meter_numbers {
        if let Some(m) = network.meters.get_mut(&num) {
            m.status = MeterStatus::Active;
        }
    }

    // Release any assigned crew
    for crew in network.crews.values_mut() {
        if crew.current_fault_number.as_deref() == Some(fault_number) {
            crew.current_fault_number = None;
            crew.status = super::model::CrewStatus::Available;
        }
    }

    Ok(record_clone)
}
