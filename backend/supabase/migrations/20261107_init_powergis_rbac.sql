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
