//! Reference electrical distribution network sample dataset for PowerGIS.
//!
//! Models the Juru and Goromonzi 132kV/33kV/11kV/0.4kV distribution network with
//! substations, feeders, transformers, poles, smart meters, customers, and crews.

use powergis_geom::Coord;

use super::model::{
    Crew, CrewAssignment, CrewStatus, CustomerCategory, CustomerProfile, FaultPriority, FaultRecord,
    FaultType, GridAssetStatus, GridAssetType, GridEdge, GridEdgeType, GridNode, MeterStatus, MeterTelemetry,
    SmartMeter,
};
use super::topology::ElectricalNetwork;

/// Generates the complete reference electrical distribution network dataset.
pub fn create_sample_network() -> ElectricalNetwork {
    let mut net = ElectricalNetwork::new();

    let base_lng = 31.15f64;
    let base_lat = -17.75f64;

    // 1. Customers
    net.add_customer(CustomerProfile {
        customer_number: "CUST-001".to_string(),
        name: "Juru General Hospital".to_string(),
        masked_name: "J*** G****** H*******".to_string(),
        category: CustomerCategory::Government,
        avg_consumption_kwh_day: 350.0,
        default_tariff_rate: 0.18,
        is_critical: true,
        critical_reason: Some("Primary regional emergency medical care and surgery facility".to_string()),
    });

    net.add_customer(CustomerProfile {
        customer_number: "CUST-002".to_string(),
        name: "Juru High School & Boarding".to_string(),
        masked_name: "J*** H*** S*****".to_string(),
        category: CustomerCategory::Government,
        avg_consumption_kwh_day: 120.0,
        default_tariff_rate: 0.18,
        is_critical: false,
        critical_reason: None,
    });

    net.add_customer(CustomerProfile {
        customer_number: "CUST-003".to_string(),
        name: "Juru Grain Mills & Processing".to_string(),
        masked_name: "J*** G**** M****".to_string(),
        category: CustomerCategory::Industrial,
        avg_consumption_kwh_day: 850.0,
        default_tariff_rate: 0.28,
        is_critical: false,
        critical_reason: None,
    });

    net.add_customer(CustomerProfile {
        customer_number: "CUST-004".to_string(),
        name: "TM Supermarket & Shopping Complex".to_string(),
        masked_name: "T* S**********".to_string(),
        category: CustomerCategory::Commercial,
        avg_consumption_kwh_day: 220.0,
        default_tariff_rate: 0.22,
        is_critical: false,
        critical_reason: None,
    });

    net.add_customer(CustomerProfile {
        customer_number: "CUST-005".to_string(),
        name: "Goromonzi Irrigation Cooperative".to_string(),
        masked_name: "G******** I*********".to_string(),
        category: CustomerCategory::Agricultural,
        avg_consumption_kwh_day: 320.0,
        default_tariff_rate: 0.15,
        is_critical: false,
        critical_reason: None,
    });

    net.add_customer(CustomerProfile {
        customer_number: "CUST-006".to_string(),
        name: "Juru Residential Block A (Dr. T. Moyo)".to_string(),
        masked_name: "D*. T. M***".to_string(),
        category: CustomerCategory::Residential,
        avg_consumption_kwh_day: 22.0,
        default_tariff_rate: 0.14,
        is_critical: false,
        critical_reason: None,
    });

    net.add_customer(CustomerProfile {
        customer_number: "CUST-007".to_string(),
        name: "Juru Residential Block B (A. Chidziwa)".to_string(),
        masked_name: "A. C*******".to_string(),
        category: CustomerCategory::Residential,
        avg_consumption_kwh_day: 16.0,
        default_tariff_rate: 0.14,
        is_critical: false,
        critical_reason: None,
    });

    net.add_customer(CustomerProfile {
        customer_number: "CUST-008".to_string(),
        name: "Civic Centre, Police & Emergency Dispatch".to_string(),
        masked_name: "C**** C*****, P*****".to_string(),
        category: CustomerCategory::Government,
        avg_consumption_kwh_day: 190.0,
        default_tariff_rate: 0.18,
        is_critical: true,
        critical_reason: Some("Emergency dispatch and public safety communications node".to_string()),
    });

    // 2. Substations
    net.add_node(GridNode {
        asset_id: "SUB-01".to_string(),
        name: "Juru Main Substation 132/33kV".to_string(),
        asset_type: GridAssetType::Substation,
        voltage_kv: 132.0,
        status: GridAssetStatus::Active,
        parent_asset_id: None,
        capacity_kva: 25_000.0,
        install_year: 2008,
        coord: Coord::new(base_lng, base_lat),
    });

    net.add_node(GridNode {
        asset_id: "SUB-02".to_string(),
        name: "Goromonzi Central Substation 132/33kV".to_string(),
        asset_type: GridAssetType::Substation,
        voltage_kv: 132.0,
        status: GridAssetStatus::Active,
        parent_asset_id: None,
        capacity_kva: 20_000.0,
        install_year: 2011,
        coord: Coord::new(base_lng + 0.08, base_lat + 0.08),
    });

    // 3. Feeders
    net.add_node(GridNode {
        asset_id: "JURU-01".to_string(),
        name: "Juru Feeder 01 (North Industrial)".to_string(),
        asset_type: GridAssetType::Feeder,
        voltage_kv: 33.0,
        status: GridAssetStatus::Active,
        parent_asset_id: Some("SUB-01".to_string()),
        capacity_kva: 7_500.0,
        install_year: 2012,
        coord: Coord::new(base_lng - 0.02, base_lat + 0.02),
    });

    net.add_node(GridNode {
        asset_id: "JURU-02".to_string(),
        name: "Juru Feeder 02 (Township North)".to_string(),
        asset_type: GridAssetType::Feeder,
        voltage_kv: 33.0,
        status: GridAssetStatus::Active,
        parent_asset_id: Some("SUB-01".to_string()),
        capacity_kva: 5_000.0,
        install_year: 2014,
        coord: Coord::new(base_lng + 0.01, base_lat - 0.01),
    });

    net.add_node(GridNode {
        asset_id: "JURU-03".to_string(),
        name: "Juru Feeder 03 (East Urban & Commercial)".to_string(),
        asset_type: GridAssetType::Feeder,
        voltage_kv: 33.0,
        status: GridAssetStatus::Active,
        parent_asset_id: Some("SUB-01".to_string()),
        capacity_kva: 7_500.0,
        install_year: 2010,
        coord: Coord::new(base_lng + 0.03, base_lat + 0.01),
    });

    net.add_node(GridNode {
        asset_id: "GORO-01".to_string(),
        name: "Goromonzi Feeder 01 (Rural South)".to_string(),
        asset_type: GridAssetType::Feeder,
        voltage_kv: 33.0,
        status: GridAssetStatus::Active,
        parent_asset_id: Some("SUB-02".to_string()),
        capacity_kva: 5_000.0,
        install_year: 2016,
        coord: Coord::new(base_lng + 0.09, base_lat + 0.07),
    });

    // 4. Line Segments
    net.add_node(GridNode {
        asset_id: "LS-JURU03-01".to_string(),
        name: "Feeder Trunk Segment J3-1".to_string(),
        asset_type: GridAssetType::LineSegment,
        voltage_kv: 33.0,
        status: GridAssetStatus::Active,
        parent_asset_id: Some("JURU-03".to_string()),
        capacity_kva: 7_500.0,
        install_year: 2010,
        coord: Coord::new(base_lng + 0.032, base_lat + 0.012),
    });

    net.add_node(GridNode {
        asset_id: "LS-JURU03-02".to_string(),
        name: "Feeder Branch Segment J3-2".to_string(),
        asset_type: GridAssetType::LineSegment,
        voltage_kv: 11.0,
        status: GridAssetStatus::Active,
        parent_asset_id: Some("LS-JURU03-01".to_string()),
        capacity_kva: 3_000.0,
        install_year: 2012,
        coord: Coord::new(base_lng + 0.035, base_lat + 0.015),
    });

    net.add_node(GridNode {
        asset_id: "LS-JURU01-01".to_string(),
        name: "Industrial Line Segment I1-1".to_string(),
        asset_type: GridAssetType::LineSegment,
        voltage_kv: 33.0,
        status: GridAssetStatus::Active,
        parent_asset_id: Some("JURU-01".to_string()),
        capacity_kva: 5_000.0,
        install_year: 2015,
        coord: Coord::new(base_lng - 0.022, base_lat + 0.022),
    });

    // 5. Transformers
    net.add_node(GridNode {
        asset_id: "T-101".to_string(),
        name: "Transformer T-101 11/0.4kV (Juru Industrial)".to_string(),
        asset_type: GridAssetType::Transformer,
        voltage_kv: 11.0,
        status: GridAssetStatus::Active,
        parent_asset_id: Some("JURU-01".to_string()),
        capacity_kva: 500.0,
        install_year: 2015,
        coord: Coord::new(base_lng - 0.025, base_lat + 0.025),
    });

    net.add_node(GridNode {
        asset_id: "T-102".to_string(),
        name: "Transformer T-102 11/0.4kV (Township West)".to_string(),
        asset_type: GridAssetType::Transformer,
        voltage_kv: 11.0,
        status: GridAssetStatus::Active,
        parent_asset_id: Some("JURU-02".to_string()),
        capacity_kva: 315.0,
        install_year: 2018,
        coord: Coord::new(base_lng + 0.012, base_lat - 0.012),
    });

    net.add_node(GridNode {
        asset_id: "T-103".to_string(),
        name: "Transformer T-103 11/0.4kV (Commercial Center)".to_string(),
        asset_type: GridAssetType::Transformer,
        voltage_kv: 11.0,
        status: GridAssetStatus::Active,
        parent_asset_id: Some("LS-JURU03-01".to_string()),
        capacity_kva: 500.0,
        install_year: 2012,
        coord: Coord::new(base_lng + 0.033, base_lat + 0.013),
    });

    net.add_node(GridNode {
        asset_id: "T-104".to_string(),
        name: "Transformer T-104 11/0.4kV (Juru Hospital & Residential)".to_string(),
        asset_type: GridAssetType::Transformer,
        voltage_kv: 11.0,
        status: GridAssetStatus::Active,
        parent_asset_id: Some("LS-JURU03-02".to_string()),
        capacity_kva: 630.0,
        install_year: 2010,
        coord: Coord::new(base_lng + 0.036, base_lat + 0.016),
    });

    net.add_node(GridNode {
        asset_id: "T-105".to_string(),
        name: "Transformer T-105 11/0.4kV (East Residential)".to_string(),
        asset_type: GridAssetType::Transformer,
        voltage_kv: 11.0,
        status: GridAssetStatus::Active,
        parent_asset_id: Some("LS-JURU03-02".to_string()),
        capacity_kva: 315.0,
        install_year: 2020,
        coord: Coord::new(base_lng + 0.040, base_lat + 0.020),
    });

    net.add_node(GridNode {
        asset_id: "T-301".to_string(),
        name: "Transformer T-301 33/0.4kV (Goromonzi Farm)".to_string(),
        asset_type: GridAssetType::Transformer,
        voltage_kv: 33.0,
        status: GridAssetStatus::Active,
        parent_asset_id: Some("GORO-01".to_string()),
        capacity_kva: 250.0,
        install_year: 2019,
        coord: Coord::new(base_lng + 0.095, base_lat + 0.075),
    });

    // 6. Distribution Poles
    for i in 1..=12 {
        let angle = i as f64 * 0.523598775f64; // ~30 degrees in radians
        let p_lng = base_lng + 0.036 + angle.sin() * 0.0025;
        let p_lat = base_lat + 0.016 + angle.cos() * 0.0025;
        net.add_node(GridNode {
            asset_id: format!("P-{}", 200 + i),
            name: format!("Pole P-{}", 200 + i),
            asset_type: GridAssetType::Pole,
            voltage_kv: 0.4,
            status: GridAssetStatus::Active,
            parent_asset_id: Some("T-104".to_string()),
            capacity_kva: 50.0,
            install_year: 2010,
            coord: Coord::new(p_lng, p_lat),
        });
    }

    // 7. Network Edges (Conductive Line Segments)
    net.add_edge(GridEdge {
        edge_id: "E-SUB01-J3".to_string(),
        from_node: "SUB-01".to_string(),
        to_node: "JURU-03".to_string(),
        edge_type: GridEdgeType::PowerLine,
        status: "CLOSED".to_string(),
        length_m: 3500.0,
        resistance_ohm: 1.25,
        reactance_ohm: 1.10,
        max_current_a: 400.0,
        path: vec![Coord::new(base_lng, base_lat), Coord::new(base_lng + 0.03, base_lat + 0.01)],
    });

    net.add_edge(GridEdge {
        edge_id: "E-J3-LS1".to_string(),
        from_node: "JURU-03".to_string(),
        to_node: "LS-JURU03-01".to_string(),
        edge_type: GridEdgeType::PowerLine,
        status: "CLOSED".to_string(),
        length_m: 800.0,
        resistance_ohm: 0.28,
        reactance_ohm: 0.25,
        max_current_a: 350.0,
        path: vec![Coord::new(base_lng + 0.03, base_lat + 0.01), Coord::new(base_lng + 0.032, base_lat + 0.012)],
    });

    net.add_edge(GridEdge {
        edge_id: "E-LS1-LS2".to_string(),
        from_node: "LS-JURU03-01".to_string(),
        to_node: "LS-JURU03-02".to_string(),
        edge_type: GridEdgeType::PowerLine,
        status: "CLOSED".to_string(),
        length_m: 600.0,
        resistance_ohm: 0.21,
        reactance_ohm: 0.18,
        max_current_a: 300.0,
        path: vec![Coord::new(base_lng + 0.032, base_lat + 0.012), Coord::new(base_lng + 0.035, base_lat + 0.015)],
    });

    net.add_edge(GridEdge {
        edge_id: "E-LS2-T104".to_string(),
        from_node: "LS-JURU03-02".to_string(),
        to_node: "T-104".to_string(),
        edge_type: GridEdgeType::PowerLine,
        status: "CLOSED".to_string(),
        length_m: 350.0,
        resistance_ohm: 0.12,
        reactance_ohm: 0.10,
        max_current_a: 250.0,
        path: vec![Coord::new(base_lng + 0.035, base_lat + 0.015), Coord::new(base_lng + 0.036, base_lat + 0.016)],
    });

    net.add_edge(GridEdge {
        edge_id: "E-LS2-T105".to_string(),
        from_node: "LS-JURU03-02".to_string(),
        to_node: "T-105".to_string(),
        edge_type: GridEdgeType::PowerLine,
        status: "CLOSED".to_string(),
        length_m: 650.0,
        resistance_ohm: 0.22,
        reactance_ohm: 0.19,
        max_current_a: 200.0,
        path: vec![Coord::new(base_lng + 0.035, base_lat + 0.015), Coord::new(base_lng + 0.040, base_lat + 0.020)],
    });

    net.add_edge(GridEdge {
        edge_id: "E-SUB01-J1".to_string(),
        from_node: "SUB-01".to_string(),
        to_node: "JURU-01".to_string(),
        edge_type: GridEdgeType::PowerLine,
        status: "CLOSED".to_string(),
        length_m: 2800.0,
        resistance_ohm: 0.95,
        reactance_ohm: 0.85,
        max_current_a: 400.0,
        path: vec![Coord::new(base_lng, base_lat), Coord::new(base_lng - 0.02, base_lat + 0.02)],
    });

    net.add_edge(GridEdge {
        edge_id: "E-J1-T101".to_string(),
        from_node: "JURU-01".to_string(),
        to_node: "T-101".to_string(),
        edge_type: GridEdgeType::PowerLine,
        status: "CLOSED".to_string(),
        length_m: 900.0,
        resistance_ohm: 0.32,
        reactance_ohm: 0.28,
        max_current_a: 300.0,
        path: vec![Coord::new(base_lng - 0.02, base_lat + 0.02), Coord::new(base_lng - 0.025, base_lat + 0.025)],
    });

    // 8. Smart Meters
    // Critical hospital meters under T-104
    net.add_meter(SmartMeter {
        meter_number: "MTR-HOSP-01".to_string(),
        customer_number: "CUST-001".to_string(),
        status: MeterStatus::Active,
        transformer_id: "T-104".to_string(),
        feeder_id: "JURU-03".to_string(),
        node_id: "P-201".to_string(),
        coord: Coord::new(base_lng + 0.0361, base_lat + 0.0161),
    });

    net.add_meter(SmartMeter {
        meter_number: "MTR-HOSP-02".to_string(),
        customer_number: "CUST-001".to_string(),
        status: MeterStatus::Active,
        transformer_id: "T-104".to_string(),
        feeder_id: "JURU-03".to_string(),
        node_id: "P-202".to_string(),
        coord: Coord::new(base_lng + 0.0363, base_lat + 0.0162),
    });

    // Emergency dispatch & civic center meters under T-104
    net.add_meter(SmartMeter {
        meter_number: "MTR-POLICE-01".to_string(),
        customer_number: "CUST-008".to_string(),
        status: MeterStatus::Active,
        transformer_id: "T-104".to_string(),
        feeder_id: "JURU-03".to_string(),
        node_id: "P-203".to_string(),
        coord: Coord::new(base_lng + 0.0365, base_lat + 0.0159),
    });

    // Residential meters under T-104 (P-204 .. P-212)
    for i in 4..=12 {
        let cust_num = if i % 2 == 0 { "CUST-006" } else { "CUST-007" };
        let pole_id = format!("P-{}", 200 + i);
        let m_coord = net.nodes.get(&pole_id).map(|p| p.coord).unwrap_or(Coord::new(base_lng + 0.036, base_lat + 0.016));
        net.add_meter(SmartMeter {
            meter_number: format!("MTR-RES-{:03}", i),
            customer_number: cust_num.to_string(),
            status: MeterStatus::Active,
            transformer_id: "T-104".to_string(),
            feeder_id: "JURU-03".to_string(),
            node_id: pole_id,
            coord: Coord::new(m_coord.x + 0.0002, m_coord.y + 0.0002),
        });
    }

    // Industrial meters under T-101
    net.add_meter(SmartMeter {
        meter_number: "MTR-IND-01".to_string(),
        customer_number: "CUST-003".to_string(),
        status: MeterStatus::Active,
        transformer_id: "T-101".to_string(),
        feeder_id: "JURU-01".to_string(),
        node_id: "T-101".to_string(),
        coord: Coord::new(base_lng - 0.0252, base_lat + 0.0252),
    });

    // Commercial meters under T-103
    net.add_meter(SmartMeter {
        meter_number: "MTR-COMM-01".to_string(),
        customer_number: "CUST-004".to_string(),
        status: MeterStatus::Active,
        transformer_id: "T-103".to_string(),
        feeder_id: "JURU-03".to_string(),
        node_id: "T-103".to_string(),
        coord: Coord::new(base_lng + 0.0332, base_lat + 0.0132),
    });

    // Agricultural meters under T-301
    net.add_meter(SmartMeter {
        meter_number: "MTR-AGRI-01".to_string(),
        customer_number: "CUST-005".to_string(),
        status: MeterStatus::Active,
        transformer_id: "T-301".to_string(),
        feeder_id: "GORO-01".to_string(),
        node_id: "T-301".to_string(),
        coord: Coord::new(base_lng + 0.0952, base_lat + 0.0752),
    });

    // High School meter under T-102
    net.add_meter(SmartMeter {
        meter_number: "MTR-SCH-01".to_string(),
        customer_number: "CUST-002".to_string(),
        status: MeterStatus::Active,
        transformer_id: "T-102".to_string(),
        feeder_id: "JURU-02".to_string(),
        node_id: "T-102".to_string(),
        coord: Coord::new(base_lng + 0.0122, base_lat - 0.0122),
    });

    // 9. Active Faults
    net.add_fault(FaultRecord {
        fault_number: "FLT-0001".to_string(),
        asset_id: "T-104".to_string(),
        asset_type: GridAssetType::Transformer,
        asset_name: "Main Transformer T-104 (Juru Center)".to_string(),
        fault_type: FaultType::TransformerFailure,
        status: "ACTIVE".to_string(),
        priority: FaultPriority::Critical,
        priority_score: 96.5,
        priority_explanation: "Major distribution transformer failure at high-density town center; 10 residential meters and critical clinic offline.".to_string(),
        total_affected_meters: 10,
        residential_affected: 10,
        commercial_affected: 0,
        industrial_affected: 0,
        agricultural_affected: 0,
        government_affected: 0,
        other_affected: 0,
        critical_customers_affected: 1,
        estimated_revenue_loss_per_day: 30.80,
        coord: Coord::new(base_lng + 0.025, base_lat + 0.005),
        created_at_ms: 1762500000000,
        restored_at_ms: None,
    });

    net.add_fault(FaultRecord {
        fault_number: "FLT-0002".to_string(),
        asset_id: "JURU-01".to_string(),
        asset_type: GridAssetType::Feeder,
        asset_name: "33kV Feeder JURU-01 (Industrial Feeder)".to_string(),
        fault_type: FaultType::VegetationInterference,
        status: "ACTIVE".to_string(),
        priority: FaultPriority::High,
        priority_score: 88.0,
        priority_explanation: "Overhead 33kV line trip from vegetation contact; Juru Grain Mills industrial processing plant affected ($238/day loss).".to_string(),
        total_affected_meters: 1,
        residential_affected: 0,
        commercial_affected: 0,
        industrial_affected: 1,
        agricultural_affected: 0,
        government_affected: 0,
        other_affected: 0,
        critical_customers_affected: 0,
        estimated_revenue_loss_per_day: 238.00,
        coord: Coord::new(base_lng - 0.025, base_lat + 0.025),
        created_at_ms: 1762503600000,
        restored_at_ms: None,
    });

    // Update faulted assets in nodes
    if let Some(t104) = net.nodes.get_mut("T-104") {
        t104.status = GridAssetStatus::Faulted;
    }
    if let Some(juru01) = net.nodes.get_mut("JURU-01") {
        juru01.status = GridAssetStatus::Faulted;
    }

    // Set meters affected by FLT-0001 (under T-104) to Outage
    for i in 1..=10 {
        let meter_id = format!("MTR-RES-{:02}", i);
        if let Some(m) = net.meters.get_mut(&meter_id) {
            m.status = MeterStatus::Outage;
        }
    }
    // Set meter affected by FLT-0002 to Outage
    if let Some(m) = net.meters.get_mut("MTR-IND-01") {
        m.status = MeterStatus::Outage;
    }

    // 10. Response Crews (with real dispatch status)
    net.add_crew(Crew {
        crew_code: "CREW-ALPHA".to_string(),
        crew_name: "Rapid Response Alpha".to_string(),
        lead_name: "Eng. Farai Ndlovu".to_string(),
        status: CrewStatus::OnSite,
        contact_phone: "+263 77 123 4567".to_string(),
        skills: vec!["HIGH_VOLTAGE".to_string(), "TRANSFORMER_SPECIALIST".to_string(), "PROTECTION_RELAYS".to_string()],
        current_fault_number: Some("FLT-0001".to_string()),
        coord: Coord::new(base_lng + 0.0252, base_lat + 0.0051),
    });

    net.add_crew(Crew {
        crew_code: "CREW-BRAVO".to_string(),
        crew_name: "Feeder & Overhead Line Bravo".to_string(),
        lead_name: "Technician T. Sibanda".to_string(),
        status: CrewStatus::EnRoute,
        contact_phone: "+263 77 234 5678".to_string(),
        skills: vec!["LINE_SPLICING".to_string(), "TREE_CLEARANCE".to_string(), "POLE_RIGGING".to_string()],
        current_fault_number: Some("FLT-0002".to_string()),
        coord: Coord::new(base_lng - 0.010, base_lat + 0.015),
    });

    net.add_crew(Crew {
        crew_code: "CREW-CHARLIE".to_string(),
        crew_name: "Metering & LV Support Charlie".to_string(),
        lead_name: "Technician R. Mutasa".to_string(),
        status: CrewStatus::Available,
        contact_phone: "+263 77 345 6789".to_string(),
        skills: vec!["SMART_METERS".to_string(), "LV_DROP_REPAIR".to_string(), "CONSUMER_INSPECTION".to_string()],
        current_fault_number: None,
        coord: Coord::new(base_lng + 0.038, base_lat + 0.018),
    });

    // 11. Dispatch & Crew Assignment Tracking
    net.add_assignment(CrewAssignment {
        crew_code: "CREW-ALPHA".to_string(),
        fault_number: "FLT-0001".to_string(),
        distance_km: 1.2,
        estimated_travel_minutes: 8.0,
        estimated_restoration_minutes: 45.0,
        status: "ON_SITE".to_string(),
        assigned_at_ms: 1762500600000,
    });

    net.add_assignment(CrewAssignment {
        crew_code: "CREW-BRAVO".to_string(),
        fault_number: "FLT-0002".to_string(),
        distance_km: 4.8,
        estimated_travel_minutes: 18.0,
        estimated_restoration_minutes: 90.0,
        status: "EN_ROUTE".to_string(),
        assigned_at_ms: 1762504200000,
    });

    // 12. Smart Meter Telemetry Data
    let meter_statuses: Vec<(String, MeterStatus)> = net.meters.iter().map(|(id, m)| (id.clone(), m.status)).collect();
    for (meter_id, status) in meter_statuses {
        let (v, i, p, pf) = match status {
            MeterStatus::Outage => (0.0, 0.0, 0.0, 0.0),
            _ => (230.8, 14.5, 3.3, 0.98),
        };
        net.add_telemetry(MeterTelemetry {
            meter_number: meter_id,
            voltage_v: v,
            current_a: i,
            active_power_kw: p,
            energy_kwh: 14250.0,
            power_factor: pf,
            timestamp_ms: 1762505000000,
        });
    }

    net.rebuild_indices();
    net
}
