from fastapi import APIRouter, Depends, HTTPException, status
from pydantic import BaseModel
from typing import List, Optional
from datetime import datetime, timedelta
from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy import select, func
from geoalchemy2.functions import ST_AsGeoJSON, ST_GeomFromGeoJSON
import json
import uuid

from backend.app.core.database import get_db
from backend.app.models.models import Ticket, TicketComment, User

router = APIRouter(prefix="/tickets", tags=["Ticketing & Incident Management"])

class CommentOut(BaseModel):
    id: str
    author_id: Optional[str]
    author_name: Optional[str]
    content: str
    is_internal: bool
    created_at: datetime

class TicketOut(BaseModel):
    id: str
    ticket_number: str
    category: str
    subject: str
    description: str
    priority: str
    status: str
    org_unit_id: Optional[str]
    related_asset_type: Optional[str]
    related_asset_id: Optional[str]
    reported_by: Optional[str]
    assigned_to: Optional[str]
    assigned_to_name: Optional[str]
    sla_due_at: Optional[datetime]
    created_at: datetime
    resolved_at: Optional[datetime]

class TicketCreate(BaseModel):
    category: str
    subject: str
    description: str
    priority: str = "medium"
    org_unit_id: Optional[str] = None
    related_asset_type: Optional[str] = None
    related_asset_id: Optional[str] = None
    assigned_to: Optional[str] = None
    reported_by: Optional[str] = None
    latitude: Optional[float] = None
    longitude: Optional[float] = None

class StatusUpdate(BaseModel):
    status: str
    notes: Optional[str] = None
    user_id: Optional[str] = None

class CommentCreate(BaseModel):
    author_id: Optional[str] = None
    content: str
    is_internal: bool = False

@router.get("", response_model=List[TicketOut])
async def list_tickets(
    status_filter: Optional[str] = None,
    priority: Optional[str] = None,
    assigned_to: Optional[str] = None,
    db: AsyncSession = Depends(get_db)
):
    query = select(Ticket, User.full_name.label("assigned_name")).outerjoin(User, Ticket.assigned_to == User.id)
    if status_filter:
        query = query.where(Ticket.status == status_filter)
    if priority:
        query = query.where(Ticket.priority == priority)
    if assigned_to:
        query = query.where(Ticket.assigned_to == uuid.UUID(assigned_to))
    query = query.order_by(Ticket.created_at.desc())
    res = await db.execute(query)
    
    out = []
    for t, u_name in res.all():
        out.append(TicketOut(
            id=str(t.id),
            ticket_number=t.ticket_number,
            category=t.category,
            subject=t.subject,
            description=t.description,
            priority=t.priority,
            status=t.status,
            org_unit_id=str(t.org_unit_id) if t.org_unit_id else None,
            related_asset_type=t.related_asset_type,
            related_asset_id=str(t.related_asset_id) if t.related_asset_id else None,
            reported_by=str(t.reported_by) if t.reported_by else None,
            assigned_to=str(t.assigned_to) if t.assigned_to else None,
            assigned_to_name=u_name,
            sla_due_at=t.sla_due_at,
            created_at=t.created_at,
            resolved_at=t.resolved_at
        ))
    return out

@router.post("", response_model=TicketOut)
async def create_ticket(req: TicketCreate, db: AsyncSession = Depends(get_db)):
    # Auto-generate ticket number
    count = (await db.execute(select(func.count(Ticket.id)))).scalar() or 0
    tkt_num = f"TKT-2026-{(count + 1):04d}"
    
    # SLA calculation
    hours_map = {"critical": 4, "high": 12, "medium": 24, "low": 48}
    sla_hours = hours_map.get(req.priority.lower(), 24)
    sla_due = datetime.utcnow() + timedelta(hours=sla_hours)
    
    # Optional geometry point
    geom = None
    if req.latitude is not None and req.longitude is not None:
        geom_json = json.dumps({"type": "Point", "coordinates": [req.longitude, req.latitude]})
        geom = ST_GeomFromGeoJSON(geom_json)

    ticket = Ticket(
        ticket_number=tkt_num,
        category=req.category,
        subject=req.subject,
        description=req.description,
        priority=req.priority,
        status="open",
        org_unit_id=uuid.UUID(req.org_unit_id) if req.org_unit_id else None,
        related_asset_type=req.related_asset_type,
        related_asset_id=uuid.UUID(req.related_asset_id) if req.related_asset_id else None,
        reported_by=uuid.UUID(req.reported_by) if req.reported_by else None,
        assigned_to=uuid.UUID(req.assigned_to) if req.assigned_to else None,
        sla_due_at=sla_due,
        geom=geom
    )
    db.add(ticket)
    await db.commit()
    await db.refresh(ticket)
    
    return TicketOut(
        id=str(ticket.id),
        ticket_number=ticket.ticket_number,
        category=ticket.category,
        subject=ticket.subject,
        description=ticket.description,
        priority=ticket.priority,
        status=ticket.status,
        org_unit_id=str(ticket.org_unit_id) if ticket.org_unit_id else None,
        related_asset_type=ticket.related_asset_type,
        related_asset_id=str(ticket.related_asset_id) if ticket.related_asset_id else None,
        reported_by=str(ticket.reported_by) if ticket.reported_by else None,
        assigned_to=str(ticket.assigned_to) if ticket.assigned_to else None,
        assigned_to_name=None,
        sla_due_at=ticket.sla_due_at,
        created_at=ticket.created_at,
        resolved_at=ticket.resolved_at
    )

@router.get("/{ticket_id}")
async def get_ticket(ticket_id: str, db: AsyncSession = Depends(get_db)):
    t_uuid = uuid.UUID(ticket_id)
    tkt_res = await db.execute(select(Ticket).where(Ticket.id == t_uuid))
    ticket = tkt_res.scalar_one_or_none()
    if not ticket:
        raise HTTPException(status_code=404, detail="Ticket not found")

    # Fetch comments
    c_res = await db.execute(
        select(TicketComment, User.full_name)
        .outerjoin(User, TicketComment.author_id == User.id)
        .where(TicketComment.ticket_id == t_uuid)
        .order_by(TicketComment.created_at)
    )
    comments = []
    for c, a_name in c_res.all():
        comments.append({
            "id": str(c.id),
            "author_id": str(c.author_id) if c.author_id else None,
            "author_name": a_name,
            "content": c.content,
            "is_internal": c.is_internal,
            "created_at": c.created_at
        })

    return {
        "ticket": {
            "id": str(ticket.id),
            "ticket_number": ticket.ticket_number,
            "category": ticket.category,
            "subject": ticket.subject,
            "description": ticket.description,
            "priority": ticket.priority,
            "status": ticket.status,
            "sla_due_at": ticket.sla_due_at,
            "created_at": ticket.created_at,
            "resolved_at": ticket.resolved_at
        },
        "comments": comments
    }

@router.put("/{ticket_id}/status")
async def update_ticket_status(ticket_id: str, req: StatusUpdate, db: AsyncSession = Depends(get_db)):
    allowed_stages = ["open", "assigned", "accepted", "in_progress", "on_hold", "resolved", "verified", "closed"]
    if req.status not in allowed_stages:
        raise HTTPException(status_code=400, detail=f"Invalid stage. Allowed: {allowed_stages}")
        
    t_uuid = uuid.UUID(ticket_id)
    tkt_res = await db.execute(select(Ticket).where(Ticket.id == t_uuid))
    ticket = tkt_res.scalar_one_or_none()
    if not ticket:
        raise HTTPException(status_code=404, detail="Ticket not found")

    old_status = ticket.status
    ticket.status = req.status
    if req.status in ["resolved", "verified", "closed"] and not ticket.resolved_at:
        ticket.resolved_at = datetime.utcnow()

    # Log status change comment
    if req.notes or req.status != old_status:
        comm = TicketComment(
            ticket_id=t_uuid,
            author_id=uuid.UUID(req.user_id) if req.user_id else None,
            content=f"Status transitioned from {old_status.upper()} to {req.status.upper()}." + (f" Note: {req.notes}" if req.notes else ""),
            is_internal=True
        )
        db.add(comm)

    await db.commit()
    return {"status": "success", "ticket_number": ticket.ticket_number, "new_status": ticket.status}

@router.post("/{ticket_id}/comments")
async def add_ticket_comment(ticket_id: str, req: CommentCreate, db: AsyncSession = Depends(get_db)):
    t_uuid = uuid.UUID(ticket_id)
    comm = TicketComment(
        ticket_id=t_uuid,
        author_id=uuid.UUID(req.author_id) if req.author_id else None,
        content=req.content,
        is_internal=req.is_internal
    )
    db.add(comm)
    await db.commit()
    await db.refresh(comm)
    return {"status": "success", "comment_id": str(comm.id)}
