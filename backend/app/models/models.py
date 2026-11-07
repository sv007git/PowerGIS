import uuid
from datetime import datetime
from sqlalchemy import (
    Column, String, Integer, Numeric, Boolean, Date, DateTime,
    ForeignKey, Text, JSON, BigInteger
)
from sqlalchemy.dialects.postgresql import UUID, INET, JSONB
from geoalchemy2 import Geometry
from sqlalchemy.orm import relationship
from backend.app.core.database import Base

# ============================================================================
# Organization Models
# ============================================================================
class OrgUnit(Base):
    __tablename__ = "units"
    __table_args__ = {"schema": "organization"}

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    code = Column(String(50), unique=True, nullable=False)
    name = Column(String(150), nullable=False)
    level = Column(String(30), nullable=False)
    parent_id = Column(UUID(as_uuid=True), ForeignKey("organization.units.id"))
    headquarter = Column(String(100))
    contact_email = Column(String(100))
    contact_phone = Column(String(30))
    jurisdiction_geom = Column(Geometry("MULTIPOLYGON", srid=4326))
    is_active = Column(Boolean, default=True, nullable=False)
    created_at = Column(DateTime(timezone=True), default=datetime.utcnow)
    updated_at = Column(DateTime(timezone=True), default=datetime.utcnow, onupdate=datetime.utcnow)

    parent = relationship("OrgUnit", remote_side=[id], backref="children")

# ============================================================================
# Auth & RBAC Models
# ============================================================================
class Role(Base):
    __tablename__ = "roles"
    __table_args__ = {"schema": "auth"}

    role = Column(String(50), primary_key=True)
    name = Column(String(100), nullable=False)
    description = Column(Text)
    hierarchy_order = Column(Integer, nullable=False)
    can_view = Column(Boolean, default=True)
    can_create = Column(Boolean, default=False)
    can_edit = Column(Boolean, default=False)
    can_delete = Column(Boolean, default=False)
    can_assign = Column(Boolean, default=False)
    can_approve = Column(Boolean, default=False)
    can_reject = Column(Boolean, default=False)
    can_export = Column(Boolean, default=False)
    can_import = Column(Boolean, default=False)
    can_manage_users = Column(Boolean, default=False)
    can_manage_gis_layers = Column(Boolean, default=False)
    can_access_reports = Column(Boolean, default=True)
    can_manage_scada = Column(Boolean, default=False)

class User(Base):
    __tablename__ = "users"
    __table_args__ = {"schema": "auth"}

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    username = Column(String(60), unique=True, nullable=False)
    email = Column(String(255), unique=True, nullable=False)
    password_hash = Column(String(255), nullable=False)
    full_name = Column(String(150), nullable=False)
    phone = Column(String(30))
    designation = Column(String(100))
    role = Column(String(50), ForeignKey("auth.roles.role"), nullable=False)
    org_unit_id = Column(UUID(as_uuid=True), ForeignKey("organization.units.id"))
    is_active = Column(Boolean, default=True, nullable=False)
    failed_login_attempts = Column(Integer, default=0)
    lockout_until = Column(DateTime(timezone=True))
    last_login_at = Column(DateTime(timezone=True))
    created_at = Column(DateTime(timezone=True), default=datetime.utcnow)
    updated_at = Column(DateTime(timezone=True), default=datetime.utcnow, onupdate=datetime.utcnow)

    role_rel = relationship("Role")
    org_unit = relationship("OrgUnit")

# ============================================================================
# Asset Models
# ============================================================================
class Substation(Base):
    __tablename__ = "substations"
    __table_args__ = {"schema": "assets"}

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    code = Column(String(50), unique=True, nullable=False)
    name = Column(String(150), nullable=False)
    voltage = Column(String(20), default="110kv", nullable=False)
    capacity_mva = Column(Numeric(8, 2), nullable=False)
    num_feeders = Column(Integer, default=4, nullable=False)
    org_unit_id = Column(UUID(as_uuid=True), ForeignKey("organization.units.id"))
    commissioning_date = Column(Date)
    status = Column(String(30), default="operational", nullable=False)
    equipment_details = Column(JSON, default=dict)
    geom = Column(Geometry("POINT", srid=4326), nullable=False)
    created_at = Column(DateTime(timezone=True), default=datetime.utcnow)

class Feeder(Base):
    __tablename__ = "feeders"
    __table_args__ = {"schema": "assets"}

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    code = Column(String(50), unique=True, nullable=False)
    name = Column(String(150), nullable=False)
    substation_id = Column(UUID(as_uuid=True), ForeignKey("assets.substations.id"), nullable=False)
    voltage = Column(String(20), default="11kv", nullable=False)
    length_km = Column(Numeric(8, 2), default=0.0)
    peak_load_amps = Column(Numeric(8, 2), default=0.0)
    conductor_type = Column(String(60), default="Rabbit ACSR")
    status = Column(String(30), default="operational", nullable=False)
    geom = Column(Geometry("LINESTRING", srid=4326), nullable=False)
    created_at = Column(DateTime(timezone=True), default=datetime.utcnow)

class Transformer(Base):
    __tablename__ = "transformers"
    __table_args__ = {"schema": "assets"}

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    code = Column(String(50), unique=True, nullable=False)
    name = Column(String(150), nullable=False)
    feeder_id = Column(UUID(as_uuid=True), ForeignKey("assets.feeders.id"))
    substation_id = Column(UUID(as_uuid=True), ForeignKey("assets.substations.id"))
    capacity_kva = Column(Integer, default=250, nullable=False)
    make = Column(String(100))
    serial_number = Column(String(100))
    installation_date = Column(Date)
    load_pct = Column(Numeric(5, 2), default=0.0)
    status = Column(String(30), default="operational", nullable=False)
    geom = Column(Geometry("POINT", srid=4326), nullable=False)

class Pole(Base):
    __tablename__ = "poles"
    __table_args__ = {"schema": "assets"}

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    code = Column(String(50), unique=True, nullable=False)
    pole_number = Column(String(50))
    pole_type = Column(String(50), default="PSCC 9m")
    height_meters = Column(Numeric(4, 1), default=9.0)
    feeder_id = Column(UUID(as_uuid=True), ForeignKey("assets.feeders.id"))
    transformer_id = Column(UUID(as_uuid=True), ForeignKey("assets.transformers.id"))
    status = Column(String(30), default="operational", nullable=False)
    geom = Column(Geometry("POINT", srid=4326), nullable=False)

class Line(Base):
    __tablename__ = "lines"
    __table_args__ = {"schema": "assets"}

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    code = Column(String(50), unique=True, nullable=False)
    voltage = Column(String(20), default="11kv", nullable=False)
    feeder_id = Column(UUID(as_uuid=True), ForeignKey("assets.feeders.id"), nullable=False)
    conductor_size = Column(String(50), default="50 sq mm")
    circuit_type = Column(String(30), default="single_circuit")
    length_meters = Column(Numeric(8, 2), nullable=False)
    geom = Column(Geometry("LINESTRING", srid=4326), nullable=False)

class Consumer(Base):
    __tablename__ = "consumers"
    __table_args__ = {"schema": "assets"}

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    consumer_number = Column(String(50), unique=True, nullable=False)
    name = Column(String(150), nullable=False)
    category = Column(String(50), default="Domestic", nullable=False)
    sanctioned_load_kw = Column(Numeric(6, 2), default=5.0, nullable=False)
    meter_number = Column(String(60))
    transformer_id = Column(UUID(as_uuid=True), ForeignKey("assets.transformers.id"))
    feeder_id = Column(UUID(as_uuid=True), ForeignKey("assets.feeders.id"))
    address = Column(Text)
    contact_phone = Column(String(30))
    status = Column(String(30), default="active")
    geom = Column(Geometry("POINT", srid=4326), nullable=False)

# ============================================================================
# Ticket & Task Models
# ============================================================================
class Ticket(Base):
    __tablename__ = "tickets"
    __table_args__ = {"schema": "tickets"}

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    ticket_number = Column(String(50), unique=True, nullable=False)
    category = Column(String(80), nullable=False)
    subject = Column(String(200), nullable=False)
    description = Column(Text, nullable=False)
    priority = Column(String(20), default="medium", nullable=False)
    status = Column(String(30), default="open", nullable=False)
    org_unit_id = Column(UUID(as_uuid=True), ForeignKey("organization.units.id"))
    related_asset_type = Column(String(50))
    related_asset_id = Column(UUID(as_uuid=True))
    reported_by = Column(UUID(as_uuid=True), ForeignKey("auth.users.id"))
    assigned_to = Column(UUID(as_uuid=True), ForeignKey("auth.users.id"))
    sla_due_at = Column(DateTime(timezone=True))
    geom = Column(Geometry("POINT", srid=4326))
    created_at = Column(DateTime(timezone=True), default=datetime.utcnow)
    resolved_at = Column(DateTime(timezone=True))

class TicketComment(Base):
    __tablename__ = "comments"
    __table_args__ = {"schema": "tickets"}

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    ticket_id = Column(UUID(as_uuid=True), ForeignKey("tickets.tickets.id"), nullable=False)
    author_id = Column(UUID(as_uuid=True), ForeignKey("auth.users.id"))
    content = Column(Text, nullable=False)
    is_internal = Column(Boolean, default=False, nullable=False)
    created_at = Column(DateTime(timezone=True), default=datetime.utcnow)

class Task(Base):
    __tablename__ = "tasks"
    __table_args__ = {"schema": "tasks"}

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    task_code = Column(String(50), unique=True, nullable=False)
    ticket_id = Column(UUID(as_uuid=True), ForeignKey("tickets.tickets.id"))
    title = Column(String(200), nullable=False)
    instructions = Column(Text)
    priority = Column(String(20), default="medium", nullable=False)
    status = Column(String(30), default="created", nullable=False)
    assigned_to_user = Column(UUID(as_uuid=True), ForeignKey("auth.users.id"))
    assigned_team = Column(String(100))
    contractor_name = Column(String(100))
    due_date = Column(Date)
    geom = Column(Geometry("POINT", srid=4326))
    completion_notes = Column(Text)
    created_at = Column(DateTime(timezone=True), default=datetime.utcnow)

# ============================================================================
# Field Surveys & Capital Projects
# ============================================================================
class SurveySubmission(Base):
    __tablename__ = "submissions"
    __table_args__ = {"schema": "surveys"}

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    survey_code = Column(String(50), unique=True, nullable=False)
    surveyor_id = Column(UUID(as_uuid=True), ForeignKey("auth.users.id"), nullable=False)
    asset_type = Column(String(50), nullable=False)
    asset_code = Column(String(60))
    gps_accuracy_meters = Column(Numeric(6, 2))
    attributes = Column(JSONB, default=dict)
    is_approved = Column(Boolean)
    geom = Column(Geometry("GEOMETRY", srid=4326), nullable=False)
    sync_status = Column(String(30), default="synced")
    created_at = Column(DateTime(timezone=True), default=datetime.utcnow)

class SurveyPhoto(Base):
    __tablename__ = "photos"
    __table_args__ = {"schema": "surveys"}

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    submission_id = Column(UUID(as_uuid=True), ForeignKey("surveys.submissions.id"), nullable=False)
    photo_type = Column(String(30), default="asset_overview")
    file_url = Column(Text, nullable=False)
    caption = Column(String(255))
    latitude = Column(Numeric(10, 7))
    longitude = Column(Numeric(10, 7))
    created_at = Column(DateTime(timezone=True), default=datetime.utcnow)

class CapitalProject(Base):
    __tablename__ = "capital_projects"
    __table_args__ = {"schema": "projects"}

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    code = Column(String(50), unique=True, nullable=False)
    title = Column(String(200), nullable=False)
    description = Column(Text)
    department = Column(String(100), default="Electricity Department")
    contractor = Column(String(100))
    start_date = Column(Date, nullable=False)
    end_date = Column(Date, nullable=False)
    estimated_cost_inr = Column(Numeric(14, 2), default=0.0)
    physical_progress_pct = Column(Numeric(5, 2), default=0.0)
    financial_progress_pct = Column(Numeric(5, 2), default=0.0)
    boundary_geom = Column(Geometry("POLYGON", srid=4326))
    created_at = Column(DateTime(timezone=True), default=datetime.utcnow)

# ============================================================================
# SCADA Models (Module 27)
# ============================================================================
class ScadaGateway(Base):
    __tablename__ = "gateways"
    __table_args__ = {"schema": "scada"}

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    name = Column(String(100), nullable=False)
    protocol = Column(String(50), default="IEC-60870-5-104", nullable=False)
    ip_address = Column(String(100), nullable=False)
    port = Column(Integer, default=2404, nullable=False)
    status = Column(String(30), default="online", nullable=False)
    last_heartbeat = Column(DateTime(timezone=True), default=datetime.utcnow)
    created_at = Column(DateTime(timezone=True), default=datetime.utcnow)

class ScadaPoint(Base):
    __tablename__ = "points"
    __table_args__ = {"schema": "scada"}

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    tag = Column(String(100), unique=True, nullable=False)
    description = Column(String(200), nullable=False)
    substation_id = Column(UUID(as_uuid=True), ForeignKey("assets.substations.id"))
    feeder_id = Column(UUID(as_uuid=True), ForeignKey("assets.feeders.id"))
    point_type = Column(String(30), default="analog")
    unit = Column(String(20), default="kV")
    nominal_value = Column(Numeric(10, 2))
    warning_threshold = Column(Numeric(10, 2))
    critical_threshold = Column(Numeric(10, 2))
    current_value = Column(Numeric(10, 2))
    quality = Column(String(20), default="GOOD")
    is_active = Column(Boolean, default=True)
    last_updated = Column(DateTime(timezone=True), default=datetime.utcnow)

class ScadaAlarm(Base):
    __tablename__ = "alarms"
    __table_args__ = {"schema": "scada"}

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    point_id = Column(UUID(as_uuid=True), ForeignKey("scada.points.id"))
    alarm_code = Column(String(50), nullable=False)
    category = Column(String(60), default="feeder_trip")
    severity = Column(String(20), default="high")
    message = Column(Text, nullable=False)
    status = Column(String(30), default="active")
    acknowledged_by = Column(UUID(as_uuid=True), ForeignKey("auth.users.id"))
    ticket_id = Column(UUID(as_uuid=True), ForeignKey("tickets.tickets.id"))
    occurred_at = Column(DateTime(timezone=True), default=datetime.utcnow)
    cleared_at = Column(DateTime(timezone=True))

# ============================================================================
# Audit Logs
# ============================================================================
class AuditLog(Base):
    __tablename__ = "logs"
    __table_args__ = {"schema": "audit"}

    id = Column(BigInteger, primary_key=True, autoincrement=True)
    user_id = Column(UUID(as_uuid=True), ForeignKey("auth.users.id"))
    username = Column(String(100))
    action = Column(String(80), nullable=False)
    resource_type = Column(String(80), nullable=False)
    resource_id = Column(String(100))
    payload = Column(JSONB)
    ip_address = Column(INET)
    created_at = Column(DateTime(timezone=True), default=datetime.utcnow)
