//! Electrical distribution grid intelligence suite (GridSense).
//!
//! Provides real-time fault detection and management for electrical distribution
//! networks: graph topology traversal, smart meter telemetry ingestion, impact
//! calculation, dynamic priority scoring, predictive risk hotspots, radial power
//! flow simulation, crew dispatch optimization, and native PowerGIS vector layer generation.

pub mod crew;
pub mod impact;
pub mod layers;
pub mod model;
pub mod power_flow;
pub mod risk;
pub mod sample;
pub mod telemetry;
pub mod topology;

#[cfg(test)]
mod tests;

pub use crew::{CrewRecommendation, assign_crew, find_best_crew};
pub use impact::{FaultImpactSummary, calculate_impact, restore_fault, trigger_fault};
pub use layers::{
    all_grid_datasets, crews_dataset, faults_dataset, feeders_dataset, meters_dataset, poles_dataset,
    power_lines_dataset, substations_dataset, transformers_dataset,
};
pub use model::{
    Crew, CrewAssignment, CrewStatus, CustomerCategory, CustomerProfile, FaultPriority, FaultRecord,
    FaultType, GridAssetStatus, GridAssetType, GridEdge, GridEdgeType, GridNode, MeterStatus, MeterTelemetry,
    PowerFlowSummary, RiskAssessment, RiskHotspot, SmartMeter, TelemetryAnomaly, TransformerLoading, UserRole,
    VoltageViolation,
};
pub use power_flow::simulate_power_flow;
pub use risk::{assess_asset_risk, calculate_hotspots};
pub use sample::create_sample_network;
pub use telemetry::ingest_telemetry_batch;
pub use topology::ElectricalNetwork;
