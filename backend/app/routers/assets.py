from fastapi import APIRouter, Depends, HTTPException, status
from pydantic import BaseModel
from typing import List, Optional, Any, Dict
from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy import select, func
from geoalchemy2.functions import ST_AsGeoJSON, ST_GeomFromGeoJSON
import json
import uuid

from backend.app.core.database import get_db
from backend.app.models.models import Substation, Feeder, Transformer, Pole, Line, Consumer

router = APIRouter(prefix="/assets", tags=["Electrical Assets Management"])

# --- Summary ---
@router.get("/summary")
async def get_assets_summary(db: AsyncSession = Depends(get_db)):
    sub_count = (await db.execute(select(func.count(Substation.id)))).scalar() or 0
    feeder_count = (await db.execute(select(func.count(Feeder.id)))).scalar() or 0
    dtr_count = (await db.execute(select(func.count(Transformer.id)))).scalar() or 0
    pole_count = (await db.execute(select(func.count(Pole.id)))).scalar() or 0
    line_count = (await db.execute(select(func.count(Line.id)))).scalar() or 0
    consumer_count = (await db.execute(select(func.count(Consumer.id)))).scalar() or 0

    return {
        "substations": sub_count,
        "feeders": feeder_count,
        "transformers": dtr_count,
        "poles": pole_count,
        "lines": line_count,
        "consumers": consumer_count,
        "total_assets": sub_count + feeder_count + dtr_count + pole_count + line_count + consumer_count
    }

# --- Substations ---
@router.get("/substations")
async def list_substations(voltage: Optional[str] = None, db: AsyncSession = Depends(get_db)):
    query = select(Substation, ST_AsGeoJSON(Substation.geom).label("geojson"))
    if voltage:
        query = query.where(Substation.voltage == voltage)
    query = query.order_by(Substation.code)
    res = await db.execute(query)
    
    items = []
    for sub, geo_str in res.all():
        items.append({
            "id": str(sub.id),
            "code": sub.code,
            "name": sub.name,
            "voltage": sub.voltage,
            "capacity_mva": float(sub.capacity_mva),
            "num_feeders": sub.num_feeders,
            "org_unit_id": str(sub.org_unit_id) if sub.org_unit_id else None,
            "status": sub.status,
            "geometry": json.loads(geo_str) if geo_str else None
        })
    return items

# --- Feeders ---
@router.get("/feeders")
async def list_feeders(substation_id: Optional[str] = None, db: AsyncSession = Depends(get_db)):
    query = select(Feeder, ST_AsGeoJSON(Feeder.geom).label("geojson"))
    if substation_id:
        query = query.where(Feeder.substation_id == uuid.UUID(substation_id))
    query = query.order_by(Feeder.code)
    res = await db.execute(query)
    
    items = []
    for feeder, geo_str in res.all():
        items.append({
            "id": str(feeder.id),
            "code": feeder.code,
            "name": feeder.name,
            "substation_id": str(feeder.substation_id),
            "voltage": feeder.voltage,
            "length_km": float(feeder.length_km),
            "peak_load_amps": float(feeder.peak_load_amps),
            "conductor_type": feeder.conductor_type,
            "status": feeder.status,
            "geometry": json.loads(geo_str) if geo_str else None
        })
    return items

# --- Transformers (DTRs) ---
@router.get("/transformers")
async def list_transformers(feeder_id: Optional[str] = None, db: AsyncSession = Depends(get_db)):
    query = select(Transformer, ST_AsGeoJSON(Transformer.geom).label("geojson"))
    if feeder_id:
        query = query.where(Transformer.feeder_id == uuid.UUID(feeder_id))
    query = query.order_by(Transformer.code)
    res = await db.execute(query)
    
    items = []
    for dtr, geo_str in res.all():
        items.append({
            "id": str(dtr.id),
            "code": dtr.code,
            "name": dtr.name,
            "feeder_id": str(dtr.feeder_id) if dtr.feeder_id else None,
            "substation_id": str(dtr.substation_id) if dtr.substation_id else None,
            "capacity_kva": dtr.capacity_kva,
            "make": dtr.make,
            "serial_number": dtr.serial_number,
            "load_pct": float(dtr.load_pct),
            "status": dtr.status,
            "geometry": json.loads(geo_str) if geo_str else None
        })
    return items

# --- Poles ---
@router.get("/poles")
async def list_poles(feeder_id: Optional[str] = None, db: AsyncSession = Depends(get_db)):
    query = select(Pole, ST_AsGeoJSON(Pole.geom).label("geojson"))
    if feeder_id:
        query = query.where(Pole.feeder_id == uuid.UUID(feeder_id))
    query = query.order_by(Pole.code)
    res = await db.execute(query)
    
    items = []
    for pole, geo_str in res.all():
        items.append({
            "id": str(pole.id),
            "code": pole.code,
            "pole_number": pole.pole_number,
            "pole_type": pole.pole_type,
            "height_meters": float(pole.height_meters),
            "feeder_id": str(pole.feeder_id) if pole.feeder_id else None,
            "status": pole.status,
            "geometry": json.loads(geo_str) if geo_str else None
        })
    return items

# --- Consumers ---
@router.get("/consumers")
async def list_consumers(transformer_id: Optional[str] = None, db: AsyncSession = Depends(get_db)):
    query = select(Consumer, ST_AsGeoJSON(Consumer.geom).label("geojson"))
    if transformer_id:
        query = query.where(Consumer.transformer_id == uuid.UUID(transformer_id))
    query = query.order_by(Consumer.consumer_number)
    res = await db.execute(query)
    
    items = []
    for con, geo_str in res.all():
        items.append({
            "id": str(con.id),
            "consumer_number": con.consumer_number,
            "name": con.name,
            "category": con.category,
            "sanctioned_load_kw": float(con.sanctioned_load_kw),
            "meter_number": con.meter_number,
            "transformer_id": str(con.transformer_id) if con.transformer_id else None,
            "address": con.address,
            "contact_phone": con.contact_phone,
            "status": con.status,
            "geometry": json.loads(geo_str) if geo_str else None
        })
    return items
