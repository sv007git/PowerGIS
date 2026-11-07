//! Comprehensive test suite for electrical distribution network and GridSense suite.

#[cfg(test)]
mod tests {
    use super::super::crew::{assign_crew, find_best_crew};
    use super::super::impact::{calculate_impact, restore_fault, trigger_fault};
    use super::super::layers::all_grid_datasets;
    use super::super::model::{
        CrewStatus, FaultPriority, FaultType, GridAssetStatus, MeterStatus, MeterTelemetry, UserRole,
    };
    use super::super::power_flow::simulate_power_flow;
    use super::super::risk::{assess_asset_risk, calculate_hotspots};
    use super::super::sample::create_sample_network;
    use super::super::telemetry::ingest_telemetry_batch;

    #[test]
    fn sample_network_initialization() {
        let net = create_sample_network();

        assert_eq!(net.nodes.len(), 24); // 2 substations + 4 feeders + 3 lines + 6 transformers + 12 poles - wait check total
        assert!(net.nodes.contains_key("SUB-01"));
        assert!(net.nodes.contains_key("SUB-02"));
        assert!(net.nodes.contains_key("T-104"));
        assert!(net.nodes.contains_key("P-201"));

        assert_eq!(net.customers.len(), 8);
        assert!(net.meters.len() >= 15);
        assert_eq!(net.crews.len(), 3);
        assert!(net.edges.len() >= 7);
    }

    #[test]
    fn downstream_traversal_and_connected_meters() {
        let net = create_sample_network();

        // Downstream from SUB-01 should reach JURU-03, T-104, and its poles
        let downstream = net.get_downstream_asset_ids("SUB-01", 15);
        assert!(downstream.contains(&"SUB-01".to_string()));
        assert!(downstream.contains(&"JURU-03".to_string()));
        assert!(downstream.contains(&"T-104".to_string()));
        assert!(downstream.contains(&"P-201".to_string()));

        // Connected meters to T-104 (hospital, police, residential)
        let meters = net.get_connected_meters("T-104");
        assert!(meters.len() >= 10);
        assert!(meters.iter().any(|m| m.meter_number == "MTR-HOSP-01"));
        assert!(meters.iter().any(|m| m.meter_number == "MTR-POLICE-01"));

        // Connected meters from feeder JURU-03 should include all meters under T-103, T-104, T-105
        let feeder_meters = net.get_connected_meters("JURU-03");
        assert!(feeder_meters.len() >= meters.len());
    }

    #[test]
    fn upstream_path_tracing() {
        let net = create_sample_network();

        // From pole P-201 upstream should reach T-104
        let path = net.get_upstream_path("P-201", 10);
        assert_eq!(path.first().map(|s| s.as_str()), Some("P-201"));
        assert!(path.contains(&"T-104".to_string()));
    }

    #[test]
    fn fault_impact_and_priority_scoring() {
        let net = create_sample_network();

        // Impact on T-104 (powers Juru Hospital & Police station)
        let impact = calculate_impact(&net, "T-104", FaultType::TransformerFailure).unwrap();

        assert_eq!(impact.asset_id, "T-104");
        assert!(impact.affected_meters_count >= 10);
        assert_eq!(impact.critical_customers_affected, 2); // Hospital + Police
        assert!(impact.critical_reasons.iter().any(|r| r.contains("medical care")));
        assert!(impact.estimated_revenue_loss_per_day > 50.0);

        // Because critical facilities are affected, priority must be CRITICAL
        assert_eq!(impact.priority, FaultPriority::Critical);
        assert!(impact.priority_score >= 80.0);
        assert!(impact.priority_explanation.contains("Priority CRITICAL"));
    }

    #[test]
    fn fault_trigger_and_restoration_lifecycle() {
        let mut net = create_sample_network();

        // Before fault: T-104 is ACTIVE, meters are ACTIVE
        assert_eq!(net.nodes["T-104"].status, GridAssetStatus::Active);
        assert_eq!(net.meters["MTR-HOSP-01"].status, MeterStatus::Active);

        // 1. Trigger fault
        let fault = trigger_fault(&mut net, "T-104", FaultType::TransformerFailure, 1700000000000).unwrap();
        assert_eq!(fault.asset_id, "T-104");
        assert_eq!(fault.status, "ACTIVE");

        // After fault: T-104 is FAULTED, meters are OUTAGE
        assert_eq!(net.nodes["T-104"].status, GridAssetStatus::Faulted);
        assert_eq!(net.meters["MTR-HOSP-01"].status, MeterStatus::Outage);
        assert_eq!(net.meters["MTR-POLICE-01"].status, MeterStatus::Outage);

        // 2. Assign crew to fault
        let assignment = assign_crew(&mut net, None, &fault.fault_number, 1700000100000).unwrap();
        let assigned_crew = &net.crews[&assignment.crew_code];
        assert_eq!(assigned_crew.status, CrewStatus::EnRoute);
        assert_eq!(assigned_crew.current_fault_number.as_deref(), Some(fault.fault_number.as_str()));

        // 3. Restore fault
        let restored = restore_fault(&mut net, &fault.fault_number, 1700000500000).unwrap();
        assert_eq!(restored.status, "RESTORED");
        assert!(restored.restored_at_ms.is_some());

        // After restore: T-104 is ACTIVE, meters are ACTIVE, crew is AVAILABLE
        assert_eq!(net.nodes["T-104"].status, GridAssetStatus::Active);
        assert_eq!(net.meters["MTR-HOSP-01"].status, MeterStatus::Active);
        assert_eq!(net.crews[&assignment.crew_code].status, CrewStatus::Available);
        assert!(net.crews[&assignment.crew_code].current_fault_number.is_none());
    }

    #[test]
    fn predictive_risk_assessment_and_hotspots() {
        let net = create_sample_network();

        // Assess T-104 risk
        let risk = assess_asset_risk(&net, "T-104", 2026).unwrap();
        assert_eq!(risk.asset_id, "T-104");
        assert!(risk.failure_probability_pct > 10.0);
        assert!(risk.risk_score > 30.0);
        assert!(!risk.recommendations.is_empty());

        // Calculate hotspots
        let hotspots = calculate_hotspots(&net, "customer_impact", 2026);
        assert!(!hotspots.is_empty());
        assert_eq!(hotspots[0].metric, "customer_impact");
        assert!(hotspots[0].intensity_score > 0.0);
    }

    #[test]
    fn power_flow_simulation() {
        let net = create_sample_network();

        let flow = simulate_power_flow(&net, Some("SUB-01")).unwrap();
        assert_eq!(flow.substation_id, "SUB-01");
        assert!(flow.total_load_kw > 0.0);
        assert!(flow.overall_efficiency_pct > 80.0 && flow.overall_efficiency_pct <= 100.0);
        assert!(!flow.transformer_loading.is_empty());

        // Check that transformers have utilization calculated
        for t in &flow.transformer_loading {
            assert!(t.rated_capacity_kva > 0.0);
            assert!(t.utilization_pct >= 0.0);
        }
    }

    #[test]
    fn crew_dispatch_spatial_optimization() {
        let mut net = create_sample_network();

        let fault = trigger_fault(&mut net, "T-104", FaultType::TransformerFailure, 1700000000000).unwrap();

        // Find best crew: should pick Rapid Response Alpha because of TRANSFORMER skill match and proximity
        let rec = find_best_crew(&net, &fault.fault_number).unwrap();
        assert_eq!(rec.crew.crew_code, "CREW-ALPHA");
        assert!(rec.skill_matched);
        assert!(rec.distance_km > 0.0);
        assert!(rec.travel_minutes > 0.0);
        assert!(rec.repair_minutes >= 120.0);
    }

    #[test]
    fn scada_telemetry_anomaly_detection() {
        let mut net = create_sample_network();

        let readings = vec![
            // Severe under-voltage
            MeterTelemetry {
                meter_number: "MTR-HOSP-01".to_string(),
                voltage_v: 198.5,
                current_a: 12.0,
                active_power_kw: 2.38,
                energy_kwh: 1450.0,
                power_factor: 0.95,
                timestamp_ms: 1700000010000,
            },
            // Complete loss of voltage (outage)
            MeterTelemetry {
                meter_number: "MTR-RES-004".to_string(),
                voltage_v: 0.0,
                current_a: 0.0,
                active_power_kw: 0.0,
                energy_kwh: 420.0,
                power_factor: 1.0,
                timestamp_ms: 1700000010000,
            },
            // High over-current
            MeterTelemetry {
                meter_number: "MTR-IND-01".to_string(),
                voltage_v: 232.0,
                current_a: 78.4,
                active_power_kw: 17.5,
                energy_kwh: 8900.0,
                power_factor: 0.96,
                timestamp_ms: 1700000010000,
            },
        ];

        let anomalies = ingest_telemetry_batch(&mut net, readings);

        assert_eq!(anomalies.len(), 3);
        assert!(anomalies.iter().any(|a| a.anomaly_type == "UNDER_VOLTAGE"));
        assert!(anomalies.iter().any(|a| a.anomaly_type == "OUTAGE_DETECTED"));
        assert!(anomalies.iter().any(|a| a.anomaly_type == "OVER_CURRENT"));

        // Meter status should reflect detected condition
        assert_eq!(net.meters["MTR-HOSP-01"].status, MeterStatus::VoltageAnomaly);
        assert_eq!(net.meters["MTR-RES-004"].status, MeterStatus::Outage);
    }

    #[test]
    fn digital_twin_tree_and_privacy_masking() {
        let net = create_sample_network();

        // 1. Dispatcher sees unmasked customer names
        let tree_dispatcher = net.build_network_tree("SUB-01", UserRole::Dispatcher).unwrap();
        let tree_str = tree_dispatcher.to_string();
        assert!(tree_str.contains("Juru General Hospital"));

        // 2. Field engineer sees masked customer names
        let tree_engineer = net.build_network_tree("SUB-01", UserRole::FieldEngineer).unwrap();
        let masked_str = tree_engineer.to_string();
        assert!(!masked_str.contains("Juru General Hospital"));
        assert!(masked_str.contains("J*** G****** H*******"));
    }

    #[test]
    fn powergis_layer_generation() {
        let net = create_sample_network();

        let layers = all_grid_datasets(&net, UserRole::Dispatcher);
        assert_eq!(layers.len(), 8);

        // Check dataset names
        let names: Vec<&str> = layers.iter().map(|ds| ds.name.as_str()).collect();
        assert!(names.contains(&"Grid Substations"));
        assert!(names.contains(&"Grid Feeders"));
        assert!(names.contains(&"Power Lines & Segments"));
        assert!(names.contains(&"Distribution Transformers"));
        assert!(names.contains(&"Distribution Poles"));
        assert!(names.contains(&"Smart Meters"));
        assert!(names.contains(&"Grid Faults"));
        assert!(names.contains(&"Response Crews"));

        // Check feature count in meters
        let meter_ds = layers.iter().find(|ds| ds.name == "Smart Meters").unwrap();
        assert_eq!(meter_ds.features.len(), net.meters.len());
    }
}
