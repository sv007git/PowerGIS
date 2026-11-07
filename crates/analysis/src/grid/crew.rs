//! Crew dispatch and spatial routing optimization (Roadmap Feature).
//!
//! Uses geodesic distance, crew availability, and skill matching to optimize
//! emergency response dispatch and compute estimated restoration times (ETR).

use rustgis_geom::haversine_m;

use super::model::{Crew, CrewAssignment, CrewStatus, FaultType, GridAssetType};
use super::topology::ElectricalNetwork;

/// Result of crew dispatch recommendation.
#[derive(Clone, Debug, PartialEq)]
pub struct CrewRecommendation {
    pub crew: Crew,
    pub distance_km: f64,
    pub travel_minutes: f64,
    pub repair_minutes: f64,
    pub total_estimated_minutes: f64,
    pub skill_matched: bool,
}

/// Finds the optimal available response crew for a registered active fault.
pub fn find_best_crew(network: &ElectricalNetwork, fault_number: &str) -> Result<CrewRecommendation, String> {
    let fault = network
        .faults
        .get(fault_number)
        .ok_or_else(|| format!("fault '{fault_number}' not found"))?;

    let required_skill = match fault.asset_type {
        GridAssetType::Substation | GridAssetType::Transformer => "TRANSFORMER",
        GridAssetType::LineSegment | GridAssetType::Feeder => "LINE",
        _ => "METER",
    };

    let mut candidates = Vec::new();

    for crew in network.crews.values() {
        if crew.status != CrewStatus::Available {
            continue;
        }

        let distance_m = haversine_m(crew.coord, fault.coord);
        let distance_km = (distance_m / 1000.0 * 10.0).round() / 10.0;

        // Skill match weighting
        let has_skill = crew.skills.iter().any(|s| s.to_ascii_uppercase().contains(required_skill));

        // Travel time based on average 40 km/h urban/rural vehicle speed
        let travel_min = (distance_km / 40.0 * 60.0).max(5.0);
        let repair_min = fault.fault_type.base_restoration_minutes();
        let total_min = travel_min + repair_min;

        candidates.push(CrewRecommendation {
            crew: crew.clone(),
            distance_km,
            travel_minutes: (travel_min * 10.0).round() / 10.0,
            repair_minutes: repair_min,
            total_estimated_minutes: (total_min * 10.0).round() / 10.0,
            skill_matched: has_skill,
        });
    }

    if candidates.is_empty() {
        return Err("no available response crews currently found on duty".to_string());
    }

    // Sort by skill match first, then by total estimated time (travel + repair)
    candidates.sort_by(|a, b| {
        match b.skill_matched.cmp(&a.skill_matched) {
            std::cmp::Ordering::Equal => {
                a.total_estimated_minutes
                    .partial_cmp(&b.total_estimated_minutes)
                    .unwrap_or(std::cmp::Ordering::Equal)
            }
            other => other,
        }
    });

    candidates.first().cloned().ok_or_else(|| "crew selection failed".to_string())
}

/// Assigns a specific crew (or auto-selected best crew) to `fault_number`.
pub fn assign_crew(
    network: &mut ElectricalNetwork,
    crew_code: Option<&str>,
    fault_number: &str,
    now_ms: u64,
) -> Result<CrewAssignment, String> {
    let chosen = match crew_code {
        Some(code) => {
            let crew = network.crews.get(code).ok_or_else(|| format!("crew '{code}' not found"))?;
            let fault = network.faults.get(fault_number).ok_or_else(|| format!("fault '{fault_number}' not found"))?;
            let distance_m = haversine_m(crew.coord, fault.coord);
            let distance_km = (distance_m / 1000.0 * 10.0).round() / 10.0;
            let travel_min = (distance_km / 40.0 * 60.0).max(5.0);
            let repair_min = fault.fault_type.base_restoration_minutes();
            CrewRecommendation {
                crew: crew.clone(),
                distance_km,
                travel_minutes: (travel_min * 10.0).round() / 10.0,
                repair_minutes: repair_min,
                total_estimated_minutes: ((travel_min + repair_min) * 10.0).round() / 10.0,
                skill_matched: true,
            }
        }
        None => find_best_crew(network, fault_number)?,
    };

    let target_code = chosen.crew.crew_code.clone();

    // Update crew in network
    if let Some(crew) = network.crews.get_mut(&target_code) {
        crew.status = CrewStatus::EnRoute;
        crew.current_fault_number = Some(fault_number.to_string());
    }

    let assignment = CrewAssignment {
        crew_code: target_code,
        fault_number: fault_number.to_string(),
        distance_km: chosen.distance_km,
        estimated_travel_minutes: chosen.travel_minutes,
        estimated_restoration_minutes: chosen.total_estimated_minutes,
        status: "ASSIGNED".to_string(),
        assigned_at_ms: now_ms,
    };

    network.assignments.push(assignment.clone());

    Ok(assignment)
}
