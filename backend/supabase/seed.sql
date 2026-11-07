-- ============================================================================
-- PowerGIS — Seed Data for Supabase & In-House Server
-- Seeds:
--   1. Supabase Auth Users & Demo Profiles for all 6 Roles
--   2. Pondicherry Grid Substations (110kV/66kV/33kV)
--   3. Pondicherry Distribution Feeders (11kV)
--   4. Distribution Transformers (100kVA - 500kVA)
--   5. Active Grid Fault Tickets & Dispatches
-- ============================================================================

-- Ensure pgcrypto extension is available for password encryption
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- 1. Create Supabase Auth Users (so they appear in Supabase Authentication Dashboard)
-- Default password for all 6 accounts: PowerGIS@2026
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM information_schema.schemata WHERE schema_name = 'auth') THEN
        INSERT INTO auth.users (
            instance_id,
            id,
            aud,
            role,
            email,
            encrypted_password,
            email_confirmed_at,
            raw_app_meta_data,
            raw_user_meta_data,
            created_at,
            updated_at
        ) VALUES 
        (
            '00000000-0000-0000-0000-000000000000',
            '00000000-0000-0000-0000-000000000001',
            'authenticated',
            'authenticated',
            'admin@powergis.gov.in',
            crypt('PowerGIS@2026', gen_salt('bf')),
            NOW(),
            '{"provider":"email","providers":["email"]}',
            '{"full_name":"Harish Vyas (Admin)","role":"admin"}',
            NOW(),
            NOW()
        ),
        (
            '00000000-0000-0000-0000-000000000000',
            '00000000-0000-0000-0000-000000000002',
            'authenticated',
            'authenticated',
            'se.pondicherry@powergis.gov.in',
            crypt('PowerGIS@2026', gen_salt('bf')),
            NOW(),
            '{"provider":"email","providers":["email"]}',
            '{"full_name":"Er. R. Sundararajan (SE)","role":"superintendent_engineer"}',
            NOW(),
            NOW()
        ),
        (
            '00000000-0000-0000-0000-000000000000',
            '00000000-0000-0000-0000-000000000003',
            'authenticated',
            'authenticated',
            'ee.distribution@powergis.gov.in',
            crypt('PowerGIS@2026', gen_salt('bf')),
            NOW(),
            '{"provider":"email","providers":["email"]}',
            '{"full_name":"Er. M. Sivakumar (EE)","role":"executive_engineer"}',
            NOW(),
            NOW()
        ),
        (
            '00000000-0000-0000-0000-000000000000',
            '00000000-0000-0000-0000-000000000004',
            'authenticated',
            'authenticated',
            'ae.substation@powergis.gov.in',
            crypt('PowerGIS@2026', gen_salt('bf')),
            NOW(),
            '{"provider":"email","providers":["email"]}',
            '{"full_name":"Er. K. Anand (AE)","role":"assistant_engineer"}',
            NOW(),
            NOW()
        ),
        (
            '00000000-0000-0000-0000-000000000000',
            '00000000-0000-0000-0000-000000000005',
            'authenticated',
            'authenticated',
            'je.field@powergis.gov.in',
            crypt('PowerGIS@2026', gen_salt('bf')),
            NOW(),
            '{"provider":"email","providers":["email"]}',
            '{"full_name":"Er. V. Murugan (JE)","role":"junior_engineer"}',
            NOW(),
            NOW()
        ),
        (
            '00000000-0000-0000-0000-000000000000',
            '00000000-0000-0000-0000-000000000006',
            'authenticated',
            'authenticated',
            'guest@powergis.gov.in',
            crypt('PowerGIS@2026', gen_salt('bf')),
            NOW(),
            '{"provider":"email","providers":["email"]}',
            '{"full_name":"Public Guest Viewer","role":"guest"}',
            NOW(),
            NOW()
        )
        ON CONFLICT (id) DO UPDATE SET
            encrypted_password = EXCLUDED.encrypted_password,
            email = EXCLUDED.email;

        -- Create Supabase identities safely (works across all Supabase Auth versions)
        IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_schema = 'auth' AND table_name = 'identities' AND column_name = 'provider_id') THEN
            EXECUTE 'INSERT INTO auth.identities (id, user_id, identity_data, provider, provider_id, last_sign_in_at, created_at, updated_at) VALUES 
            (''00000000-0000-0000-0000-000000000001'', ''00000000-0000-0000-0000-000000000001'', jsonb_build_object(''sub'', ''00000000-0000-0000-0000-000000000001'', ''email'', ''admin@powergis.gov.in''), ''email'', ''00000000-0000-0000-0000-000000000001'', NOW(), NOW(), NOW()),
            (''00000000-0000-0000-0000-000000000002'', ''00000000-0000-0000-0000-000000000002'', jsonb_build_object(''sub'', ''00000000-0000-0000-0000-000000000002'', ''email'', ''se.pondicherry@powergis.gov.in''), ''email'', ''00000000-0000-0000-0000-000000000002'', NOW(), NOW(), NOW()),
            (''00000000-0000-0000-0000-000000000003'', ''00000000-0000-0000-0000-000000000003'', jsonb_build_object(''sub'', ''00000000-0000-0000-0000-000000000003'', ''email'', ''ee.distribution@powergis.gov.in''), ''email'', ''00000000-0000-0000-0000-000000000003'', NOW(), NOW(), NOW()),
            (''00000000-0000-0000-0000-000000000004'', ''00000000-0000-0000-0000-000000000004'', jsonb_build_object(''sub'', ''00000000-0000-0000-0000-000000000004'', ''email'', ''ae.substation@powergis.gov.in''), ''email'', ''00000000-0000-0000-0000-000000000004'', NOW(), NOW(), NOW()),
            (''00000000-0000-0000-0000-000000000005'', ''00000000-0000-0000-0000-000000000005'', jsonb_build_object(''sub'', ''00000000-0000-0000-0000-000000000005'', ''email'', ''je.field@powergis.gov.in''), ''email'', ''00000000-0000-0000-0000-000000000005'', NOW(), NOW(), NOW()),
            (''00000000-0000-0000-0000-000000000006'', ''00000000-0000-0000-0000-000000000006'', jsonb_build_object(''sub'', ''00000000-0000-0000-0000-000000000006'', ''email'', ''guest@powergis.gov.in''), ''email'', ''00000000-0000-0000-0000-000000000006'', NOW(), NOW(), NOW())
            ON CONFLICT DO NOTHING';
        ELSE
            EXECUTE 'INSERT INTO auth.identities (id, user_id, identity_data, provider, last_sign_in_at, created_at, updated_at) VALUES 
            (''00000000-0000-0000-0000-000000000001'', ''00000000-0000-0000-0000-000000000001'', jsonb_build_object(''sub'', ''00000000-0000-0000-0000-000000000001'', ''email'', ''admin@powergis.gov.in''), ''email'', NOW(), NOW(), NOW()),
            (''00000000-0000-0000-0000-000000000002'', ''00000000-0000-0000-0000-000000000002'', jsonb_build_object(''sub'', ''00000000-0000-0000-0000-000000000002'', ''email'', ''se.pondicherry@powergis.gov.in''), ''email'', NOW(), NOW(), NOW()),
            (''00000000-0000-0000-0000-000000000003'', ''00000000-0000-0000-0000-000000000003'', jsonb_build_object(''sub'', ''00000000-0000-0000-0000-000000000003'', ''email'', ''ee.distribution@powergis.gov.in''), ''email'', NOW(), NOW(), NOW()),
            (''00000000-0000-0000-0000-000000000004'', ''00000000-0000-0000-0000-000000000004'', jsonb_build_object(''sub'', ''00000000-0000-0000-0000-000000000004'', ''email'', ''ae.substation@powergis.gov.in''), ''email'', NOW(), NOW(), NOW()),
            (''00000000-0000-0000-0000-000000000005'', ''00000000-0000-0000-0000-000000000005'', jsonb_build_object(''sub'', ''00000000-0000-0000-0000-000000000005'', ''email'', ''je.field@powergis.gov.in''), ''email'', NOW(), NOW(), NOW()),
            (''00000000-0000-0000-0000-000000000006'', ''00000000-0000-0000-0000-000000000006'', jsonb_build_object(''sub'', ''00000000-0000-0000-0000-000000000006'', ''email'', ''guest@powergis.gov.in''), ''email'', NOW(), NOW(), NOW())
            ON CONFLICT DO NOTHING';
        END IF;
    END IF;
END $$;


-- 1. Demo User Profiles (All 6 Roles)
INSERT INTO public.profiles (
    id, username, email, full_name, role, department, division, subdivision, phone
) VALUES
(
    '00000000-0000-0000-0000-000000000001',
    'admin_harish',
    'admin@powergis.gov.in',
    'Harish Vyas (Admin)',
    'admin',
    'IT & GIS Center of Excellence',
    'State Headquarters',
    'System Administration',
    '+91 94430 11001'
),
(
    '00000000-0000-0000-0000-000000000002',
    'se_pondicherry',
    'se.pondicherry@powergis.gov.in',
    'Er. R. Sundararajan (SE)',
    'superintendent_engineer',
    'Electricity Department',
    'Pondicherry Circle',
    'Planning & Monitoring',
    '+91 94430 11002'
),
(
    '00000000-0000-0000-0000-000000000003',
    'ee_urban',
    'ee.distribution@powergis.gov.in',
    'Er. M. Sivakumar (EE)',
    'executive_engineer',
    'Electricity Department',
    'Urban Distribution Division',
    'Operations & Network Planning',
    '+91 94430 11003'
),
(
    '00000000-0000-0000-0000-000000000004',
    'ae_marapalam',
    'ae.substation@powergis.gov.in',
    'Er. K. Anand (AE)',
    'assistant_engineer',
    'Electricity Department',
    'Urban Division',
    'Marapalam Sub-Division',
    '+91 94430 11004'
),
(
    '00000000-0000-0000-0000-000000000005',
    'je_field_crew',
    'je.field@powergis.gov.in',
    'Er. V. Murugan (JE)',
    'junior_engineer',
    'Electricity Department',
    'Urban Division',
    'Field Maintenance & Breakdown',
    '+91 94430 11005'
),
(
    '00000000-0000-0000-0000-000000000006',
    'guest_citizen',
    'guest@powergis.gov.in',
    'Public Guest Viewer',
    'guest',
    'Public Portal',
    'Citizen Services',
    'Outage Tracker',
    '+91 94430 11006'
)
ON CONFLICT (id) DO UPDATE SET
    username = EXCLUDED.username,
    email = EXCLUDED.email,
    full_name = EXCLUDED.full_name,
    role = EXCLUDED.role;

-- 2. Pondicherry Substations (Points in WGS84 - EPSG:4326)
INSERT INTO public.grid_substations (id, name, substation_code, voltage_kv, capacity_mva, division, status, geom) VALUES
(1, 'Marapalam 110kV Substation', 'SS-MARA-110', 110, 80.00, 'Urban Division', 'operational', ST_SetSRID(ST_Point(79.8183, 11.9216), 4326)),
(2, 'Villianur 230kV Auto Substation', 'SS-VILL-230', 230, 200.00, 'Rural Division', 'operational', ST_SetSRID(ST_Point(79.7562, 11.9168), 4326)),
(3, 'Lawspet 66kV Substation', 'SS-LAWS-066', 66, 50.00, 'North Division', 'operational', ST_SetSRID(ST_Point(79.8124, 11.9682), 4326)),
(4, 'Muthialpet 33kV Substation', 'SS-MUTH-033', 33, 25.00, 'Urban Division', 'operational', ST_SetSRID(ST_Point(79.8340, 11.9540), 4326)),
(5, 'Venkata Nagar 33kV Substation', 'SS-VNAG-033', 33, 30.00, 'Town Division', 'operational', ST_SetSRID(ST_Point(79.8248, 11.9360), 4326))
ON CONFLICT (id) DO UPDATE SET
    name = EXCLUDED.name,
    geom = EXCLUDED.geom;

-- 3. Pondicherry 11kV Distribution Feeders (LineStrings)
INSERT INTO public.grid_feeders (id, substation_id, name, feeder_code, voltage_kv, length_km, peak_load_amps, status, geom) VALUES
(
    1, 1, 'Boulevard South 11kV Feeder', 'FDR-BLVD-S', 11, 4.20, 240.0, 'energized',
    ST_SetSRID(ST_GeomFromText('LINESTRING(79.8183 11.9216, 79.8250 11.9280, 79.8310 11.9320, 79.8345 11.9335)'), 4326)
),
(
    2, 1, 'Anna Salai Commercial 11kV Feeder', 'FDR-ANNA-C', 11, 3.80, 280.0, 'energized',
    ST_SetSRID(ST_GeomFromText('LINESTRING(79.8183 11.9216, 79.8220 11.9290, 79.8260 11.9350, 79.8280 11.9390)'), 4326)
),
(
    3, 3, 'Airport Industrial 11kV Feeder', 'FDR-AIRP-IND', 11, 6.10, 310.0, 'energized',
    ST_SetSRID(ST_GeomFromText('LINESTRING(79.8124 11.9682, 79.8160 11.9750, 79.8190 11.9820, 79.8140 11.9890)'), 4326)
),
(
    4, 4, 'Heritage French Quarter 11kV Feeder', 'FDR-FR-QTR', 11, 2.90, 180.0, 'energized',
    ST_SetSRID(ST_GeomFromText('LINESTRING(79.8340 11.9540, 79.8350 11.9460, 79.8355 11.9380, 79.8348 11.9340)'), 4326)
)
ON CONFLICT (id) DO UPDATE SET
    name = EXCLUDED.name,
    geom = EXCLUDED.geom;

-- 4. Distribution Transformers (DTRs)
INSERT INTO public.grid_transformers (id, feeder_id, transformer_code, capacity_kva, connected_consumers, load_pct, status, geom) VALUES
(1, 1, 'DTR-BLVD-001', 250, 120, 68.5, 'healthy', ST_SetSRID(ST_Point(79.8250, 11.9280), 4326)),
(2, 1, 'DTR-BLVD-002', 500, 280, 84.0, 'healthy', ST_SetSRID(ST_Point(79.8310, 11.9320), 4326)),
(3, 2, 'DTR-ANNA-001', 500, 310, 92.4, 'overloaded', ST_SetSRID(ST_Point(79.8220, 11.9290), 4326)),
(4, 2, 'DTR-ANNA-002', 250, 140, 74.0, 'healthy', ST_SetSRID(ST_Point(79.8260, 11.9350), 4326)),
(5, 3, 'DTR-AIRP-001', 500, 85, 55.0, 'healthy', ST_SetSRID(ST_Point(79.8160, 11.9750), 4326)),
(6, 4, 'DTR-FQTR-001', 250, 95, 62.0, 'healthy', ST_SetSRID(ST_Point(79.8350, 11.9460), 4326))
ON CONFLICT (id) DO UPDATE SET
    transformer_code = EXCLUDED.transformer_code,
    geom = EXCLUDED.geom;

-- 5. Active Grid Faults (Pondicherry Region)
INSERT INTO public.grid_faults (
    id, fault_ticket, feeder_id, transformer_id, title, fault_type,
    severity, status, description, reported_by, assigned_to, geom, reported_at
) VALUES
(
    1,
    'FLT-PDY-2026-081',
    2,
    3,
    '11kV Cable Joint Puncture near Anna Salai Junction',
    'Cable Breakdown',
    'critical',
    'assigned',
    'Underground cable joint failure causing feeder tripping. Commercial area affected.',
    '00000000-0000-0000-0000-000000000005',
    '00000000-0000-0000-0000-000000000004',
    ST_SetSRID(ST_Point(79.8235, 11.9312), 4326),
    NOW() - INTERVAL '45 minutes'
),
(
    2,
    'FLT-PDY-2026-082',
    1,
    2,
    'HT Bushing Flashover at Boulevard DTR-002',
    'Equipment Breakdown',
    'high',
    'in_progress',
    'Heavy sea salt deposition causing flashover on B-phase bushing.',
    '00000000-0000-0000-0000-000000000005',
    '00000000-0000-0000-0000-000000000005',
    ST_SetSRID(ST_Point(79.8315, 11.9325), 4326),
    NOW() - INTERVAL '2 hours'
),
(
    3,
    'FLT-PDY-2026-083',
    4,
    6,
    'Overhead Tree Branch Touching LT Line on Goubert Avenue',
    'Vegetation Interference',
    'medium',
    'reported',
    'Intermittent sparking during high sea winds.',
    '00000000-0000-0000-0000-000000000006',
    NULL,
    ST_SetSRID(ST_Point(79.8358, 11.9420), 4326),
    NOW() - INTERVAL '15 minutes'
)
ON CONFLICT (id) DO UPDATE SET
    title = EXCLUDED.title,
    severity = EXCLUDED.severity,
    status = EXCLUDED.status,
    geom = EXCLUDED.geom;

-- 6. Dispatches
INSERT INTO public.grid_dispatches (
    id, dispatch_code, fault_id, crew_lead_id, vehicle_number, technicians_count, dispatched_by, status
) VALUES
(
    1, 'DSP-2026-042', 1, '00000000-0000-0000-0000-000000000005', 'PY-01-G-4412', 3, '00000000-0000-0000-0000-000000000003', 'dispatched'
),
(
    2, 'DSP-2026-043', 2, '00000000-0000-0000-0000-000000000005', 'PY-01-G-7820', 2, '00000000-0000-0000-0000-000000000004', 'on_site'
)
ON CONFLICT (id) DO NOTHING;
