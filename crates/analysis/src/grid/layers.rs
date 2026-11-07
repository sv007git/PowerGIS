//! Conversion of electrical distribution grid assets to PowerGIS vector Datasets.
//!
//! Enables native spatial rendering, styling, filtering, selection, and attribute table
//! inspection of grid infrastructure across PowerGIS desktop and web maps.

use rustgis_data::{Dataset, Field, FieldType, Geometry, GeometryType, Value};

use super::model::{GridAssetType, UserRole};
use super::topology::ElectricalNetwork;

/// Generates a point dataset of all high-voltage substations.
pub fn substations_dataset(network: &ElectricalNetwork) -> Dataset {
    let fields = vec![
        Field::new("asset_id", FieldType::Text),
        Field::new("name", FieldType::Text),
        Field::new("voltage_kv", FieldType::Double),
        Field::new("capacity_kva", FieldType::Double),
        Field::new("status", FieldType::Text),
        Field::new("install_year", FieldType::Integer),
    ];

    let mut ds = Dataset::new("Grid Substations", GeometryType::Point, fields);

    for node in network.nodes.values() {
        if node.asset_type == GridAssetType::Substation {
            let geom = Some(Geometry::Point(node.coord));
            let attrs = vec![
                Value::Text(node.asset_id.clone()),
                Value::Text(node.name.clone()),
                Value::Float(node.voltage_kv),
                Value::Float(node.capacity_kva),
                Value::Text(node.status.as_str().to_string()),
                Value::Int(node.install_year as i64),
            ];
            ds.push(geom, attrs);
        }
    }

    ds
}

/// Generates a point dataset of all distribution feeders.
pub fn feeders_dataset(network: &ElectricalNetwork) -> Dataset {
    let fields = vec![
        Field::new("asset_id", FieldType::Text),
        Field::new("name", FieldType::Text),
        Field::new("voltage_kv", FieldType::Double),
        Field::new("substation_id", FieldType::Text),
        Field::new("capacity_kva", FieldType::Double),
        Field::new("status", FieldType::Text),
    ];

    let mut ds = Dataset::new("Grid Feeders", GeometryType::Point, fields);

    for node in network.nodes.values() {
        if node.asset_type == GridAssetType::Feeder {
            let geom = Some(Geometry::Point(node.coord));
            let parent = node.parent_asset_id.clone().unwrap_or_default();
            let attrs = vec![
                Value::Text(node.asset_id.clone()),
                Value::Text(node.name.clone()),
                Value::Float(node.voltage_kv),
                Value::Text(parent),
                Value::Float(node.capacity_kva),
                Value::Text(node.status.as_str().to_string()),
            ];
            ds.push(geom, attrs);
        }
    }

    ds
}

/// Generates a LineString dataset of conductive power lines and line segments.
pub fn power_lines_dataset(network: &ElectricalNetwork) -> Dataset {
    let fields = vec![
        Field::new("edge_id", FieldType::Text),
        Field::new("from_node", FieldType::Text),
        Field::new("to_node", FieldType::Text),
        Field::new("edge_type", FieldType::Text),
        Field::new("status", FieldType::Text),
        Field::new("length_m", FieldType::Double),
        Field::new("resistance_ohm", FieldType::Double),
        Field::new("max_current_a", FieldType::Double),
    ];

    let mut ds = Dataset::new("Power Lines & Segments", GeometryType::Line, fields);

    for edge in &network.edges {
        let geom = if edge.path.len() >= 2 {
            Some(Geometry::Line(vec![edge.path.clone()]))
        } else if let (Some(from), Some(to)) = (network.nodes.get(&edge.from_node), network.nodes.get(&edge.to_node)) {
            Some(Geometry::Line(vec![vec![from.coord, to.coord]]))
        } else {
            None
        };

        let attrs = vec![
            Value::Text(edge.edge_id.clone()),
            Value::Text(edge.from_node.clone()),
            Value::Text(edge.to_node.clone()),
            Value::Text(edge.edge_type.as_str().to_string()),
            Value::Text(edge.status.clone()),
            Value::Float(edge.length_m),
            Value::Float(edge.resistance_ohm),
            Value::Float(edge.max_current_a),
        ];

        ds.push(geom, attrs);
    }

    ds
}

/// Generates a point dataset of distribution transformers with connected load statistics.
pub fn transformers_dataset(network: &ElectricalNetwork) -> Dataset {
    let fields = vec![
        Field::new("transformer_id", FieldType::Text),
        Field::new("name", FieldType::Text),
        Field::new("voltage_kv", FieldType::Double),
        Field::new("capacity_kva", FieldType::Double),
        Field::new("connected_meters", FieldType::Integer),
        Field::new("install_year", FieldType::Integer),
        Field::new("status", FieldType::Text),
    ];

    let mut ds = Dataset::new("Distribution Transformers", GeometryType::Point, fields);

    for node in network.nodes.values() {
        if node.asset_type == GridAssetType::Transformer {
            let meters = network.get_connected_meters(&node.asset_id);
            let geom = Some(Geometry::Point(node.coord));
            let attrs = vec![
                Value::Text(node.asset_id.clone()),
                Value::Text(node.name.clone()),
                Value::Float(node.voltage_kv),
                Value::Float(node.capacity_kva),
                Value::Int(meters.len() as i64),
                Value::Int(node.install_year as i64),
                Value::Text(node.status.as_str().to_string()),
            ];
            ds.push(geom, attrs);
        }
    }

    ds
}

/// Generates a point dataset of distribution poles.
pub fn poles_dataset(network: &ElectricalNetwork) -> Dataset {
    let fields = vec![
        Field::new("pole_id", FieldType::Text),
        Field::new("name", FieldType::Text),
        Field::new("transformer_id", FieldType::Text),
        Field::new("status", FieldType::Text),
    ];

    let mut ds = Dataset::new("Distribution Poles", GeometryType::Point, fields);

    for node in network.nodes.values() {
        if node.asset_type == GridAssetType::Pole {
            let geom = Some(Geometry::Point(node.coord));
            let parent = node.parent_asset_id.clone().unwrap_or_default();
            let attrs = vec![
                Value::Text(node.asset_id.clone()),
                Value::Text(node.name.clone()),
                Value::Text(parent),
                Value::Text(node.status.as_str().to_string()),
            ];
            ds.push(geom, attrs);
        }
    }

    ds
}

/// Generates a point dataset of smart meters with privacy-masked customer attributes.
pub fn meters_dataset(network: &ElectricalNetwork, role: UserRole) -> Dataset {
    let mask = role.should_mask_names();

    let fields = vec![
        Field::new("meter_number", FieldType::Text),
        Field::new("customer_number", FieldType::Text),
        Field::new("customer_name", FieldType::Text),
        Field::new("category", FieldType::Text),
        Field::new("is_critical", FieldType::Boolean),
        Field::new("critical_reason", FieldType::Text),
        Field::new("daily_kwh", FieldType::Double),
        Field::new("tariff_rate", FieldType::Double),
        Field::new("transformer_id", FieldType::Text),
        Field::new("feeder_id", FieldType::Text),
        Field::new("status", FieldType::Text),
    ];

    let mut ds = Dataset::new("Smart Meters", GeometryType::Point, fields);

    for meter in network.meters.values() {
        let geom = Some(Geometry::Point(meter.coord));
        let cust = network.customers.get(&meter.customer_number);

        let cust_name = cust
            .map(|c| if mask { c.masked_name.as_str() } else { c.name.as_str() })
            .unwrap_or("Unknown");
        let category = cust.map(|c| c.category.code()).unwrap_or("OTHER");
        let is_crit = cust.map(|c| c.is_critical).unwrap_or(false);
        let crit_reason = cust.and_then(|c| c.critical_reason.as_deref()).unwrap_or("");
        let daily_kwh = cust.map(|c| c.avg_consumption_kwh_day).unwrap_or(15.0);
        let tariff = cust.map(|c| c.default_tariff_rate).unwrap_or(0.14);

        let attrs = vec![
            Value::Text(meter.meter_number.clone()),
            Value::Text(meter.customer_number.clone()),
            Value::Text(cust_name.to_string()),
            Value::Text(category.to_string()),
            Value::Bool(is_crit),
            Value::Text(crit_reason.to_string()),
            Value::Float(daily_kwh),
            Value::Float(tariff),
            Value::Text(meter.transformer_id.clone()),
            Value::Text(meter.feeder_id.clone()),
            Value::Text(meter.status.as_str().to_string()),
        ];

        ds.push(geom, attrs);
    }

    ds
}

/// Generates a point dataset of active and historical grid faults.
pub fn faults_dataset(network: &ElectricalNetwork) -> Dataset {
    let fields = vec![
        Field::new("fault_number", FieldType::Text),
        Field::new("asset_id", FieldType::Text),
        Field::new("asset_name", FieldType::Text),
        Field::new("fault_type", FieldType::Text),
        Field::new("priority", FieldType::Text),
        Field::new("priority_score", FieldType::Double),
        Field::new("affected_meters", FieldType::Integer),
        Field::new("critical_affected", FieldType::Integer),
        Field::new("daily_revenue_loss", FieldType::Double),
        Field::new("status", FieldType::Text),
        Field::new("explanation", FieldType::Text),
    ];

    let mut ds = Dataset::new("Grid Faults", GeometryType::Point, fields);

    for fault in network.faults.values() {
        let geom = Some(Geometry::Point(fault.coord));
        let attrs = vec![
            Value::Text(fault.fault_number.clone()),
            Value::Text(fault.asset_id.clone()),
            Value::Text(fault.asset_name.clone()),
            Value::Text(fault.fault_type.as_str().to_string()),
            Value::Text(fault.priority.as_str().to_string()),
            Value::Float(fault.priority_score),
            Value::Int(fault.total_affected_meters as i64),
            Value::Int(fault.critical_customers_affected as i64),
            Value::Float(fault.estimated_revenue_loss_per_day),
            Value::Text(fault.status.clone()),
            Value::Text(fault.priority_explanation.clone()),
        ];
        ds.push(geom, attrs);
    }

    ds
}

/// Generates a point dataset of response crew locations.
pub fn crews_dataset(network: &ElectricalNetwork) -> Dataset {
    let fields = vec![
        Field::new("crew_code", FieldType::Text),
        Field::new("crew_name", FieldType::Text),
        Field::new("lead_name", FieldType::Text),
        Field::new("status", FieldType::Text),
        Field::new("contact_phone", FieldType::Text),
        Field::new("current_fault", FieldType::Text),
        Field::new("skills", FieldType::Text),
    ];

    let mut ds = Dataset::new("Response Crews", GeometryType::Point, fields);

    for crew in network.crews.values() {
        let geom = Some(Geometry::Point(crew.coord));
        let fault = crew.current_fault_number.clone().unwrap_or_default();
        let skills = crew.skills.join(", ");

        let attrs = vec![
            Value::Text(crew.crew_code.clone()),
            Value::Text(crew.crew_name.clone()),
            Value::Text(crew.lead_name.clone()),
            Value::Text(crew.status.as_str().to_string()),
            Value::Text(crew.contact_phone.clone()),
            Value::Text(fault),
            Value::Text(skills),
        ];

        ds.push(geom, attrs);
    }

    ds
}

/// Returns all 8 grid datasets ready to be added as layers to a PowerGIS map.
pub fn all_grid_datasets(network: &ElectricalNetwork, role: UserRole) -> Vec<Dataset> {
    vec![
        substations_dataset(network),
        feeders_dataset(network),
        power_lines_dataset(network),
        transformers_dataset(network),
        poles_dataset(network),
        meters_dataset(network, role),
        faults_dataset(network),
        crews_dataset(network),
    ]
}
