-- ============================================================================
-- PowerGIS Enterprise GIS Utility Management Platform
-- Seed Data: 16 Roles, 6-Level Org Hierarchy, Users, Electrical Assets,
--            Tickets, Tasks, Surveys, Projects, SCADA Telemetry & Alarms
-- ============================================================================

-- Ensure pgcrypto extension is active
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- 1. Seed the 16 System Roles
INSERT INTO auth.roles (
    role, name, description, hierarchy_order,
    can_view, can_create, can_edit, can_delete, can_assign, can_approve,
    can_reject, can_export, can_import, can_manage_users, can_manage_gis_layers,
    can_access_reports, can_manage_scada
) VALUES
('super_admin', 'Super Administrator', 'Global root administrator with unrestricted system-wide authority', 1, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE),
('state_admin', 'State Administrator', 'State-level executive oversight and policy administration', 2, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE),
('circle_admin', 'Circle Administrator', 'Circle management across all circle divisions and capital works', 3, TRUE, TRUE, TRUE, FALSE, TRUE, TRUE, TRUE, TRUE, TRUE, FALSE, TRUE, TRUE, FALSE),
('superintending_engineer', 'Superintending Engineer', 'Circle Chief Engineer: capital approvals, high-level switching & network strategy', 4, TRUE, TRUE, TRUE, FALSE, TRUE, TRUE, TRUE, TRUE, FALSE, FALSE, TRUE, TRUE, TRUE),
('executive_engineer', 'Executive Engineer', 'Divisional Head: feeder operations, work order sanctioning, crew mobilization', 5, TRUE, TRUE, TRUE, FALSE, TRUE, TRUE, TRUE, TRUE, FALSE, FALSE, FALSE, TRUE, TRUE),
('assistant_executive_engineer', 'Assistant Executive Engineer', 'Sub-divisional operational supervisor: line maintenance & substation oversight', 6, TRUE, TRUE, TRUE, FALSE, TRUE, TRUE, FALSE, TRUE, FALSE, FALSE, FALSE, TRUE, FALSE),
('junior_engineer', 'Junior Engineer', 'Section Engineer: breakdown maintenance, fault clearance, field operations', 7, TRUE, TRUE, TRUE, FALSE, FALSE, FALSE, FALSE, TRUE, FALSE, FALSE, FALSE, TRUE, FALSE),
('gis_admin', 'GIS Administrator', 'Geospatial schema manager: layers, coordinate references, styling and snapping', 8, TRUE, TRUE, TRUE, TRUE, FALSE, FALSE, FALSE, TRUE, TRUE, FALSE, TRUE, TRUE, FALSE),
('gis_operator', 'GIS Operator', 'Digitization, spatial asset editing, GPS data cleaning, topology editing', 9, TRUE, TRUE, TRUE, FALSE, FALSE, FALSE, FALSE, TRUE, TRUE, FALSE, TRUE, FALSE, FALSE),
('section_engineer', 'Section Engineer', 'Local section technical officer: asset validation and customer metering', 10, TRUE, TRUE, TRUE, FALSE, FALSE, FALSE, FALSE, TRUE, FALSE, FALSE, FALSE, TRUE, FALSE),
('om_supervisor', 'O&M Supervisor', 'Operation and maintenance crew leader: field scheduling and tools', 11, TRUE, TRUE, TRUE, FALSE, TRUE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, TRUE, FALSE),
('field_engineer', 'Field Engineer', 'Mobile inspection, GPS asset capture, line patrolling, field repair', 12, TRUE, TRUE, TRUE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE),
('surveyor', 'Surveyor', 'Field data collection: poles, transformers, consumer geocoding, photos', 13, TRUE, TRUE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE),
('contractor', 'Contractor', 'Authorized third-party construction, cabling, and project execution crew', 14, TRUE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE),
('auditor', 'Auditor', 'Independent regulatory, safety, and operational compliance auditor', 15, TRUE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, TRUE, FALSE, FALSE, FALSE, TRUE, FALSE),
('readonly_viewer', 'Read-only Viewer', 'Public and stakeholder read-only view of network status and planned outages', 16, TRUE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE)
ON CONFLICT (role) DO UPDATE SET
    name = EXCLUDED.name,
    description = EXCLUDED.description,
    can_manage_users = EXCLUDED.can_manage_users,
    can_manage_scada = EXCLUDED.can_manage_scada;

-- 2. Seed 6-Level Organizational Hierarchy
-- State
INSERT INTO organization.units (id, code, name, level, parent_id, headquarter, contact_email, contact_phone) VALUES
('10000000-0000-0000-0000-000000000001', 'STATE-PONDY', 'Pondicherry Electricity Department', 'state', NULL, 'Gubert Avenue, Puducherry', 'secy-power.py@gov.in', '+91 413 2336331')
ON CONFLICT (code) DO NOTHING;

-- Circle
INSERT INTO organization.units (id, code, name, level, parent_id, headquarter, contact_email, contact_phone) VALUES
('10000000-0000-0000-0000-000000000002', 'CIRC-PONDY', 'Puducherry Operation Circle', 'circle', '10000000-0000-0000-0000-000000000001', 'Marapalam, Puducherry', 'se-circle.py@powergis.gov.in', '+91 413 2221102')
ON CONFLICT (code) DO NOTHING;

-- Divisions
INSERT INTO organization.units (id, code, name, level, parent_id, headquarter, contact_email, contact_phone) VALUES
('10000000-0000-0000-0000-000000000003', 'DIV-URBAN', 'Urban Distribution Division', 'division', '10000000-0000-0000-0000-000000000002', 'Venkata Nagar, Puducherry', 'ee-urban.py@powergis.gov.in', '+91 413 2221103'),
('10000000-0000-0000-0000-000000000004', 'DIV-RURAL', 'Rural Distribution Division', 'division', '10000000-0000-0000-0000-000000000002', 'Villianur, Puducherry', 'ee-rural.py@powergis.gov.in', '+91 413 2666104')
ON CONFLICT (code) DO NOTHING;

-- Subdivisions
INSERT INTO organization.units (id, code, name, level, parent_id, headquarter, contact_email, contact_phone) VALUES
('10000000-0000-0000-0000-000000000005', 'SUBDIV-MARA', 'Marapalam Sub-Division', 'subdivision', '10000000-0000-0000-0000-000000000003', 'Marapalam', 'aee-marapalam@powergis.gov.in', '+91 413 2221105'),
('10000000-0000-0000-0000-000000000006', 'SUBDIV-VILL', 'Villianur Sub-Division', 'subdivision', '10000000-0000-0000-0000-000000000004', 'Villianur', 'aee-villianur@powergis.gov.in', '+91 413 2666106')
ON CONFLICT (code) DO NOTHING;

-- Sections
INSERT INTO organization.units (id, code, name, level, parent_id, headquarter, contact_email, contact_phone) VALUES
('10000000-0000-0000-0000-000000000007', 'SEC-TOWN', 'Town Section', 'section', '10000000-0000-0000-0000-000000000005', 'White Town', 'sec-town@powergis.gov.in', '+91 413 2221107'),
('10000000-0000-0000-0000-000000000008', 'SEC-MUTH', 'Muthialpet Section', 'section', '10000000-0000-0000-0000-000000000005', 'Muthialpet', 'sec-muth@powergis.gov.in', '+91 413 2221108')
ON CONFLICT (code) DO NOTHING;

-- O&M Units
INSERT INTO organization.units (id, code, name, level, parent_id, headquarter, contact_email, contact_phone) VALUES
('10000000-0000-0000-0000-000000000009', 'OM-MARA-1', 'O&M Unit 1 (Marapalam)', 'om_unit', '10000000-0000-0000-0000-000000000007', 'Marapalam Depot', 'om-mara@powergis.gov.in', '+91 413 2221109'),
('10000000-0000-0000-0000-000000000010', 'OM-VNAG-2', 'O&M Unit 2 (Venkata Nagar)', 'om_unit', '10000000-0000-0000-0000-000000000007', 'Venkata Nagar Depot', 'om-vnag@powergis.gov.in', '+91 413 2221110')
ON CONFLICT (code) DO NOTHING;

-- 3. Seed Users for All 16 Roles (Password: PowerGIS@2026)
INSERT INTO auth.users (
    id, username, email, password_hash, full_name, phone, designation, role, org_unit_id
) VALUES
('20000000-0000-0000-0000-000000000001', 'superadmin', 'superadmin@powergis.gov.in', crypt('PowerGIS@2026', gen_salt('bf')), 'Harish Vyas', '+91 94430 11001', 'Chief System Architect', 'super_admin', '10000000-0000-0000-0000-000000000001'),
('20000000-0000-0000-0000-000000000002', 'state_admin', 'stateadmin@powergis.gov.in', crypt('PowerGIS@2026', gen_salt('bf')), 'Er. P. Rajasekharan', '+91 94430 11002', 'Director of Electricity', 'state_admin', '10000000-0000-0000-0000-000000000001'),
('20000000-0000-0000-0000-000000000003', 'circle_admin', 'circleadmin@powergis.gov.in', crypt('PowerGIS@2026', gen_salt('bf')), 'Er. N. Balamurugan', '+91 94430 11003', 'Circle Administrative Head', 'circle_admin', '10000000-0000-0000-0000-000000000002'),
('20000000-0000-0000-0000-000000000004', 'se_pondicherry', 'se.pondicherry@powergis.gov.in', crypt('PowerGIS@2026', gen_salt('bf')), 'Er. R. Sundararajan', '+91 94430 11004', 'Superintending Engineer', 'superintending_engineer', '10000000-0000-0000-0000-000000000002'),
('20000000-0000-0000-0000-000000000005', 'ee_urban', 'ee.distribution@powergis.gov.in', crypt('PowerGIS@2026', gen_salt('bf')), 'Er. M. Sivakumar', '+91 94430 11005', 'Executive Engineer (Urban)', 'executive_engineer', '10000000-0000-0000-0000-000000000003'),
('20000000-0000-0000-0000-000000000006', 'aee_marapalam', 'aee.marapalam@powergis.gov.in', crypt('PowerGIS@2026', gen_salt('bf')), 'Er. T. Senthil Kumar', '+91 94430 11006', 'Asst. Executive Engineer', 'assistant_executive_engineer', '10000000-0000-0000-0000-000000000005'),
('20000000-0000-0000-0000-000000000007', 'je_town', 'je.town@powergis.gov.in', crypt('PowerGIS@2026', gen_salt('bf')), 'Er. V. Murugan', '+91 94430 11007', 'Junior Engineer', 'junior_engineer', '10000000-0000-0000-0000-000000000007'),
('20000000-0000-0000-0000-000000000008', 'gis_admin', 'gisadmin@powergis.gov.in', crypt('PowerGIS@2026', gen_salt('bf')), 'Er. K. Deepalakshmi', '+91 94430 11008', 'GIS Center Lead', 'gis_admin', '10000000-0000-0000-0000-000000000001'),
('20000000-0000-0000-0000-000000000009', 'gis_operator', 'gisoperator@powergis.gov.in', crypt('PowerGIS@2026', gen_salt('bf')), 'S. Ananthi', '+91 94430 11009', 'Senior GIS Analyst', 'gis_operator', '10000000-0000-0000-0000-000000000003'),
('20000000-0000-0000-0000-000000000010', 'section_eng', 'seceng@powergis.gov.in', crypt('PowerGIS@2026', gen_salt('bf')), 'Er. G. Ilangovan', '+91 94430 11010', 'Section In-charge', 'section_engineer', '10000000-0000-0000-0000-000000000007'),
('20000000-0000-0000-0000-000000000011', 'om_supervisor', 'omsupervisor@powergis.gov.in', crypt('PowerGIS@2026', gen_salt('bf')), 'R. Krishnan', '+91 94430 11011', 'O&M Line Supervisor', 'om_supervisor', '10000000-0000-0000-0000-000000000009'),
('20000000-0000-0000-0000-000000000012', 'field_engineer', 'fieldeng@powergis.gov.in', crypt('PowerGIS@2026', gen_salt('bf')), 'K. Vijayakumar', '+91 94430 11012', 'Field Patrol Engineer', 'field_engineer', '10000000-0000-0000-0000-000000000009'),
('20000000-0000-0000-0000-000000000013', 'surveyor_user', 'surveyor@powergis.gov.in', crypt('PowerGIS@2026', gen_salt('bf')), 'A. Karthik', '+91 94430 11013', 'Certified GPS Surveyor', 'surveyor', '10000000-0000-0000-0000-000000000007'),
('20000000-0000-0000-0000-000000000014', 'contractor_user', 'contractor@powergis.gov.in', crypt('PowerGIS@2026', gen_salt('bf')), 'Larsen Power Infrastructure', '+91 94430 11014', 'EPC Project Contractor', 'contractor', '10000000-0000-0000-0000-000000000003'),
('20000000-0000-0000-0000-000000000015', 'auditor_user', 'auditor@powergis.gov.in', crypt('PowerGIS@2026', gen_salt('bf')), 'Dr. S. Ramachandran', '+91 94430 11015', 'Grid Compliance Auditor', 'auditor', '10000000-0000-0000-0000-000000000001'),
('20000000-0000-0000-0000-000000000016', 'guest_user', 'guest@powergis.gov.in', crypt('PowerGIS@2026', gen_salt('bf')), 'Public Citizen Viewer', '+91 94430 11016', 'Public Viewer', 'readonly_viewer', '10000000-0000-0000-0000-000000000001')
ON CONFLICT (username) DO NOTHING;

-- 4. Seed Substations (Pondicherry Region)
INSERT INTO assets.substations (
    id, code, name, voltage, capacity_mva, num_feeders, org_unit_id, commissioning_date, status, geom
) VALUES
('30000000-0000-0000-0000-000000000001', 'SS-MARA-110', 'Marapalam 110/22kV Substation', '110kv', 80.00, 6, '10000000-0000-0000-0000-000000000003', '2015-08-15', 'operational', ST_SetSRID(ST_Point(79.8183, 11.9216), 4326)),
('30000000-0000-0000-0000-000000000002', 'SS-VILL-230', 'Villianur 230/110kV Auto Substation', '230kv', 200.00, 8, '10000000-0000-0000-0000-000000000004', '2012-01-26', 'operational', ST_SetSRID(ST_Point(79.7562, 11.9168), 4326)),
('30000000-0000-0000-0000-000000000003', 'SS-LAWS-066', 'Lawspet 66/11kV Substation', '66kv', 50.00, 4, '10000000-0000-0000-0000-000000000003', '2018-05-10', 'operational', ST_SetSRID(ST_Point(79.8124, 11.9682), 4326)),
('30000000-0000-0000-0000-000000000004', 'SS-MUTH-033', 'Muthialpet 33/11kV Substation', '33kv', 25.00, 4, '10000000-0000-0000-0000-000000000003', '2020-03-22', 'operational', ST_SetSRID(ST_Point(79.8340, 11.9540), 4326)),
('30000000-0000-0000-0000-000000000005', 'SS-VNAG-033', 'Venkata Nagar 33/11kV Substation', '33kv', 30.00, 5, '10000000-0000-0000-0000-000000000003', '2019-11-04', 'operational', ST_SetSRID(ST_Point(79.8248, 11.9360), 4326))
ON CONFLICT (code) DO NOTHING;

-- 5. Seed Feeders (LineStrings)
INSERT INTO assets.feeders (
    id, code, name, substation_id, voltage, length_km, peak_load_amps, conductor_type, status, geom
) VALUES
('40000000-0000-0000-0000-000000000001', 'FDR-MARA-01', 'Marapalam - Boulevard 11kV Feeder', '30000000-0000-0000-0000-000000000001', '11kv', 4.80, 245.0, 'Rabbit ACSR', 'operational', ST_SetSRID(ST_GeomFromText('LINESTRING(79.8183 11.9216, 79.8210 11.9250, 79.8250 11.9310, 79.8290 11.9340, 79.8310 11.9370)'), 4326)),
('40000000-0000-0000-0000-000000000002', 'FDR-VNAG-02', 'Venkata Nagar - Heritage Town 11kV', '30000000-0000-0000-0000-000000000005', '11kv', 3.60, 280.0, 'Dog ACSR', 'operational', ST_SetSRID(ST_GeomFromText('LINESTRING(79.8248 11.9360, 79.8270 11.9390, 79.8310 11.9420, 79.8350 11.9460)'), 4326)),
('40000000-0000-0000-0000-000000000003', 'FDR-LAWS-03', 'Lawspet - Airport Road 11kV Feeder', '30000000-0000-0000-0000-000000000003', '11kv', 5.20, 195.0, 'Rabbit ACSR', 'operational', ST_SetSRID(ST_GeomFromText('LINESTRING(79.8124 11.9682, 79.8140 11.9650, 79.8160 11.9600, 79.8200 11.9560)'), 4326)),
('40000000-0000-0000-0000-000000000004', 'FDR-VILL-04', 'Villianur - Industrial Estate 22kV', '30000000-0000-0000-0000-000000000002', '22kv', 7.40, 310.0, 'Panther ACSR', 'operational', ST_SetSRID(ST_GeomFromText('LINESTRING(79.7562 11.9168, 79.7620 11.9180, 79.7710 11.9210, 79.7820 11.9240)'), 4326))
ON CONFLICT (code) DO NOTHING;

-- 6. Seed Transformers (DTRs)
INSERT INTO assets.transformers (
    id, code, name, feeder_id, substation_id, capacity_kva, make, serial_number, load_pct, status, geom
) VALUES
('50000000-0000-0000-0000-000000000001', 'DTR-BLVD-01', 'Boulevard North DTR 250kVA', '40000000-0000-0000-0000-000000000001', '30000000-0000-0000-0000-000000000001', 250, 'Voltamp Transformers', 'VT-2022-8812', 68.5, 'operational', ST_SetSRID(ST_Point(79.8250, 11.9310), 4326)),
('50000000-0000-0000-0000-000000000002', 'DTR-HERI-02', 'French Quarter Heritage DTR 500kVA', '40000000-0000-0000-0000-000000000002', '30000000-0000-0000-0000-000000000005', 500, 'ABB India Ltd', 'ABB-2021-9943', 82.0, 'operational', ST_SetSRID(ST_Point(79.8310, 11.9420), 4326)),
('50000000-0000-0000-0000-000000000003', 'DTR-ARPT-03', 'Airport Gateway DTR 160kVA', '40000000-0000-0000-0000-000000000003', '30000000-0000-0000-0000-000000000003', 160, 'Kirloskar Electric', 'KEC-2023-1102', 45.2, 'operational', ST_SetSRID(ST_Point(79.8160, 11.9600), 4326)),
('50000000-0000-0000-0000-000000000004', 'DTR-INDU-04', 'Sedarapet Industrial DTR 1000kVA', '40000000-0000-0000-0000-000000000004', '30000000-0000-0000-0000-000000000002', 1000, 'Schneider Electric', 'SCH-2020-4491', 94.5, 'operational', ST_SetSRID(ST_Point(79.7710, 11.9210), 4326))
ON CONFLICT (code) DO NOTHING;

-- 7. Seed Utility Poles
INSERT INTO assets.poles (
    id, code, pole_number, pole_type, height_meters, feeder_id, transformer_id, status, geom
) VALUES
('60000000-0000-0000-0000-000000000001', 'POL-MARA-001', 'P-01/MARA', 'PSCC 9m', 9.0, '40000000-0000-0000-0000-000000000001', '50000000-0000-0000-0000-000000000001', 'operational', ST_SetSRID(ST_Point(79.8210, 11.9250), 4326)),
('60000000-0000-0000-0000-000000000002', 'POL-MARA-002', 'P-02/MARA', 'PSCC 9m', 9.0, '40000000-0000-0000-0000-000000000001', '50000000-0000-0000-0000-000000000001', 'operational', ST_SetSRID(ST_Point(79.8230, 11.9280), 4326)),
('60000000-0000-0000-0000-000000000003', 'POL-HERI-010', 'P-10/HERI', 'Steel Tubular 11m', 11.0, '40000000-0000-0000-0000-000000000002', '50000000-0000-0000-0000-000000000002', 'operational', ST_SetSRID(ST_Point(79.8290, 11.9400), 4326))
ON CONFLICT (code) DO NOTHING;

-- 8. Seed Consumers
INSERT INTO assets.consumers (
    id, consumer_number, name, category, sanctioned_load_kw, meter_number, transformer_id, feeder_id, address, status, geom
) VALUES
('70000000-0000-0000-0000-000000000001', 'CON-04-102-9901', 'Alliance Francaise de Pondichery', 'Commercial', 35.0, 'MTR-LNT-44910', '50000000-0000-0000-0000-000000000002', '40000000-0000-0000-0000-000000000002', '58 Suffren Street, Puducherry', 'active', ST_SetSRID(ST_Point(79.8320, 11.9390), 4326)),
('70000000-0000-0000-0000-000000000002', 'CON-04-102-9902', 'Aurobindo Ashram Guest House', 'Domestic', 15.0, 'MTR-LNT-44911', '50000000-0000-0000-0000-000000000002', '40000000-0000-0000-0000-000000000002', 'Marine Street, White Town', 'active', ST_SetSRID(ST_Point(79.8340, 11.9430), 4326)),
('70000000-0000-0000-0000-000000000003', 'CON-04-102-9903', 'Pondicherry Central Bus Stand', 'Commercial', 65.0, 'MTR-LNT-44912', '50000000-0000-0000-0000-000000000001', '40000000-0000-0000-0000-000000000001', 'Maraimalai Adigal Salai', 'active', ST_SetSRID(ST_Point(79.8220, 11.9270), 4326))
ON CONFLICT (consumer_number) DO NOTHING;

-- 9. Seed Tickets
INSERT INTO tickets.tickets (
    id, ticket_number, category, subject, description, priority, status, org_unit_id,
    related_asset_type, related_asset_id, reported_by, assigned_to, sla_due_at, geom
) VALUES
('80000000-0000-0000-0000-000000000001', 'TKT-2026-0001', 'Feeder Trip / Outage', '11kV Feeder Overcurrent Trip at Marapalam', 'Feeder FDR-MARA-01 tripped on R-phase earth fault after tree branch contact on Maraimalai Adigal Salai.', 'critical', 'in_progress', '10000000-0000-0000-0000-000000000003', 'feeder', '40000000-0000-0000-0000-000000000001', '20000000-0000-0000-0000-000000000007', '20000000-0000-0000-0000-000000000011', NOW() + INTERVAL '2 hours', ST_SetSRID(ST_Point(79.8240, 11.9290), 4326)),
('80000000-0000-0000-0000-000000000002', 'TKT-2026-0002', 'DTR Overload', 'High Oil Temperature & Overload on French Quarter DTR', 'DTR-HERI-02 operating at 82% continuous load with oil temperature warning alarm from SCADA gateway.', 'high', 'open', '10000000-0000-0000-0000-000000000003', 'transformer', '50000000-0000-0000-0000-000000000002', '20000000-0000-0000-0000-000000000005', '20000000-0000-0000-0000-000000000006', NOW() + INTERVAL '8 hours', ST_SetSRID(ST_Point(79.8310, 11.9420), 4326)),
('80000000-0000-0000-0000-000000000003', 'TKT-2026-0003', 'GPS Coordinate Correction', 'Relocate Pole P-02/MARA Shifted for Road Expansion', 'PWD road widening shifted pole 12 meters eastward; GIS mapping requires spatial coordinate update.', 'medium', 'assigned', '10000000-0000-0000-0000-000000000007', 'pole', '60000000-0000-0000-0000-000000000002', '20000000-0000-0000-0000-000000000007', '20000000-0000-0000-0000-000000000013', NOW() + INTERVAL '24 hours', ST_SetSRID(ST_Point(79.8230, 11.9280), 4326))
ON CONFLICT (ticket_number) DO NOTHING;

-- 10. Seed Tasks / Work Orders
INSERT INTO tasks.tasks (
    id, task_code, ticket_id, title, instructions, priority, status, assigned_to_user, due_date, geom
) VALUES
('90000000-0000-0000-0000-000000000001', 'TSK-2026-0101', '80000000-0000-0000-0000-000000000001', 'Patrol and Clear Tree Overhang on Feeder FDR-MARA-01', 'Deploy bucket truck and tree trimming gang to span between Pole 1 and Pole 3 on Maraimalai Adigal Salai.', 'critical', 'in_progress', '20000000-0000-0000-0000-000000000011', CURRENT_DATE, ST_SetSRID(ST_Point(79.8220, 11.9260), 4326)),
('90000000-0000-0000-0000-000000000002', 'TSK-2026-0102', '80000000-0000-0000-0000-000000000002', 'Load Balancing & Phase Redistribution on DTR-HERI-02', 'Measure neutral current and balance single-phase consumer loads to secondary LT circuit 2.', 'high', 'assigned', '20000000-0000-0000-0000-000000000006', CURRENT_DATE + INTERVAL '2 days', ST_SetSRID(ST_Point(79.8310, 11.9420), 4326))
ON CONFLICT (task_code) DO NOTHING;

-- 11. Seed Capital Projects
INSERT INTO projects.capital_projects (
    id, code, title, department, responsible_officer_id, contractor, start_date, end_date,
    estimated_cost_inr, physical_progress_pct, financial_progress_pct
) VALUES
('a0000000-0000-0000-0000-000000000001', 'PRJ-RDSS-2026', 'RDSS Underground Cabling Project (Heritage Town)', 'Electricity Department', '20000000-0000-0000-0000-000000000005', 'Larsen & Toubro Ltd', '2025-06-01', '2027-03-31', 48500000.00, 64.5, 58.2),
('a0000000-0000-0000-0000-000000000002', 'PRJ-SCADA-SMART', 'Smart Grid Substation Automation & RTU Retrofitting', 'IT & GIS Center', '20000000-0000-0000-0000-000000000001', 'Schneider Electric Systems', '2025-10-15', '2026-12-31', 21200000.00, 78.0, 75.0)
ON CONFLICT (code) DO NOTHING;

-- 12. Seed SCADA Gateways & Telemetry Points (Module 27)
INSERT INTO scada.gateways (
    id, name, protocol, ip_address, port, status
) VALUES
('b0000000-0000-0000-0000-000000000001', 'Pondy Central SCADA Gateway (IEC-104)', 'IEC-60870-5-104', '192.168.10.20', 2404, 'online'),
('b0000000-0000-0000-0000-000000000002', 'Villianur RTU Gateway (Modbus TCP)', 'Modbus-TCP', '192.168.20.15', 502, 'online')
ON CONFLICT DO NOTHING;

INSERT INTO scada.points (
    id, tag, description, substation_id, feeder_id, point_type, unit, nominal_value, warning_threshold, critical_threshold, current_value, quality
) VALUES
('c0000000-0000-0000-0000-000000000001', 'MARA_110_BUS_VOLT', 'Marapalam 110kV Bus Voltage', '30000000-0000-0000-0000-000000000001', NULL, 'analog', 'kV', 110.0, 115.0, 118.0, 111.4, 'GOOD'),
('c0000000-0000-0000-0000-000000000002', 'FDR_MARA_01_AMP', 'Marapalam Feeder 1 Current (Phase A)', '30000000-0000-0000-0000-000000000001', '40000000-0000-0000-0000-000000000001', 'analog', 'A', 200.0, 260.0, 300.0, 245.8, 'GOOD'),
('c0000000-0000-0000-0000-000000000003', 'FDR_MARA_01_MW', 'Marapalam Feeder 1 Active Power', '30000000-0000-0000-0000-000000000001', '40000000-0000-0000-0000-000000000001', 'analog', 'MW', 4.5, 5.5, 6.2, 4.82, 'GOOD'),
('c0000000-0000-0000-0000-000000000004', 'FDR_MARA_01_CB_STAT', 'Marapalam Feeder 1 Breaker Status', '30000000-0000-0000-0000-000000000001', '40000000-0000-0000-0000-000000000001', 'digital', 'state', 1.0, 0.0, 0.0, 1.0, 'GOOD'),
('c0000000-0000-0000-0000-000000000005', 'GRID_FREQ_HZ', 'Pondicherry Grid Frequency', '30000000-0000-0000-0000-000000000001', NULL, 'analog', 'Hz', 50.0, 49.8, 49.5, 50.02, 'GOOD')
ON CONFLICT (tag) DO NOTHING;

-- 13. Seed SCADA Alarms
INSERT INTO scada.alarms (
    id, point_id, alarm_code, category, severity, message, status, occurred_at
) VALUES
('d0000000-0000-0000-0000-000000000001', 'c0000000-0000-0000-0000-000000000002', 'ALM-OC-401', 'Overcurrent', 'high', 'Feeder FDR-MARA-01 current exceeded 245A warning threshold (245.8A recorded)', 'active', NOW() - INTERVAL '15 minutes')
ON CONFLICT DO NOTHING;
