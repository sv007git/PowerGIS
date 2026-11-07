# Grid Intelligence (GridSense)

PowerGIS incorporates **GridSense**, an intelligent, real-time fault detection, network topology,
power flow simulation, and crew dispatch system for electrical distribution networks. It pairs
geospatial intelligence with smart meter telemetry, graph topology traversal, and predictive risk
analytics to minimize outage durations and safeguard critical infrastructure.

---

## Capabilities Overview

- **Real-Time Fault Detection & Impact Analysis**: Autonomous and simulated fault impact analysis across substations, feeders, transformers, line segments, and poles.
- **Dynamic Priority Scoring (0–100)**: Multi-factor scoring weighting customer volume, critical facility impact (hospitals, emergency dispatch), line voltage tier, and economic revenue exposure.
- **Topological Digital Twin**: Graph-based upstream and downstream network tree traversal with automated customer privacy masking.
- **Predictive Risk & Hotspot Analytics**: Multi-factor failure probability modeling (asset age, historical faults, weather, vegetation encroachment) and spatial concentration hotspot mapping.
- **Advanced Power Flow Simulation**: Radial load flow calculating voltage drop profiles along feeders, transformer capacity utilization %, and technical $I^2R$ line losses.
- **Intelligent Crew Dispatch**: Geodesic proximity routing, skill matching (e.g. High Voltage, Transformer specialist), ETA calculation, and restoration time estimation.
- **SCADA Telemetry Stream Monitoring**: Automated threshold anomaly detection for under-voltage, over-voltage, overcurrent, and power factor degradation.
- **Native GIS Vector Layers**: Generates styled, queryable PowerGIS vector layers for substations, feeders, power lines, transformers, poles, smart meters, faults, and crews.

---

## Ribbon & UI Integration

In the **Analyze** tab of the ribbon, open the **Grid Intelligence** group:

- **Grid Layers**: Adds all 8 distribution network layers onto the active map canvas with styling and full attribute tables.
- **Power Flow**: Runs the radial distribution power flow simulation on the network.
- **Grid Status**: Opens the real-time operational status summary.
- **Hotspots**: Computes predictive risk and customer exposure hotspots.
- **Reset Sample**: Re-initializes the Juru/Goromonzi 132kV/33kV reference distribution network.

---

## Command Reference (`grid.*`)

Every action in GridSense is a scriptable PowerGIS command:

| Command ID | Title | Summary & Parameters |
|---|---|---|
| `grid.sample` | Load Sample Grid | Loads or resets the reference electrical network.<br>`{}` |
| `grid.createLayers` | Create Grid Layers | Adds 8 vector layers onto the map.<br>`{"role"?: "ADMIN" \| "DISPATCHER" \| "FIELD_ENGINEER" \| "GUEST"}` |
| `grid.status` | Grid Operational Status | Returns control room KPI dashboard metrics.<br>`{}` |
| `grid.analyzeImpact` | Analyze Fault Impact | Evaluates impact without modifying live network state.<br>`{"assetId": "T-104", "faultType"?: "TRANSFORMER_FAILURE"}` |
| `grid.triggerFault` | Trigger Fault | Simulates a fault, setting asset to `FAULTED` and meters to `OUTAGE`.<br>`{"assetId": "T-104", "faultType"?: "..."}` |
| `grid.restoreFault` | Restore Fault | Re-energizes the asset and reconnects all affected meters.<br>`{"faultNumber": "FLT-0001"}` |
| `grid.assessRisk` | Assess Asset Risk | Evaluates failure probability % and engineering recommendations.<br>`{"assetId": "T-104", "currentYear"?: 2026}` |
| `grid.hotspots` | Risk Hotspots | Clusters geographic concentration hotspots.<br>`{"metric"?: "customer_impact" \| "risk_score" \| "revenue_exposure"}` |
| `grid.powerFlow` | Power Flow Simulation | Solves radial power flow and flags voltage/load violations.<br>`{"substationId"?: "SUB-01"}` |
| `grid.assignCrew` | Assign Response Crew | Dispatches optimal response crew using proximity and skills.<br>`{"faultNumber": "FLT-0001", "crewCode"?: "CREW-ALPHA"}` |
| `grid.ingestTelemetry` | Ingest SCADA Telemetry | Evaluates telemetry readings against ANSI/IEC limits.<br>`{"readings": [...]}` |
| `grid.networkTree` | Digital Twin Network Tree | Generates hierarchical topology tree with customer masking.<br>`{"rootId"?: "SUB-01", "role"?: "DISPATCHER"}` |

---

## Python SDK & Scripting Examples

You can script GridSense workflows directly from Python or Jupyter notebooks using the `powergis` / `rustgis` Python package:

```python
import rustgis

session = rustgis.Session()

# 1. Load the electrical grid and render layers on the map
session.run("grid.sample", {})
session.run("grid.createLayers", {"role": "DISPATCHER"})

# 2. What-if fault simulation on Transformer T-104
impact = session.run("grid.analyzeImpact", {
    "assetId": "T-104",
    "faultType": "TRANSFORMER_FAILURE"
})
print(f"Priority: {impact['priority']} (Score: {impact['priority_score']}/100)")
print(f"Affected meters: {impact['affected_meters_count']}")
print(f"Revenue loss: ${impact['estimated_revenue_loss_per_day']:.2f}/day")

# 3. Simulate fault and dispatch optimal response crew
fault = session.run("grid.triggerFault", {"assetId": "T-104"})
assignment = session.run("grid.assignCrew", {"faultNumber": fault["fault_number"]})
print(f"Assigned Crew: {assignment['crew_code']} (ETA: {assignment['estimated_travel_minutes']} mins)")

# 4. Run radial power flow simulation
flow = session.run("grid.powerFlow", {"substationId": "SUB-01"})
print(f"Total Load: {flow['total_load_kw']} kW | Losses: {flow['total_losses_kw']} kW")
print(f"Efficiency: {flow['overall_efficiency_pct']}%")

# 5. Restore fault
session.run("grid.restoreFault", {"faultNumber": fault["fault_number"]})
```
