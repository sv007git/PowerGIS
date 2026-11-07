# PowerGIS Backend & RBAC Architecture

This directory contains the database schema, PostGIS spatial models, Role-Based Access Control (RBAC), and deployment infrastructure for PowerGIS.

---

## 1. Supported Deployment Targets

| Feature | Supabase (Cloud Demo) | In-House Server (On-Premise) |
|---|---|---|
| **Database** | Managed PostgreSQL 15/16 + PostGIS | PostgreSQL 16 + PostGIS 3.4 (Docker) |
| **Authentication** | Supabase Auth (JWT) | PostgREST / mTLS / LDAP / Local RBAC |
| **REST API** | Auto-generated PostgREST | PostgREST Container |
| **Spatial Engine** | Native PostGIS | Native PostGIS |
| **Row Level Security** | Enabled per Role | Enabled per Role |

---

## 2. Role-Based Access Control (RBAC) Hierarchy

The system defines 6 specialized engineering and administrative profiles:

1. **`admin` (System Administrator)**:
   - Full read/write across all databases, projects, and users.
   - User account lifecycle, system settings, raw SQL execution, spatial index tuning.
2. **`superintendent_engineer` (Superintendent Engineer - SE)**:
   - High-level engineering & circle oversight.
   - Approves major works, capital distribution projects, reviews system audit trails.
3. **`executive_engineer` (Executive Engineer - EE)**:
   - Divisional operations head.
   - Feeder management, crew dispatch assignment, operational planning, network load analysis.
4. **`assistant_engineer` (Assistant Engineer - AE)**:
   - Substation & line maintenance in-charge.
   - Network layer updates, transformer health verification, crew arrival validation.
5. **`junior_engineer` (Junior Engineer - JE)**:
   - Field operations & line maintenance.
   - Fault logging with coordinates, transformer status reporting, field measurement inputs.
6. **`guest` (Public / Citizen Viewer)**:
   - Read-only access to published maps, feeder routes, and outage notice bulletins. No editing allowed.

---

## 3. Quick Start: Supabase Demo Setup (2 Minutes)

1. Create a free project at [supabase.com](https://supabase.com).
2. Open the **SQL Editor** in your Supabase dashboard.
3. Copy and run [`backend/supabase/migrations/20261107_init_powergis_rbac.sql`](supabase/migrations/20261107_init_powergis_rbac.sql).
4. Run [`backend/supabase/seed.sql`](supabase/seed.sql) to populate demo users, substations, and active faults in the Pondicherry region.
5. Copy your **Project URL** and **Anon Key** from `Settings › API` and paste them into the PowerGIS web app or desktop connection dialog!

---

## 4. In-House Server Deployment (Docker Compose)

For high-security in-house distribution environments without external cloud dependencies:

```bash
cd backend/inhouse
cp .env.example .env
docker compose up -d
```

Services started:
- **PostGIS Database**: `localhost:5432` (`powergis`)
- **REST API**: `http://localhost:3000`
- **pgAdmin 4**: `http://localhost:5050`
