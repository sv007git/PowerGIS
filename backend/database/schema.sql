-- ============================================================================
-- PowerGIS Enterprise GIS Utility Management Platform
-- Comprehensive PostgreSQL 16 + PostGIS 3.4 Schema
-- Schemas: auth, organization, assets, network, tickets, tasks, surveys,
--          projects, scada, audit
-- ============================================================================

-- 1. Enable Extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE EXTENSION IF NOT EXISTS "postgis";

-- 2. Schemas Initialization
CREATE SCHEMA IF NOT EXISTS auth;
CREATE SCHEMA IF NOT EXISTS organization;
CREATE SCHEMA IF NOT EXISTS assets;
CREATE SCHEMA IF NOT EXISTS network;
CREATE SCHEMA IF NOT EXISTS tickets;
CREATE SCHEMA IF NOT EXISTS tasks;
CREATE SCHEMA IF NOT EXISTS surveys;
CREATE SCHEMA IF NOT EXISTS projects;
CREATE SCHEMA IF NOT EXISTS scada;
CREATE SCHEMA IF NOT EXISTS audit;

-- ============================================================================
-- SCHEMA: organization
-- 6-Level Hierarchy: State -> Circle -> Division -> Subdivision -> Section -> O&M
-- ============================================================================
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'org_unit_level') THEN
        CREATE TYPE organization.org_unit_level AS ENUM (
            'state',
            'circle',
            'division',
            'subdivision',
            'section',
            'om_unit'
        );
    END IF;
END $$;

CREATE TABLE IF NOT EXISTS organization.units (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(50) UNIQUE NOT NULL,
    name VARCHAR(150) NOT NULL,
    level organization.org_unit_level NOT NULL,
    parent_id UUID REFERENCES organization.units(id) ON DELETE RESTRICT,
    headquarter VARCHAR(100),
    contact_email VARCHAR(100),
    contact_phone VARCHAR(30),
    jurisdiction_geom GEOMETRY(MultiPolygon, 4326),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_org_units_geom ON organization.units USING GIST(jurisdiction_geom);
CREATE INDEX IF NOT EXISTS idx_org_units_parent ON organization.units(parent_id);

-- ============================================================================
-- SCHEMA: auth (16 Roles, RBAC, Users, Sessions)
-- ============================================================================
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'powergis_role_type') THEN
        CREATE TYPE auth.powergis_role_type AS ENUM (
            'super_admin',
            'state_admin',
            'circle_admin',
            'superintending_engineer',
            'executive_engineer',
            'assistant_executive_engineer',
            'junior_engineer',
            'gis_admin',
            'gis_operator',
            'section_engineer',
            'om_supervisor',
            'field_engineer',
            'surveyor',
            'contractor',
            'auditor',
            'readonly_viewer'
        );
    END IF;
END $$;

CREATE TABLE IF NOT EXISTS auth.roles (
    role auth.powergis_role_type PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    hierarchy_order INTEGER NOT NULL,
    can_view BOOLEAN NOT NULL DEFAULT TRUE,
    can_create BOOLEAN NOT NULL DEFAULT FALSE,
    can_edit BOOLEAN NOT NULL DEFAULT FALSE,
    can_delete BOOLEAN NOT NULL DEFAULT FALSE,
    can_assign BOOLEAN NOT NULL DEFAULT FALSE,
    can_approve BOOLEAN NOT NULL DEFAULT FALSE,
    can_reject BOOLEAN NOT NULL DEFAULT FALSE,
    can_export BOOLEAN NOT NULL DEFAULT FALSE,
    can_import BOOLEAN NOT NULL DEFAULT FALSE,
    can_manage_users BOOLEAN NOT NULL DEFAULT FALSE,
    can_manage_gis_layers BOOLEAN NOT NULL DEFAULT FALSE,
    can_access_reports BOOLEAN NOT NULL DEFAULT TRUE,
    can_manage_scada BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS auth.users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(60) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(150) NOT NULL,
    phone VARCHAR(30),
    designation VARCHAR(100),
    role auth.powergis_role_type NOT NULL REFERENCES auth.roles(role),
    org_unit_id UUID REFERENCES organization.units(id) ON DELETE SET NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    failed_login_attempts INTEGER NOT NULL DEFAULT 0,
    lockout_until TIMESTAMPTZ,
    last_login_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_auth_users_org ON auth.users(org_unit_id);
CREATE INDEX IF NOT EXISTS idx_auth_users_role ON auth.users(role);

CREATE TABLE IF NOT EXISTS auth.user_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL,
    ip_address INET,
    user_agent TEXT,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- SCHEMA: assets (Substations, Feeders, DTRs, Poles, Lines, Consumers)
-- ============================================================================
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'operational_status') THEN
        CREATE TYPE assets.operational_status AS ENUM (
            'planned',
            'under_construction',
            'operational',
            'maintenance',
            'tripped',
            'decommissioned'
        );
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'voltage_level') THEN
        CREATE TYPE assets.voltage_level AS ENUM (
            '400v_lt',
            '11kv',
            '22kv',
            '33kv',
            '66kv',
            '110kv',
            '230kv'
        );
    END IF;
END $$;

-- 1. Substations (Points)
CREATE TABLE IF NOT EXISTS assets.substations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(50) UNIQUE NOT NULL,
    name VARCHAR(150) NOT NULL,
    voltage assets.voltage_level NOT NULL DEFAULT '110kv',
    capacity_mva NUMERIC(8, 2) NOT NULL,
    num_feeders INTEGER NOT NULL DEFAULT 4,
    org_unit_id UUID REFERENCES organization.units(id) ON DELETE RESTRICT,
    commissioning_date DATE,
    status assets.operational_status NOT NULL DEFAULT 'operational',
    equipment_details JSONB DEFAULT '{}'::jsonb,
    geom GEOMETRY(Point, 4326) NOT NULL,
    created_by UUID REFERENCES auth.users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_substations_geom ON assets.substations USING GIST(geom);
CREATE INDEX IF NOT EXISTS idx_substations_org ON assets.substations(org_unit_id);

-- 2. Feeders (LineStrings)
CREATE TABLE IF NOT EXISTS assets.feeders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(50) UNIQUE NOT NULL,
    name VARCHAR(150) NOT NULL,
    substation_id UUID NOT NULL REFERENCES assets.substations(id) ON DELETE RESTRICT,
    voltage assets.voltage_level NOT NULL DEFAULT '11kv',
    length_km NUMERIC(8, 2) NOT NULL DEFAULT 0.0,
    peak_load_amps NUMERIC(8, 2) DEFAULT 0.0,
    conductor_type VARCHAR(60) DEFAULT 'Rabbit ACSR',
    status assets.operational_status NOT NULL DEFAULT 'operational',
    geom GEOMETRY(LineString, 4326) NOT NULL,
    created_by UUID REFERENCES auth.users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_feeders_geom ON assets.feeders USING GIST(geom);
CREATE INDEX IF NOT EXISTS idx_feeders_substation ON assets.feeders(substation_id);

-- 3. Distribution Transformers (DTRs - Points)
CREATE TABLE IF NOT EXISTS assets.transformers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(50) UNIQUE NOT NULL,
    name VARCHAR(150) NOT NULL,
    feeder_id UUID REFERENCES assets.feeders(id) ON DELETE SET NULL,
    substation_id UUID REFERENCES assets.substations(id) ON DELETE SET NULL,
    capacity_kva INTEGER NOT NULL DEFAULT 250,
    make VARCHAR(100),
    serial_number VARCHAR(100),
    installation_date DATE,
    load_pct NUMERIC(5, 2) DEFAULT 0.0,
    status assets.operational_status NOT NULL DEFAULT 'operational',
    geom GEOMETRY(Point, 4326) NOT NULL,
    created_by UUID REFERENCES auth.users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_transformers_geom ON assets.transformers USING GIST(geom);
CREATE INDEX IF NOT EXISTS idx_transformers_feeder ON assets.transformers(feeder_id);

-- 4. Utility Poles (Points)
CREATE TABLE IF NOT EXISTS assets.poles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(50) UNIQUE NOT NULL,
    pole_number VARCHAR(50),
    pole_type VARCHAR(50) DEFAULT 'PSCC 9m',
    height_meters NUMERIC(4, 1) DEFAULT 9.0,
    feeder_id UUID REFERENCES assets.feeders(id) ON DELETE SET NULL,
    transformer_id UUID REFERENCES assets.transformers(id) ON DELETE SET NULL,
    status assets.operational_status NOT NULL DEFAULT 'operational',
    geom GEOMETRY(Point, 4326) NOT NULL,
    created_by UUID REFERENCES auth.users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_poles_geom ON assets.poles USING GIST(geom);
CREATE INDEX IF NOT EXISTS idx_poles_feeder ON assets.poles(feeder_id);

-- 5. Electrical Lines (Segments with Connectivity)
CREATE TABLE IF NOT EXISTS assets.lines (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(50) UNIQUE NOT NULL,
    voltage assets.voltage_level NOT NULL DEFAULT '11kv',
    from_node_id UUID,
    to_node_id UUID,
    feeder_id UUID REFERENCES assets.feeders(id) ON DELETE CASCADE,
    conductor_size VARCHAR(50) DEFAULT '50 sq mm',
    circuit_type VARCHAR(30) DEFAULT 'single_circuit',
    length_meters NUMERIC(8, 2) NOT NULL,
    geom GEOMETRY(LineString, 4326) NOT NULL,
    created_by UUID REFERENCES auth.users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_lines_geom ON assets.lines USING GIST(geom);
CREATE INDEX IF NOT EXISTS idx_lines_feeder ON assets.lines(feeder_id);

-- 6. Consumers (Service Points)
CREATE TABLE IF NOT EXISTS assets.consumers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    consumer_number VARCHAR(50) UNIQUE NOT NULL,
    name VARCHAR(150) NOT NULL,
    category VARCHAR(50) NOT NULL DEFAULT 'Domestic',
    sanctioned_load_kw NUMERIC(6, 2) NOT NULL DEFAULT 5.0,
    meter_number VARCHAR(60),
    transformer_id UUID REFERENCES assets.transformers(id) ON DELETE SET NULL,
    feeder_id UUID REFERENCES assets.feeders(id) ON DELETE SET NULL,
    address TEXT,
    contact_phone VARCHAR(30),
    status VARCHAR(30) DEFAULT 'active',
    geom GEOMETRY(Point, 4326) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_consumers_geom ON assets.consumers USING GIST(geom);
CREATE INDEX IF NOT EXISTS idx_consumers_transformer ON assets.consumers(transformer_id);

-- ============================================================================
-- SCHEMA: tickets (Lifecycle, SLA, Assignment, History)
-- ============================================================================
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'ticket_status_type') THEN
        CREATE TYPE tickets.ticket_status_type AS ENUM (
            'open',
            'assigned',
            'accepted',
            'in_progress',
            'on_hold',
            'resolved',
            'verified',
            'closed'
        );
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'priority_level') THEN
        CREATE TYPE tickets.priority_level AS ENUM (
            'critical',
            'high',
            'medium',
            'low'
        );
    END IF;
END $$;

CREATE TABLE IF NOT EXISTS tickets.tickets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ticket_number VARCHAR(50) UNIQUE NOT NULL,
    category VARCHAR(80) NOT NULL,
    subject VARCHAR(200) NOT NULL,
    description TEXT NOT NULL,
    priority tickets.priority_level NOT NULL DEFAULT 'medium',
    status tickets.ticket_status_type NOT NULL DEFAULT 'open',
    org_unit_id UUID REFERENCES organization.units(id) ON DELETE SET NULL,
    related_asset_type VARCHAR(50),
    related_asset_id UUID,
    reported_by UUID REFERENCES auth.users(id) ON DELETE SET NULL,
    assigned_to UUID REFERENCES auth.users(id) ON DELETE SET NULL,
    approved_by UUID REFERENCES auth.users(id) ON DELETE SET NULL,
    sla_due_at TIMESTAMPTZ,
    geom GEOMETRY(Point, 4326),
    resolved_at TIMESTAMPTZ,
    closed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_tickets_geom ON tickets.tickets USING GIST(geom);
CREATE INDEX IF NOT EXISTS idx_tickets_status ON tickets.tickets(status);
CREATE INDEX IF NOT EXISTS idx_tickets_assigned ON tickets.tickets(assigned_to);

CREATE TABLE IF NOT EXISTS tickets.comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ticket_id UUID NOT NULL REFERENCES tickets.tickets(id) ON DELETE CASCADE,
    author_id UUID REFERENCES auth.users(id) ON DELETE SET NULL,
    content TEXT NOT NULL,
    is_internal BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- SCHEMA: tasks (Work Orders, Kanban, Assignments)
-- ============================================================================
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'task_status_type') THEN
        CREATE TYPE tasks.task_status_type AS ENUM (
            'created',
            'assigned',
            'accepted',
            'in_progress',
            'pending',
            'on_hold',
            'completed',
            'verified',
            'rejected',
            'cancelled'
        );
    END IF;
END $$;

CREATE TABLE IF NOT EXISTS tasks.tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    task_code VARCHAR(50) UNIQUE NOT NULL,
    ticket_id UUID REFERENCES tickets.tickets(id) ON DELETE SET NULL,
    title VARCHAR(200) NOT NULL,
    instructions TEXT,
    priority tickets.priority_level NOT NULL DEFAULT 'medium',
    status tasks.task_status_type NOT NULL DEFAULT 'created',
    assigned_to_user UUID REFERENCES auth.users(id) ON DELETE SET NULL,
    assigned_team VARCHAR(100),
    contractor_name VARCHAR(100),
    due_date DATE,
    geom GEOMETRY(Point, 4326),
    completion_notes TEXT,
    completed_at TIMESTAMPTZ,
    verified_by UUID REFERENCES auth.users(id) ON DELETE SET NULL,
    created_by UUID REFERENCES auth.users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_tasks_geom ON tasks.tasks USING GIST(geom);
CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks.tasks(status);

-- ============================================================================
-- SCHEMA: surveys (Field Operations, Dynamic Forms, Geotagged Photos)
-- ============================================================================
CREATE TABLE IF NOT EXISTS surveys.submissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    survey_code VARCHAR(50) UNIQUE NOT NULL,
    surveyor_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE RESTRICT,
    asset_type VARCHAR(50) NOT NULL,
    asset_code VARCHAR(60),
    gps_accuracy_meters NUMERIC(6, 2),
    attributes JSONB NOT NULL DEFAULT '{}'::jsonb,
    is_approved BOOLEAN,
    reviewed_by UUID REFERENCES auth.users(id) ON DELETE SET NULL,
    geom GEOMETRY(Geometry, 4326) NOT NULL,
    sync_status VARCHAR(30) DEFAULT 'synced',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_surveys_geom ON surveys.submissions USING GIST(geom);

CREATE TABLE IF NOT EXISTS surveys.photos (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    submission_id UUID REFERENCES surveys.submissions(id) ON DELETE CASCADE,
    photo_type VARCHAR(30) DEFAULT 'asset_overview',
    file_url TEXT NOT NULL,
    caption VARCHAR(255),
    latitude NUMERIC(10, 7),
    longitude NUMERIC(10, 7),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- SCHEMA: projects (Capital GIS Projects, Milestones)
-- ============================================================================
CREATE TABLE IF NOT EXISTS projects.capital_projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(50) UNIQUE NOT NULL,
    title VARCHAR(200) NOT NULL,
    description TEXT,
    department VARCHAR(100) DEFAULT 'Electricity Department',
    responsible_officer_id UUID REFERENCES auth.users(id) ON DELETE SET NULL,
    contractor VARCHAR(100),
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    estimated_cost_inr NUMERIC(14, 2) NOT NULL DEFAULT 0.0,
    physical_progress_pct NUMERIC(5, 2) DEFAULT 0.0,
    financial_progress_pct NUMERIC(5, 2) DEFAULT 0.0,
    boundary_geom GEOMETRY(Polygon, 4326),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_projects_geom ON projects.capital_projects USING GIST(boundary_geom);

-- ============================================================================
-- SCHEMA: scada (Module 27: Telemetry, Single Line Diagram, Alarms, Rules)
-- ============================================================================
CREATE TABLE IF NOT EXISTS scada.gateways (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    protocol VARCHAR(50) NOT NULL DEFAULT 'IEC-60870-5-104',
    ip_address VARCHAR(100) NOT NULL,
    port INTEGER NOT NULL DEFAULT 2404,
    status VARCHAR(30) NOT NULL DEFAULT 'online',
    last_heartbeat TIMESTAMPTZ DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS scada.points (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tag VARCHAR(100) UNIQUE NOT NULL,
    description VARCHAR(200) NOT NULL,
    substation_id UUID REFERENCES assets.substations(id) ON DELETE CASCADE,
    feeder_id UUID REFERENCES assets.feeders(id) ON DELETE SET NULL,
    transformer_id UUID REFERENCES assets.transformers(id) ON DELETE SET NULL,
    point_type VARCHAR(30) NOT NULL DEFAULT 'analog',
    unit VARCHAR(20) DEFAULT 'kV',
    nominal_value NUMERIC(10, 2),
    warning_threshold NUMERIC(10, 2),
    critical_threshold NUMERIC(10, 2),
    current_value NUMERIC(10, 2),
    quality VARCHAR(20) DEFAULT 'GOOD',
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    last_updated TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_scada_points_substation ON scada.points(substation_id);

CREATE TABLE IF NOT EXISTS scada.alarms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    point_id UUID REFERENCES scada.points(id) ON DELETE CASCADE,
    alarm_code VARCHAR(50) NOT NULL,
    category VARCHAR(60) NOT NULL DEFAULT 'feeder_trip',
    severity tickets.priority_level NOT NULL DEFAULT 'high',
    message TEXT NOT NULL,
    status VARCHAR(30) NOT NULL DEFAULT 'active',
    acknowledged_by UUID REFERENCES auth.users(id) ON DELETE SET NULL,
    acknowledged_at TIMESTAMPTZ,
    ticket_id UUID REFERENCES tickets.tickets(id) ON DELETE SET NULL,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    cleared_at TIMESTAMPTZ
);
CREATE INDEX IF NOT EXISTS idx_scada_alarms_status ON scada.alarms(status);

-- ============================================================================
-- SCHEMA: audit (System Activity & Compliance Logging)
-- ============================================================================
CREATE TABLE IF NOT EXISTS audit.logs (
    id BIGSERIAL PRIMARY KEY,
    user_id UUID REFERENCES auth.users(id) ON DELETE SET NULL,
    username VARCHAR(100),
    action VARCHAR(80) NOT NULL,
    resource_type VARCHAR(80) NOT NULL,
    resource_id VARCHAR(100),
    payload JSONB,
    ip_address INET,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_audit_logs_action ON audit.logs(action);
CREATE INDEX IF NOT EXISTS idx_audit_logs_user ON audit.logs(user_id);
