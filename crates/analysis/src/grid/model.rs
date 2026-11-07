//! Electrical distribution network domain models for PowerGIS.
//!
//! Models substations, feeders, line segments, transformers, poles, smart meters,
//! customers, telemetry, faults, crews, risk assessments, and power flow metrics.

use powergis_geom::Coord;
use serde::{Deserialize, Serialize};

/// The asset category in an electrical distribution grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GridAssetType {
    Substation,
    Feeder,
    LineSegment,
    Transformer,
    Pole,
    Switch,
    Meter,
}

impl GridAssetType {
    pub fn as_str(&self) -> &'static str {
        match self {
            GridAssetType::Substation => "SUBSTATION",
            GridAssetType::Feeder => "FEEDER",
            GridAssetType::LineSegment => "LINE_SEGMENT",
            GridAssetType::Transformer => "TRANSFORMER",
            GridAssetType::Pole => "POLE",
            GridAssetType::Switch => "SWITCH",
            GridAssetType::Meter => "METER",
        }
    }

    pub fn from_str_lossy(s: &str) -> Self {
        match s.to_ascii_uppercase().as_str() {
            "SUBSTATION" => GridAssetType::Substation,
            "FEEDER" => GridAssetType::Feeder,
            "LINE_SEGMENT" | "LINE" => GridAssetType::LineSegment,
            "TRANSFORMER" => GridAssetType::Transformer,
            "POLE" => GridAssetType::Pole,
            "SWITCH" => GridAssetType::Switch,
            _ => GridAssetType::Meter,
        }
    }
}

/// Operational status of a grid node.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GridAssetStatus {
    #[default]
    Active,
    Inactive,
    Faulted,
    Maintenance,
}

impl GridAssetStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            GridAssetStatus::Active => "ACTIVE",
            GridAssetStatus::Inactive => "INACTIVE",
            GridAssetStatus::Faulted => "FAULTED",
            GridAssetStatus::Maintenance => "MAINTENANCE",
        }
    }
}

/// The type of electrical conductor / connection edge.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GridEdgeType {
    PowerLine,
    LvSegment,
    ServiceDrop,
}

impl GridEdgeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            GridEdgeType::PowerLine => "POWER_LINE",
            GridEdgeType::LvSegment => "LV_SEGMENT",
            GridEdgeType::ServiceDrop => "SERVICE_DROP",
        }
    }
}

/// Category of power consumer with characteristic consumption and tariff profile.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CustomerCategory {
    #[default]
    Residential,
    Commercial,
    Industrial,
    Agricultural,
    Government,
    Other,
}

impl CustomerCategory {
    pub fn code(&self) -> &'static str {
        match self {
            CustomerCategory::Residential => "RESIDENTIAL",
            CustomerCategory::Commercial => "COMMERCIAL",
            CustomerCategory::Industrial => "INDUSTRIAL",
            CustomerCategory::Agricultural => "AGRICULTURAL",
            CustomerCategory::Government => "GOVERNMENT",
            CustomerCategory::Other => "OTHER",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            CustomerCategory::Residential => "Residential Household",
            CustomerCategory::Commercial => "Commercial Business",
            CustomerCategory::Industrial => "Industrial & Factory",
            CustomerCategory::Agricultural => "Agricultural & Farming",
            CustomerCategory::Government => "Government & Public Services",
            CustomerCategory::Other => "Other / Special Tariff",
        }
    }

    pub fn default_daily_kwh(&self) -> f64 {
        match self {
            CustomerCategory::Residential => 15.0,
            CustomerCategory::Commercial => 85.0,
            CustomerCategory::Industrial => 650.0,
            CustomerCategory::Agricultural => 120.0,
            CustomerCategory::Government => 180.0,
            CustomerCategory::Other => 25.0,
        }
    }

    pub fn default_tariff_rate(&self) -> f64 {
        match self {
            CustomerCategory::Residential => 0.14,
            CustomerCategory::Commercial => 0.22,
            CustomerCategory::Industrial => 0.28,
            CustomerCategory::Agricultural => 0.15,
            CustomerCategory::Government => 0.18,
            CustomerCategory::Other => 0.15,
        }
    }
}

/// The nature of electrical fault condition.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FaultType {
    #[default]
    TransformerFailure,
    LineBreak,
    ThreePhase,
    SinglePhaseGround,
    VegetationInterference,
    EquipmentFailure,
}

impl FaultType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FaultType::TransformerFailure => "TRANSFORMER_FAILURE",
            FaultType::LineBreak => "LINE_BREAK",
            FaultType::ThreePhase => "THREE_PHASE",
            FaultType::SinglePhaseGround => "SINGLE_PHASE_GROUND",
            FaultType::VegetationInterference => "VEGETATION_INTERFERENCE",
            FaultType::EquipmentFailure => "EQUIPMENT_FAILURE",
        }
    }

    pub fn from_str_lossy(s: &str) -> Self {
        match s.to_ascii_uppercase().as_str() {
            "TRANSFORMER_FAILURE" => FaultType::TransformerFailure,
            "LINE_BREAK" => FaultType::LineBreak,
            "THREE_PHASE" => FaultType::ThreePhase,
            "SINGLE_PHASE_GROUND" => FaultType::SinglePhaseGround,
            "VEGETATION_INTERFERENCE" => FaultType::VegetationInterference,
            _ => FaultType::EquipmentFailure,
        }
    }

    pub fn base_restoration_minutes(&self) -> f64 {
        match self {
            FaultType::TransformerFailure => 180.0,
            FaultType::LineBreak => 120.0,
            FaultType::ThreePhase => 150.0,
            FaultType::SinglePhaseGround => 60.0,
            FaultType::VegetationInterference => 45.0,
            FaultType::EquipmentFailure => 90.0,
        }
    }
}

/// Priority tier assigned dynamically to a fault or risk.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FaultPriority {
    #[default]
    Low,
    Medium,
    High,
    Critical,
}

impl FaultPriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            FaultPriority::Low => "LOW",
            FaultPriority::Medium => "MEDIUM",
            FaultPriority::High => "HIGH",
            FaultPriority::Critical => "CRITICAL",
        }
    }
}

/// Operational status of a response crew.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CrewStatus {
    #[default]
    Available,
    EnRoute,
    OnSite,
    OffDuty,
}

impl CrewStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            CrewStatus::Available => "AVAILABLE",
            CrewStatus::EnRoute => "EN_ROUTE",
            CrewStatus::OnSite => "ON_SITE",
            CrewStatus::OffDuty => "OFF_DUTY",
        }
    }
}

/// Operational status of a smart meter.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MeterStatus {
    #[default]
    Active,
    Outage,
    VoltageAnomaly,
    Offline,
}

impl MeterStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            MeterStatus::Active => "ACTIVE",
            MeterStatus::Outage => "OUTAGE",
            MeterStatus::VoltageAnomaly => "VOLTAGE_ANOMALY",
            MeterStatus::Offline => "OFFLINE",
        }
    }
}

/// Role-based access control tier for customer privacy masking.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserRole {
    Admin,
    #[default]
    Dispatcher,
    FieldEngineer,
    Guest,
}

impl UserRole {
    pub fn should_mask_names(&self) -> bool {
        matches!(self, UserRole::FieldEngineer | UserRole::Guest)
    }
}

/// A node in the electrical distribution grid (substation, feeder, line, transformer, pole, switch).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GridNode {
    pub asset_id: String,
    pub name: String,
    pub asset_type: GridAssetType,
    pub voltage_kv: f64,
    pub status: GridAssetStatus,
    pub parent_asset_id: Option<String>,
    pub capacity_kva: f64,
    pub install_year: u32,
    pub coord: Coord,
}

/// A physical line conductor or service drop connecting two grid nodes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GridEdge {
    pub edge_id: String,
    pub from_node: String,
    pub to_node: String,
    pub edge_type: GridEdgeType,
    pub status: String,
    pub length_m: f64,
    pub resistance_ohm: f64,
    pub reactance_ohm: f64,
    pub max_current_a: f64,
    pub path: Vec<Coord>,
}

/// Customer account record associated with one or more meters.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomerProfile {
    pub customer_number: String,
    pub name: String,
    pub masked_name: String,
    pub category: CustomerCategory,
    pub avg_consumption_kwh_day: f64,
    pub default_tariff_rate: f64,
    pub is_critical: bool,
    pub critical_reason: Option<String>,
}

/// Smart meter asset providing telemetry and status tracking.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartMeter {
    pub meter_number: String,
    pub customer_number: String,
    pub status: MeterStatus,
    pub transformer_id: String,
    pub feeder_id: String,
    pub node_id: String,
    pub coord: Coord,
}

/// Real-time or simulated electrical telemetry reading.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MeterTelemetry {
    pub meter_number: String,
    pub voltage_v: f64,
    pub current_a: f64,
    pub active_power_kw: f64,
    pub energy_kwh: f64,
    pub power_factor: f64,
    pub timestamp_ms: u64,
}

/// A registered active or historical fault on a grid asset.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FaultRecord {
    pub fault_number: String,
    pub asset_id: String,
    pub asset_type: GridAssetType,
    pub asset_name: String,
    pub fault_type: FaultType,
    pub status: String,
    pub priority: FaultPriority,
    pub priority_score: f64,
    pub priority_explanation: String,
    pub total_affected_meters: usize,
    pub residential_affected: usize,
    pub commercial_affected: usize,
    pub industrial_affected: usize,
    pub agricultural_affected: usize,
    pub government_affected: usize,
    pub other_affected: usize,
    pub critical_customers_affected: usize,
    pub estimated_revenue_loss_per_day: f64,
    pub coord: Coord,
    pub created_at_ms: u64,
    pub restored_at_ms: Option<u64>,
}

/// Field maintenance and restoration response crew.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Crew {
    pub crew_code: String,
    pub crew_name: String,
    pub lead_name: String,
    pub status: CrewStatus,
    pub contact_phone: String,
    pub skills: Vec<String>,
    pub current_fault_number: Option<String>,
    pub coord: Coord,
}

/// Record of crew dispatch to a fault.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CrewAssignment {
    pub crew_code: String,
    pub fault_number: String,
    pub distance_km: f64,
    pub estimated_travel_minutes: f64,
    pub estimated_restoration_minutes: f64,
    pub status: String,
    pub assigned_at_ms: u64,
}

/// Predictive risk factors and maintenance assessment.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub asset_id: String,
    pub asset_name: String,
    pub asset_type: GridAssetType,
    pub voltage_kv: f64,
    pub risk_score: f64,
    pub risk_level: FaultPriority,
    pub failure_probability_pct: f64,
    pub historical_failure_count: usize,
    pub asset_age_years: u32,
    pub connected_meters: usize,
    pub critical_customers: usize,
    pub daily_revenue_exposure: f64,
    pub weather_risk_factor: f64,
    pub vegetation_risk_factor: f64,
    pub recommendations: Vec<String>,
}

/// Geographic risk concentration hotspot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RiskHotspot {
    pub id: String,
    pub name: String,
    pub coord: Coord,
    pub metric: String,
    pub intensity_score: f64,
    pub affected_meters: usize,
    pub revenue_exposure_daily: f64,
}

/// Power flow calculation summary along a feeder or transformer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransformerLoading {
    pub transformer_id: String,
    pub name: String,
    pub rated_capacity_kva: f64,
    pub connected_load_kva: f64,
    pub utilization_pct: f64,
    pub is_overloaded: bool,
}

/// Voltage drop violation point along a feeder.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VoltageViolation {
    pub node_id: String,
    pub name: String,
    pub nominal_kv: f64,
    pub actual_kv: f64,
    pub per_unit_voltage: f64,
    pub drop_pct: f64,
    pub violation_tier: String,
}

/// Comprehensive power flow simulation output.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerFlowSummary {
    pub substation_id: String,
    pub total_generation_kwh: f64,
    pub total_load_kw: f64,
    pub total_losses_kw: f64,
    pub overall_efficiency_pct: f64,
    pub transformer_loading: Vec<TransformerLoading>,
    pub voltage_violations: Vec<VoltageViolation>,
}

/// SCADA telemetry anomaly alert.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TelemetryAnomaly {
    pub meter_number: String,
    pub customer_number: String,
    pub anomaly_type: String,
    pub observed_value: f64,
    pub threshold: f64,
    pub unit: String,
    pub message: String,
    pub timestamp_ms: u64,
}
