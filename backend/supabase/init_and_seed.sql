-- ============================================================================
-- PowerGIS — Complete Setup: Schema Migration + RBAC Profiles + Seed Data
-- Run this single script in Supabase SQL Editor (or In-House PostgreSQL 16).
-- ============================================================================

-- ============================================================================
-- PowerGIS — Supabase & In-House PostgreSQL / PostGIS RBAC Schema
-- Supports:
--   1. Cloud Demo on Supabase
--   2. Direct deployment to In-House PostgreSQL 16 + PostGIS 3.4 Server
-- Roles:
--   - Admin
--   - Superintendent Engineer
--   - Executive Engineer
--   - Assistant Engineer
--   - Junior Engineer
--   - Guest
-- ============================================================================

-- 1. Enable Required Extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE EXTENSION IF NOT EXISTS "postgis";

-- 2. Define Enum Types
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'powergis_role') THEN
        CREATE TYPE powergis_role AS ENUM (
            'admin',
            'superintendent_engineer',
            'executive_engineer',
            'assistant_engineer',
            'junior_engineer',
            'guest'
        );
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'fault_severity') THEN
        CREATE TYPE fault_severity AS ENUM ('critical', 'high', 'medium', 'low');
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'fault_status') THEN
        CREATE TYPE fault_status AS ENUM ('reported', 'assigned', 'in_progress', 'resolved', 'closed');
    END IF;
END $$;

-- 3. Profiles Table (Linked to auth.users if available, with standalone fallback)
CREATE TABLE IF NOT EXISTS public.profiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    auth_user_id UUID,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    full_name VARCHAR(100) NOT NULL,
    role powergis_role NOT NULL DEFAULT 'guest',
    department VARCHAR(100) DEFAULT 'Electricity Department',
    division VARCHAR(100) DEFAULT 'Pondicherry Operations Circle',
    subdivision VARCHAR(100),
    phone VARCHAR(20),
    avatar_url TEXT,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 4. Role Permissions Matrix Table
CREATE TABLE IF NOT EXISTS public.role_permissions (
    role powergis_role PRIMARY KEY,
    title VARCHAR(60) NOT NULL,
    description TEXT NOT NULL,
    can_manage_users BOOLEAN NOT NULL DEFAULT FALSE,
    can_admin_system BOOLEAN NOT NULL DEFAULT FALSE,
    can_approve_work_orders BOOLEAN NOT NULL DEFAULT FALSE,
    can_dispatch_crews BOOLEAN NOT NULL DEFAULT FALSE,
    can_edit_network_layers BOOLEAN NOT NULL DEFAULT FALSE,
    can_report_faults BOOLEAN NOT NULL DEFAULT FALSE,
    can_resolve_faults BOOLEAN NOT NULL DEFAULT FALSE,
    can_view_audit_logs BOOLEAN NOT NULL DEFAULT FALSE,
    can_export_data BOOLEAN NOT NULL DEFAULT FALSE,
    can_view_map BOOLEAN NOT NULL DEFAULT TRUE
);

-- Insert or update permissions matrix for all 6 profiles
INSERT INTO public.role_permissions (
    role, title, description,
    can_manage_users, can_admin_system, can_approve_work_orders,
    can_dispatch_crews, can_edit_network_layers, can_report_faults,
    can_resolve_faults, can_view_audit_logs, can_export_data, can_view_map
) VALUES
(
    'admin',
    'System Administrator',
    'Full administrative control: user management, database connections, and system settings.',
    TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE
),
(
    'superintendent_engineer',
    'Superintendent Engineer (SE)',
    'Executive oversight across circle: approves capital works, dispatch orders, reviews system audit trails.',
    FALSE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE
),
(
    'executive_engineer',
    'Executive Engineer (EE)',
    'Divisional engineering leadership: manages grid feeders, assigns fault tickets, dispatches repair teams.',
    FALSE, FALSE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE
),
(
    'assistant_engineer',
    'Assistant Engineer (AE)',
    'Sub-division and substation supervision: maintains asset parameters, verifies field reports, inspects lines.',
    FALSE, FALSE, FALSE, TRUE, TRUE, TRUE, TRUE, FALSE, TRUE, TRUE
),
(
    'junior_engineer',
    'Junior Engineer (JE)',
    'Field engineer & inspection: reports active faults, updates transformer status, inputs maintenance logs.',
    FALSE, FALSE, FALSE, FALSE, FALSE, TRUE, TRUE, FALSE, FALSE, TRUE
),
(
    'guest',
    'Guest / Citizen Viewer',
    'Public read-only portal: views basemaps, public grid boundaries, and published outage notices.',
    FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, TRUE
)
ON CONFLICT (role) DO UPDATE SET
    title = EXCLUDED.title,
    description = EXCLUDED.description,
    can_manage_users = EXCLUDED.can_manage_users,
    can_admin_system = EXCLUDED.can_admin_system,
    can_approve_work_orders = EXCLUDED.can_approve_work_orders,
    can_dispatch_crews = EXCLUDED.can_dispatch_crews,
    can_edit_network_layers = EXCLUDED.can_edit_network_layers,
    can_report_faults = EXCLUDED.can_report_faults,
    can_resolve_faults = EXCLUDED.can_resolve_faults,
    can_view_audit_logs = EXCLUDED.can_view_audit_logs,
    can_export_data = EXCLUDED.can_export_data,
    can_view_map = EXCLUDED.can_view_map;

-- 5. Electrical Grid Spatial Tables (Pondicherry Region)

-- 5.1 Substations (Point)
CREATE TABLE IF NOT EXISTS public.grid_substations (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    substation_code VARCHAR(30) UNIQUE NOT NULL,
    voltage_kv INTEGER NOT NULL,
    capacity_mva NUMERIC(6, 2) NOT NULL,
    division VARCHAR(100) DEFAULT 'Pondicherry Operations',
    status VARCHAR(20) DEFAULT 'operational',
    geom GEOMETRY(Point, 4326) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_grid_substations_geom ON public.grid_substations USING GIST(geom);

-- 5.2 Transmission & Distribution Feeders (LineString)
CREATE TABLE IF NOT EXISTS public.grid_feeders (
    id SERIAL PRIMARY KEY,
    substation_id INTEGER REFERENCES public.grid_substations(id) ON DELETE SET NULL,
    name VARCHAR(100) NOT NULL,
    feeder_code VARCHAR(30) UNIQUE NOT NULL,
    voltage_kv INTEGER NOT NULL DEFAULT 11,
    length_km NUMERIC(6, 2) NOT NULL,
    peak_load_amps NUMERIC(6, 2) DEFAULT 0.0,
    status VARCHAR(20) DEFAULT 'energized',
    geom GEOMETRY(LineString, 4326) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_grid_feeders_geom ON public.grid_feeders USING GIST(geom);

-- 5.3 Distribution Transformers (Point)
CREATE TABLE IF NOT EXISTS public.grid_transformers (
    id SERIAL PRIMARY KEY,
    feeder_id INTEGER REFERENCES public.grid_feeders(id) ON DELETE SET NULL,
    transformer_code VARCHAR(30) UNIQUE NOT NULL,
    capacity_kva INTEGER NOT NULL,
    connected_consumers INTEGER DEFAULT 0,
    load_pct NUMERIC(5, 2) DEFAULT 0.0,
    status VARCHAR(20) DEFAULT 'healthy',
    geom GEOMETRY(Point, 4326) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_grid_transformers_geom ON public.grid_transformers USING GIST(geom);

-- 5.4 Grid Faults (Point)
CREATE TABLE IF NOT EXISTS public.grid_faults (
    id SERIAL PRIMARY KEY,
    fault_ticket VARCHAR(30) UNIQUE NOT NULL,
    feeder_id INTEGER REFERENCES public.grid_feeders(id) ON DELETE SET NULL,
    transformer_id INTEGER REFERENCES public.grid_transformers(id) ON DELETE SET NULL,
    title VARCHAR(150) NOT NULL,
    fault_type VARCHAR(50) NOT NULL,
    severity fault_severity NOT NULL DEFAULT 'medium',
    status fault_status NOT NULL DEFAULT 'reported',
    description TEXT,
    reported_by UUID REFERENCES public.profiles(id) ON DELETE SET NULL,
    assigned_to UUID REFERENCES public.profiles(id) ON DELETE SET NULL,
    approved_by UUID REFERENCES public.profiles(id) ON DELETE SET NULL,
    geom GEOMETRY(Point, 4326) NOT NULL,
    reported_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    resolved_at TIMESTAMPTZ
);
CREATE INDEX IF NOT EXISTS idx_grid_faults_geom ON public.grid_faults USING GIST(geom);

-- 5.5 Crew Dispatches
CREATE TABLE IF NOT EXISTS public.grid_dispatches (
    id SERIAL PRIMARY KEY,
    dispatch_code VARCHAR(30) UNIQUE NOT NULL,
    fault_id INTEGER REFERENCES public.grid_faults(id) ON DELETE CASCADE,
    crew_lead_id UUID REFERENCES public.profiles(id) ON DELETE SET NULL,
    vehicle_number VARCHAR(20),
    technicians_count INTEGER DEFAULT 2,
    dispatched_by UUID REFERENCES public.profiles(id) ON DELETE SET NULL,
    status VARCHAR(20) DEFAULT 'dispatched',
    dispatched_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    arrived_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ
);

-- 5.6 PowerGIS Cloud Projects & Maps
CREATE TABLE IF NOT EXISTS public.powergis_projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(150) NOT NULL,
    owner_id UUID REFERENCES public.profiles(id) ON DELETE SET NULL,
    project_json JSONB NOT NULL,
    thumbnail_url TEXT,
    is_public BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- 5.7 Audit Trail
CREATE TABLE IF NOT EXISTS public.audit_logs (
    id BIGSERIAL PRIMARY KEY,
    user_id UUID,
    role VARCHAR(50),
    action VARCHAR(50) NOT NULL,
    target_table VARCHAR(50) NOT NULL,
    record_id VARCHAR(50),
    details JSONB,
    ip_address INET,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 6. Helper Functions for RBAC & Permissions Checking

-- 6.1 Get current user's role
CREATE OR REPLACE FUNCTION public.powergis_current_role()
RETURNS powergis_role AS $$
DECLARE
    v_role powergis_role;
    v_uid UUID;
BEGIN
    v_uid := auth.uid();
    IF v_uid IS NULL THEN
        -- Fallback check for session setting or header
        RETURN 'guest'::powergis_role;
    END IF;

    SELECT role INTO v_role
    FROM public.profiles
    WHERE auth_user_id = v_uid OR id = v_uid
    LIMIT 1;

    RETURN COALESCE(v_role, 'guest'::powergis_role);
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- 6.2 Check if current user has specific capability
CREATE OR REPLACE FUNCTION public.powergis_has_permission(p_permission TEXT)
RETURNS BOOLEAN AS $$
DECLARE
    v_role powergis_role;
    v_allowed BOOLEAN := FALSE;
BEGIN
    v_role := public.powergis_current_role();

    EXECUTE format('SELECT %I FROM public.role_permissions WHERE role = $1', p_permission)
    INTO v_allowed
    USING v_role;

    RETURN COALESCE(v_allowed, FALSE);
EXCEPTION WHEN OTHERS THEN
    RETURN FALSE;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- 7. Row Level Security (RLS) Policies
ALTER TABLE public.profiles ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.grid_substations ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.grid_feeders ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.grid_transformers ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.grid_faults ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.grid_dispatches ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.powergis_projects ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.audit_logs ENABLE ROW LEVEL SECURITY;

-- 7.1 Profiles Policies
CREATE POLICY "Public profiles are viewable by all users"
ON public.profiles FOR SELECT
USING (TRUE);

CREATE POLICY "Users can update their own profile"
ON public.profiles FOR UPDATE
USING (auth.uid() = auth_user_id OR auth.uid() = id);

CREATE POLICY "Admin can manage all profiles"
ON public.profiles FOR ALL
USING (public.powergis_current_role() = 'admin');

-- 7.2 Grid Substations & Feeders Policies
CREATE POLICY "Substations viewable by all"
ON public.grid_substations FOR SELECT USING (TRUE);

CREATE POLICY "Substations modifiable by engineers and admins"
ON public.grid_substations FOR ALL
USING (public.powergis_current_role() IN ('admin', 'superintendent_engineer', 'executive_engineer', 'assistant_engineer'));

CREATE POLICY "Feeders viewable by all"
ON public.grid_feeders FOR SELECT USING (TRUE);

CREATE POLICY "Feeders modifiable by engineers and admins"
ON public.grid_feeders FOR ALL
USING (public.powergis_current_role() IN ('admin', 'superintendent_engineer', 'executive_engineer', 'assistant_engineer'));

-- 7.3 Grid Faults Policies
CREATE POLICY "Faults viewable by everyone"
ON public.grid_faults FOR SELECT USING (TRUE);

CREATE POLICY "Engineers and Admin can report faults"
ON public.grid_faults FOR INSERT
WITH CHECK (public.powergis_current_role() IN ('admin', 'superintendent_engineer', 'executive_engineer', 'assistant_engineer', 'junior_engineer'));

CREATE POLICY "Engineers and Admin can update faults"
ON public.grid_faults FOR UPDATE
USING (public.powergis_current_role() IN ('admin', 'superintendent_engineer', 'executive_engineer', 'assistant_engineer', 'junior_engineer'));

-- 7.4 Audit Logs Policies
CREATE POLICY "Superintendents and Admin can view audit logs"
ON public.audit_logs FOR SELECT
USING (public.powergis_current_role() IN ('admin', 'superintendent_engineer'));

CREATE POLICY "System can record audit logs"
ON public.audit_logs FOR INSERT WITH CHECK (TRUE);


-- ============================================================================
-- SEED DATA: AUTH USERS, PROFILES, AND PONDICHERRY ELECTRICAL GRID ASSETS
-- ============================================================================

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

