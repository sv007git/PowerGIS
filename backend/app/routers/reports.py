from fastapi import APIRouter, Depends, HTTPException, Response
from typing import Optional, List, Dict
from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy import select, func
from datetime import datetime
import io
import csv

from backend.app.core.database import get_db
from backend.app.models.models import Substation, Feeder, Transformer, Ticket, Consumer

router = APIRouter(prefix="/reports", tags=["Reporting & Compliance Analytics"])

@router.get("/asset-inventory")
async def get_asset_inventory_report(db: AsyncSession = Depends(get_db)):
    sub_count = (await db.execute(select(func.count(Substation.id)))).scalar() or 0
    feeder_count = (await db.execute(select(func.count(Feeder.id)))).scalar() or 0
    dtr_count = (await db.execute(select(func.count(Transformer.id)))).scalar() or 0
    con_count = (await db.execute(select(func.count(Consumer.id)))).scalar() or 0
    total_capacity_mva = (await db.execute(select(func.sum(Substation.capacity_mva)))).scalar() or 0.0
    total_feeder_length = (await db.execute(select(func.sum(Feeder.length_km)))).scalar() or 0.0

    return {
        "report_date": datetime.utcnow().isoformat(),
        "summary": {
            "substations_count": sub_count,
            "feeders_count": feeder_count,
            "transformers_count": dtr_count,
            "consumers_count": con_count,
            "total_transformation_capacity_mva": float(total_capacity_mva),
            "total_distribution_network_length_km": float(total_feeder_length)
        }
    }

@router.get("/feeder-loading")
async def get_feeder_loading_report(db: AsyncSession = Depends(get_db)):
    feeders = (await db.execute(select(Feeder).order_by(Feeder.peak_load_amps.desc()))).scalars().all()
    results = []
    for f in feeders:
        current = float(f.peak_load_amps)
        # Assuming nominal safe rating of Rabbit ACSR ~200 Amps
        pct = round((current / 200.0) * 100, 1)
        loading_status = "CRITICAL" if pct >= 90 else ("HIGH" if pct >= 75 else "NORMAL")
        results.append({
            "code": f.code,
            "name": f.name,
            "voltage": f.voltage,
            "length_km": float(f.length_km),
            "peak_amps": current,
            "loading_pct": pct,
            "status": loading_status
        })
    return results

@router.get("/sla-compliance")
async def get_sla_compliance_report(db: AsyncSession = Depends(get_db)):
    total_tickets = (await db.execute(select(func.count(Ticket.id)))).scalar() or 0
    resolved_tickets = (await db.execute(select(func.count(Ticket.id)).where(Ticket.status.in_(["resolved", "verified", "closed"])))).scalar() or 0
    
    # Check breached: sla_due_at < resolved_at or (sla_due_at < now and status not resolved)
    now = datetime.utcnow()
    breached_tickets = (await db.execute(
        select(func.count(Ticket.id)).where(
            (Ticket.sla_due_at < now) & (~Ticket.status.in_(["resolved", "verified", "closed"]))
        )
    )).scalar() or 0

    compliance_pct = 100.0 if total_tickets == 0 else round(((total_tickets - breached_tickets) / total_tickets) * 100, 1)

    return {
        "total_tickets": total_tickets,
        "resolved_tickets": resolved_tickets,
        "breached_tickets": breached_tickets,
        "sla_compliance_percentage": compliance_pct
    }

@router.get("/export/csv")
async def export_assets_csv(entity: str = "transformers", db: AsyncSession = Depends(get_db)):
    output = io.StringIO()
    writer = csv.writer(output)

    if entity == "transformers":
        writer.writerow(["Code", "Name", "Capacity_kVA", "Load_Pct", "Status"])
        items = (await db.execute(select(Transformer))).scalars().all()
        for i in items:
            writer.writerow([i.code, i.name, i.capacity_kva, float(i.load_pct), i.status])
    elif entity == "feeders":
        writer.writerow(["Code", "Name", "Voltage", "Length_KM", "Peak_Load_Amps", "Status"])
        items = (await db.execute(select(Feeder))).scalars().all()
        for i in items:
            writer.writerow([i.code, i.name, i.voltage, float(i.length_km), float(i.peak_load_amps), i.status])
    else:
        writer.writerow(["Code", "Name", "Voltage", "Capacity_MVA", "Num_Feeders", "Status"])
        items = (await db.execute(select(Substation))).scalars().all()
        for i in items:
            writer.writerow([i.code, i.name, i.voltage, float(i.capacity_mva), i.num_feeders, i.status])

    return Response(
        content=output.getvalue(),
        media_type="text/csv",
        headers={"Content-Disposition": f"attachment; filename=powergis_{entity}_export.csv"}
    )
