//! Grid topology graph and recursive network traversal algorithms.
//!
//! Provides bounded upstream and downstream network tree traversal, connected
//! meter discovery, and digital twin hierarchical tree generation.

use std::collections::{HashMap, HashSet, VecDeque};

use serde_json::{Value, json};

use super::model::{CustomerProfile, GridAssetType, GridEdge, GridNode, SmartMeter, UserRole};

/// Complete electrical distribution network state in memory.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ElectricalNetwork {
    pub nodes: HashMap<String, GridNode>,
    pub edges: Vec<GridEdge>,
    pub customers: HashMap<String, CustomerProfile>,
    pub meters: HashMap<String, SmartMeter>,
    pub telemetry: HashMap<String, super::model::MeterTelemetry>,
    pub faults: HashMap<String, super::model::FaultRecord>,
    pub crews: HashMap<String, super::model::Crew>,
    pub assignments: Vec<super::model::CrewAssignment>,

    // Adjacency indices for constant-time graph lookup
    pub downstream_adj: HashMap<String, Vec<usize>>,
    pub upstream_adj: HashMap<String, Vec<usize>>,
}

impl ElectricalNetwork {
    /// Creates an empty network.
    pub fn new() -> Self {
        ElectricalNetwork::default()
    }

    /// Adds a grid node.
    pub fn add_node(&mut self, node: GridNode) {
        self.nodes.insert(node.asset_id.clone(), node);
    }

    /// Adds a connection edge.
    pub fn add_edge(&mut self, edge: GridEdge) {
        let idx = self.edges.len();
        self.downstream_adj.entry(edge.from_node.clone()).or_default().push(idx);
        self.upstream_adj.entry(edge.to_node.clone()).or_default().push(idx);
        self.edges.push(edge);
    }

    /// Adds a customer profile.
    pub fn add_customer(&mut self, customer: CustomerProfile) {
        self.customers.insert(customer.customer_number.clone(), customer);
    }

    /// Adds a smart meter.
    pub fn add_meter(&mut self, meter: SmartMeter) {
        self.meters.insert(meter.meter_number.clone(), meter);
    }

    /// Adds a response crew.
    pub fn add_crew(&mut self, crew: super::model::Crew) {
        self.crews.insert(crew.crew_code.clone(), crew);
    }

    /// Rebuilds graph adjacency indices from edges.
    pub fn rebuild_indices(&mut self) {
        self.downstream_adj.clear();
        self.upstream_adj.clear();
        for (idx, edge) in self.edges.iter().enumerate() {
            self.downstream_adj.entry(edge.from_node.clone()).or_default().push(idx);
            self.upstream_adj.entry(edge.to_node.clone()).or_default().push(idx);
        }
    }

    /// Traverses the grid network downstream from `root_asset_id` using bounded BFS.
    ///
    /// Never loops infinitely; guarantees termination with a visited set and depth limit.
    /// Returns all downstream reachable asset IDs including the root itself.
    pub fn get_downstream_asset_ids(&self, root_asset_id: &str, max_depth: usize) -> Vec<String> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        visited.insert(root_asset_id.to_string());
        result.push(root_asset_id.to_string());
        queue.push_back((root_asset_id.to_string(), 0usize));

        while let Some((curr, depth)) = queue.pop_front() {
            if depth >= max_depth {
                continue;
            }

            // Also check parent_asset_id links
            for (id, node) in &self.nodes {
                if let Some(parent) = &node.parent_asset_id
                    && parent == &curr
                    && visited.insert(id.clone())
                {
                    result.push(id.clone());
                    queue.push_back((id.clone(), depth + 1));
                }
            }

            // Check topological edges
            if let Some(edge_indices) = self.downstream_adj.get(&curr) {
                for &edge_idx in edge_indices {
                    if let Some(edge) = self.edges.get(edge_idx)
                        && visited.insert(edge.to_node.clone())
                    {
                        result.push(edge.to_node.clone());
                        queue.push_back((edge.to_node.clone(), depth + 1));
                    }
                }
            }
        }

        result
    }

    /// Traces the upstream path from `asset_id` back to the feeding substation.
    pub fn get_upstream_path(&self, asset_id: &str, max_depth: usize) -> Vec<String> {
        let mut path = Vec::new();
        let mut visited = HashSet::new();
        let mut curr = asset_id.to_string();

        visited.insert(curr.clone());
        path.push(curr.clone());

        for _ in 0..max_depth {
            // First check parent_asset_id
            if let Some(node) = self.nodes.get(&curr)
                && let Some(parent) = &node.parent_asset_id
            {
                if visited.insert(parent.clone()) {
                    path.push(parent.clone());
                    curr = parent.clone();
                    continue;
                }
                break;
            }

            // Check upstream edges
            if let Some(edges) = self.upstream_adj.get(&curr)
                && let Some(&edge_idx) = edges.first()
                && let Some(edge) = self.edges.get(edge_idx)
            {
                if visited.insert(edge.from_node.clone()) {
                    path.push(edge.from_node.clone());
                    curr = edge.from_node.clone();
                    continue;
                }
            }

            break;
        }

        path
    }

    /// Finds all smart meters connected directly or downstream from an asset.
    pub fn get_connected_meters(&self, asset_id: &str) -> Vec<&SmartMeter> {
        // First check if meters directly name this asset
        let mut direct = Vec::new();
        for m in self.meters.values() {
            if m.transformer_id == asset_id || m.feeder_id == asset_id || m.node_id == asset_id {
                direct.push(m);
            }
        }

        if !direct.is_empty() {
            return direct;
        }

        // Otherwise traverse downstream graph
        let downstream_ids = self.get_downstream_asset_ids(asset_id, 15);
        let id_set: HashSet<&str> = downstream_ids.iter().map(|s| s.as_str()).collect();

        self.meters
            .values()
            .filter(|m| {
                id_set.contains(m.transformer_id.as_str())
                    || id_set.contains(m.feeder_id.as_str())
                    || id_set.contains(m.node_id.as_str())
            })
            .collect()
    }

    /// Builds a hierarchical Digital Twin network tree from `root_id` down to smart meters.
    ///
    /// Masking is applied to customer names when requested by user role.
    pub fn build_network_tree(&self, root_id: &str, role: UserRole) -> Option<Value> {
        let node = self.nodes.get(root_id)?;
        let downstream = self.get_downstream_asset_ids(root_id, 15);
        let id_set: HashSet<&str> = downstream.iter().map(|s| s.as_str()).collect();

        let connected_meters: Vec<&SmartMeter> = self
            .meters
            .values()
            .filter(|m| {
                id_set.contains(m.transformer_id.as_str())
                    || id_set.contains(m.feeder_id.as_str())
                    || id_set.contains(m.node_id.as_str())
            })
            .collect();

        let active_count = connected_meters.iter().filter(|m| m.status == super::model::MeterStatus::Active).count();
        let outage_count = connected_meters.iter().filter(|m| m.status == super::model::MeterStatus::Outage).count();

        // Direct children
        let mut children = Vec::new();
        for (child_id, child_node) in &self.nodes {
            if child_node.parent_asset_id.as_deref() == Some(root_id) && child_id != root_id {
                if let Some(child_tree) = self.build_network_tree(child_id, role) {
                    children.push(child_tree);
                }
            }
        }

        // Direct meters under this node
        let mask = role.should_mask_names();
        let direct_meters: Vec<Value> = self
            .meters
            .values()
            .filter(|m| m.transformer_id == root_id || m.node_id == root_id)
            .map(|m| {
                let cust = self.customers.get(&m.customer_number);
                let customer_name = cust
                    .map(|c| if mask { c.masked_name.as_str() } else { c.name.as_str() })
                    .unwrap_or("Unknown Customer");
                let is_critical = cust.map(|c| c.is_critical).unwrap_or(false);
                let category = cust.map(|c| c.category.code()).unwrap_or("OTHER");

                json!({
                    "meterNumber": m.meter_number,
                    "customerNumber": m.customer_number,
                    "customerName": customer_name,
                    "category": category,
                    "isCritical": is_critical,
                    "status": m.status.as_str(),
                    "longitude": m.coord.x,
                    "latitude": m.coord.y,
                })
            })
            .collect();

        Some(json!({
            "assetId": node.asset_id,
            "name": node.name,
            "assetType": node.asset_type.as_str(),
            "voltageKv": node.voltage_kv,
            "status": node.status.as_str(),
            "capacityKva": node.capacity_kva,
            "totalConnectedMeters": connected_meters.len(),
            "activeMeters": active_count,
            "outageMeters": outage_count,
            "children": children,
            "directMeters": direct_meters,
        }))
    }
}
