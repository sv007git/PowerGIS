from fastapi import APIRouter, Depends, HTTPException, status
from pydantic import BaseModel
from typing import List, Optional, Dict, Any
from datetime import datetime, timedelta
from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy import select, func
import uuid

from backend.app.core.database import get_db
from backend.app.models.models import ScadaGateway, ScadaPoint, ScadaAlarm, Ticket, Substation, Feeder

router = APIRouter(prefix="/scada", tags=["SCADA & Power System Telemetry (Module 27)"])

class AlarmAckRequest(BaseModel):
    user_id: str

@router.get("/overview")
async def get_scada_overview(db: AsyncSession = Depends(get_db)):
    gateways = (await db.execute(select(ScadaGateway))).scalars().all()
    points = (await db.execute(select(ScadaPoint).where(ScadaPoint.is_active == True))).scalars().all()
    active_alarms = (await db.execute(select(ScadaAlarm).where(ScadaAlarm.status == "active"))).scalars().all()

    return {
        "system_status": "NORMAL" if len(active_alarms) == 0 else "WARNING",
        "gateways": [
            {
                "id": str(g.id),
                "name": g.name,
                "protocol": g.protocol,
                "ip_address": g.ip_address,
                "port": g.port,
                "status": g.status,
                "last_heartbeat": g.last_heartbeat
            }
            for g in gateways
        ],
        "active_alarms_count": len(active_alarms),
        "total_monitored_points": len(points),
        "grid_frequency_hz": 49.98,
        "grid_average_voltage_kv": 110.2
    }

@router.get("/points")
async def list_scada_points(substation_id: Optional[str] = None, db: AsyncSession = Depends(get_db)):
    query = select(ScadaPoint)
    if substation_id:
        query = query.where(ScadaPoint.substation_id == uuid.UUID(substation_id))
    query = query.order_by(ScadaPoint.tag)
    res = await db.execute(query)
    points = res.scalars().all()

    return [
        {
            "id": str(p.id),
            "tag": p.tag,
            "description": p.description,
            "point_type": p.point_type,
            "unit": p.unit,
            "nominal_value": float(p.nominal_value) if p.nominal_value else None,
            "current_value": float(p.current_value) if p.current_value else None,
            "quality": p.quality,
            "last_updated": p.last_updated
        }
        for p in points
    ]

@router.get("/sld/{substation_code}")
async def get_substation_sld(substation_code: str, db: AsyncSession = Depends(get_db)):
    """
    Returns topological Single Line Diagram components (Busbars, Incomers, Transformers, Outgoing Feeders)
    with live SCADA telemetry points for drawing the interactive SVG SLD.
    """
    sub_res = await db.execute(select(Substation).where(Substation.code == substation_code))
    sub = sub_res.scalar_one_or_none()
    if not sub:
        # Default fallback to 110kV Villianur
        sub_res = await db.execute(select(Substation).order_by(Substation.code))
        sub = sub_res.scalars().first()

    feeders_res = await db.execute(select(Feeder).where(Feeder.substation_id == sub.id))
    feeders = feeders_res.scalars().all()

    points_res = await db.execute(select(ScadaPoint).where(ScadaPoint.substation_id == sub.id))
    points = {p.tag: float(p.current_value) if p.current_value else 0.0 for p in points_res.scalars().all()}

    sld_data = {
        "substation": {
            "code": sub.code,
            "name": sub.name,
            "voltage": sub.voltage,
            "capacity_mva": float(sub.capacity_mva)
        },
        "busbar_110kv": {
            "id": "BUS-110KV-A",
            "voltage_kv": points.get("SS_VIL_110KV_BUS_V", 110.2),
            "frequency_hz": points.get("SS_VIL_GRID_FREQ", 49.98),
            "status": "energized"
        },
        "power_transformers": [
            {
                "id": "PTR-1",
                "capacity": "25 MVA",
                "ratio": "110kV / 11kV",
                "primary_voltage": 110.2,
                "secondary_voltage": 11.05,
                "current_amps": 48.5,
                "oil_temp_c": 54.2,
                "status": "normal"
            },
            {
                "id": "PTR-2",
                "capacity": "25 MVA",
                "ratio": "110kV / 11kV",
                "primary_voltage": 110.2,
                "secondary_voltage": 11.02,
                "current_amps": 42.1,
                "oil_temp_c": 51.0,
                "status": "normal"
            }
        ],
        "busbar_11kv": {
            "id": "BUS-11KV-MAIN",
            "voltage_kv": 11.04,
            "status": "energized"
        },
        "feeders": [
            {
                "id": str(f.id),
                "code": f.code,
                "name": f.name,
                "breaker_status": "TRIPPED" if "01" in f.code else "CLOSED",
                "current_amps": float(f.peak_load_amps),
                "active_power_mw": round(float(f.peak_load_amps) * 11.0 * 1.732 * 0.85 / 1000, 2),
                "status": f.status
            }
            for f in feeders
        ]
    }
    return sld_data

@router.get("/alarms")
async def list_alarms(status_filter: Optional[str] = None, db: AsyncSession = Depends(get_db)):
    query = select(ScadaAlarm).order_by(ScadaAlarm.occurred_at.desc())
    if status_filter:
        query = query.where(ScadaAlarm.status == status_filter)
    res = await db.execute(query)
    alarms = res.scalars().all()

    return [
        {
            "id": str(a.id),
            "alarm_code": a.alarm_code,
            "category": a.category,
            "severity": a.severity,
            "message": a.message,
            "status": a.status,
            "occurred_at": a.occurred_at,
            "ticket_id": str(a.ticket_id) if a.ticket_id else None
        }
        for a in alarms
    ]

@router.post("/alarms/{alarm_id}/acknowledge")
async def acknowledge_alarm(alarm_id: str, req: AlarmAckRequest, db: AsyncSession = Depends(get_db)):
    a_uuid = uuid.UUID(alarm_id)
    res = await db.execute(select(ScadaAlarm).where(ScadaAlarm.id == a_uuid))
    alarm = res.scalar_one_or_none()
    if not alarm:
        raise HTTPException(status_code=404, detail="Alarm not found")

    alarm.status = "acknowledged"
    alarm.acknowledged_by = uuid.UUID(req.user_id)
    alarm.acknowledged_at = datetime.utcnow()
    await db.commit()
    return {"status": "success", "alarm_code": alarm.alarm_code, "new_status": "acknowledged"}

@router.post("/alarms/{alarm_id}/create-ticket")
async def auto_create_ticket_from_alarm(alarm_id: str, db: AsyncSession = Depends(get_db)):
    a_uuid = uuid.UUID(alarm_id)
    res = await db.execute(select(ScadaAlarm).where(ScadaAlarm.id == a_uuid))
    alarm = res.scalar_one_or_none()
    if not alarm:
        raise HTTPException(status_code=404, detail="Alarm not found")

    if alarm.ticket_id:
        return {"status": "already_linked", "ticket_id": str(alarm.ticket_id)}

    # Auto generate ticket
    count = (await db.execute(select(func.count(Ticket.id)))).scalar() or 0
    tkt_num = f"TKT-2026-{(count + 1):04d}"

    ticket = Ticket(
        ticket_number=tkt_num,
        category="SCADA Grid Alarm",
        subject=f"Automated Trip Action: {alarm.alarm_code}",
        description=f"Auto-generated dispatch from SCADA gateway event:\n{alarm.message}",
        priority=alarm.severity if alarm.severity in ["critical", "high"] else "high",
        status="open",
        sla_due_at=datetime.utcnow() + timedelta(hours=4)
    )
    db.add(ticket)
    await db.flush()

    alarm.ticket_id = ticket.id
    await db.commit()
    return {"status": "success", "ticket_number": ticket.ticket_number, "ticket_id": str(ticket.id)}
