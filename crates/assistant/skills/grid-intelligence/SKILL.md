---
name: grid-intelligence
description: Electrical distribution network intelligence (GridSense) — fault detection, impact assessment, customer categorization, dynamic priority scoring, predictive risk hotspots, radial power flow, SCADA telemetry anomaly alerts, and crew dispatch.
---

# Grid Intelligence (GridSense)

PowerGIS includes a complete electrical distribution network analysis and operational intelligence suite (`grid.*` commands).

## Common User Requests & Commands

1. **Load or Reset Electrical Grid Sample Network**:
   - Run `grid.sample` `{}`
   - Loads the 132kV/33kV/11kV/0.4kV reference distribution network (substations, feeders, line segments, distribution transformers, poles, smart meters, customer accounts, and response crews).

2. **Add Grid Infrastructure Layers to Map**:
   - Run `grid.createLayers` `{"role": "DISPATCHER"}`
   - Creates and renders 8 vector layers: `Grid Substations`, `Grid Feeders`, `Power Lines & Segments`, `Distribution Transformers`, `Distribution Poles`, `Smart Meters`, `Grid Faults`, and `Response Crews`.
   - Use `"role": "FIELD_ENGINEER"` or `"GUEST"` to mask customer personally identifiable information (PII).

3. **Check Control Room Operational Status / KPIs**:
   - Run `grid.status` `{}`
   - Returns active faults, impacted meters count, critical customers at risk, available response crews, and daily revenue loss.

4. **Analyze Fault Impact (Pre-Analysis / What-If)**:
   - Run `grid.analyzeImpact` `{"assetId": "T-104", "faultType": "TRANSFORMER_FAILURE"}`
   - Returns affected meters count, breakdown across customer categories (Residential, Commercial, Industrial, Agricultural, Government), critical facility names, daily revenue exposure, and dynamic priority score (0-100).

5. **Trigger / Simulate a Grid Fault**:
   - Run `grid.triggerFault` `{"assetId": "T-104", "faultType": "TRANSFORMER_FAILURE"}`
   - Marks asset as FAULTED, disconnects downstream meters to OUTAGE status, generates a `FLT-XXXX` fault record, and updates the map.

6. **Dispatch / Assign Response Crew**:
   - Run `grid.assignCrew` `{"faultNumber": "FLT-0001"}`
   - Automatically evaluates available crews, calculates geodesic distance, matches technical certifications (e.g. High Voltage, Transformer specialist), estimates travel time (ETA) and repair duration, and updates crew status to EN_ROUTE.

7. **Restore a Fault**:
   - Run `grid.restoreFault` `{"faultNumber": "FLT-0001"}`
   - Re-energizes the asset back to ACTIVE, restores all connected meters, and frees the assigned response crew back to AVAILABLE.

8. **Predictive Asset Risk & Maintenance Recommendations**:
   - Run `grid.assessRisk` `{"assetId": "T-104", "currentYear": 2026}`
   - Evaluates multi-factor risk: asset age, historical failures, vegetation proximity, weather exposure, and connected load. Returns failure probability %, risk score, and preventive maintenance action items.

9. **Identify Geographic Risk Hotspots**:
   - Run `grid.hotspots` `{"metric": "customer_impact"}` (or `"risk_score"`, `"revenue_exposure"`)
   - Returns sorted concentration hotspots with intensity scores and coordinates.

10. **Advanced Power Flow Simulation**:
    - Run `grid.powerFlow` `{"substationId": "SUB-01"}`
    - Solves radial load flow: calculates active power flow, branch current, voltage drop along lines, per-unit bus voltages, transformer capacity utilization %, and technical I²R line losses. Flags under-voltage and equipment overloads.

11. **Ingest SCADA Telemetry Stream**:
    - Run `grid.ingestTelemetry` `{"readings": [...]}`
    - Monitors real-time telemetry against utility standards and flags alerts: UNDER_VOLTAGE, OVER_VOLTAGE, OVER_CURRENT, OUTAGE_DETECTED, and LOW_POWER_FACTOR.

12. **Explore Digital Twin Network Tree**:
    - Run `grid.networkTree` `{"rootId": "SUB-01"}`
    - Traverses the upstream/downstream graph hierarchy and returns the topological tree from substation down to individual meters.
