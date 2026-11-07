//! Predictive risk assessment and spatial hotspot analytics.
//!
//! Evaluates failure probability using asset age, historical fault frequency,
//! vegetation proximity, weather conditions, and load exposure.

use super::model::{FaultPriority, GridAssetType, RiskAssessment, RiskHotspot};
use super::topology::ElectricalNetwork;

/// Assesses predictive grid failure risk and generates maintenance recommendations for `asset_id`.
pub fn assess_asset_risk(
    network: &ElectricalNetwork,
    asset_id: &str,
    current_year: u32,
) -> Result<RiskAssessment, String> {
    let node = network
        .nodes
        .get(asset_id)
        .ok_or_else(|| format!("grid asset '{asset_id}' not found"))?;

    // 1. Historical failure count
    let historical_failures = network.faults.values().filter(|f| f.asset_id == asset_id).count();

    // 2. Asset Age
    let install_year = if node.install_year > 0 { node.install_year } else { 2012 };
    let asset_age = current_year.saturating_sub(install_year).max(1);

    // 3. Connected load & critical customer presence
    let meters = network.get_connected_meters(asset_id);
    let mut critical_count = 0usize;
    let mut daily_revenue = 0.0f64;

    for m in &meters {
        if let Some(c) = network.customers.get(&m.customer_number) {
            if c.is_critical {
                critical_count += 1;
            }
            daily_revenue += c.avg_consumption_kwh_day * c.default_tariff_rate;
        }
    }

    // 4. Environmental risk coefficients
    let weather_risk = 0.35f64; // Moderate seasonal storm probability
    let vegetation_risk = match node.asset_type {
        GridAssetType::LineSegment => 0.65f64,
        GridAssetType::Pole => 0.40f64,
        GridAssetType::Transformer => 0.20f64,
        _ => 0.10f64,
    };

    // Predictive failure probability model
    let mut prob = 0.05f64; // 5% baseline
    prob += historical_failures as f64 * 0.15;
    prob += (asset_age as f64 / 30.0) * 0.25;
    prob += vegetation_risk * 0.20;
    prob += weather_risk * 0.15;

    let failure_prob_pct = (prob * 100.0).round().clamp(5.0, 95.0);

    // Consequence / impact weight
    let mut impact_score = 0.0f64;
    impact_score += (meters.len() as f64 * 0.5).min(40.0);
    impact_score += critical_count as f64 * 20.0;
    impact_score += (daily_revenue / 100.0).min(20.0);

    // Composite risk score: 60% probability, 40% consequence
    let raw_risk = (failure_prob_pct * 0.6) + (impact_score * 0.4);
    let risk_score = raw_risk.round().clamp(0.0, 100.0);

    let risk_level = if risk_score >= 80.0 {
        FaultPriority::Critical
    } else if risk_score >= 60.0 {
        FaultPriority::High
    } else if risk_score >= 35.0 {
        FaultPriority::Medium
    } else {
        FaultPriority::Low
    };

    let mut recommendations = Vec::new();
    if asset_age > 12 {
        recommendations.push("Schedule thermal imaging and insulation degradation testing due to asset age.".to_string());
    }
    if historical_failures >= 2 {
        recommendations.push("Perform comprehensive protection relay audit (repeat fault pattern observed).".to_string());
    }
    if vegetation_risk > 0.5 {
        recommendations.push("Schedule urgent vegetation trimming along overhead line corridor.".to_string());
    }
    if critical_count > 0 {
        recommendations.push("Prioritize for auto-recloser deployment and dual-feed redundancy to safeguard critical facility.".to_string());
    }
    if recommendations.is_empty() {
        recommendations.push("Asset within nominal parameters. Continue standard scheduled maintenance.".to_string());
    }

    Ok(RiskAssessment {
        asset_id: node.asset_id.clone(),
        asset_name: node.name.clone(),
        asset_type: node.asset_type,
        voltage_kv: node.voltage_kv,
        risk_score,
        risk_level,
        failure_probability_pct: failure_prob_pct,
        historical_failure_count: historical_failures,
        asset_age_years: asset_age,
        connected_meters: meters.len(),
        critical_customers: critical_count,
        daily_revenue_exposure: (daily_revenue * 100.0).round() / 100.0,
        weather_risk_factor: weather_risk,
        vegetation_risk_factor: vegetation_risk,
        recommendations,
    })
}

/// Identifies geographic risk hotspots grouped across transformers and primary nodes.
///
/// `metric`:
/// - `"customer_impact"`: concentration of meters & customers
/// - `"risk_score"`: highest composite failure risk
/// - `"revenue_exposure"`: highest economic revenue at risk
pub fn calculate_hotspots(network: &ElectricalNetwork, metric: &str, current_year: u32) -> Vec<RiskHotspot> {
    let mut hotspots = Vec::new();

    for node in network.nodes.values() {
        if node.asset_type != GridAssetType::Transformer && node.asset_type != GridAssetType::Feeder {
            continue;
        }

        let meters = network.get_connected_meters(&node.asset_id);
        if meters.is_empty() {
            continue;
        }

        let mut daily_rev = 0.0f64;
        let mut critical_count = 0usize;

        for m in &meters {
            if let Some(c) = network.customers.get(&m.customer_number) {
                if c.is_critical {
                    critical_count += 1;
                }
                daily_rev += c.avg_consumption_kwh_day * c.default_tariff_rate;
            }
        }

        let risk = assess_asset_risk(network, &node.asset_id, current_year).ok();
        let risk_score = risk.map(|r| r.risk_score).unwrap_or(20.0);

        let intensity = match metric {
            "risk_score" => risk_score,
            "revenue_exposure" => (daily_rev / 50.0).clamp(0.0, 100.0),
            _ => (meters.len() as f64 * 4.0 + critical_count as f64 * 20.0).clamp(0.0, 100.0),
        };

        hotspots.push(RiskHotspot {
            id: format!("HOTSPOT-{}", node.asset_id),
            name: format!("{} Zone", node.name),
            coord: node.coord,
            metric: metric.to_string(),
            intensity_score: (intensity * 10.0).round() / 10.0,
            affected_meters: meters.len(),
            revenue_exposure_daily: (daily_rev * 100.0).round() / 100.0,
        });
    }

    hotspots.sort_by(|a, b| b.intensity_score.partial_cmp(&a.intensity_score).unwrap_or(std::cmp::Ordering::Equal));
    hotspots
}
