from fastapi import APIRouter, Depends, HTTPException, Query
from typing import Optional, List, Dict, Any
from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy import select, or_
from geoalchemy2.functions import ST_AsGeoJSON, ST_DWithin, ST_MakePoint
import json
import uuid

from backend.app.core.database import get_db
from backend.app.models.models import Substation, Feeder, Transformer, Pole, Line, Consumer, OrgUnit

router = APIRouter(prefix="/gis", tags=["GIS Engine & Spatial Analysis"])

@router.get("/layers")
async def get_available_layers():
    return [
        {
            "id": "substations",
            "name": "Substations (EHV/HV)",
            "type": "point",
            "color": "#e53e3e",
            "icon": "zap",
            "visible": True,
            "opacity": 1.0
        },
        {
            "id": "feeders",
            "name": "11kV / 22kV Feeders",
            "type": "linestring",
            "color": "#3182ce",
            "icon": "git-commit",
            "visible": True,
            "opacity": 0.9
        },
        {
            "id": "transformers",
            "name": "Distribution Transformers (DTR)",
            "type": "point",
            "color": "#d69e2e",
            "icon": "box",
            "visible": True,
            "opacity": 1.0
        },
        {
            "id": "lines",
            "name": "LT / HT Lines",
            "type": "linestring",
            "color": "#4a5568",
            "icon": "activity",
            "visible": False,
            "opacity": 0.8
        },
        {
            "id": "poles",
            "name": "Utility Poles",
            "type": "point",
            "color": "#718096",
            "icon": "map-pin",
            "visible": False,
            "opacity": 0.8
        },
        {
            "id": "consumers",
            "name": "Consumers (Service Connections)",
            "type": "point",
            "color": "#38a169",
            "icon": "user",
            "visible": False,
            "opacity": 1.0
        },
        {
            "id": "jurisdictions",
            "name": "Organizational Boundaries",
            "type": "polygon",
            "color": "#805ad5",
            "icon": "shield",
            "visible": True,
            "opacity": 0.3
        }
    ]

@router.get("/geojson/{layer_name}")
async def get_layer_geojson(layer_name: str, db: AsyncSession = Depends(get_db)):
    features = []
    
    if layer_name == "substations":
        query = select(Substation, ST_AsGeoJSON(Substation.geom).label("geom_json"))
        res = await db.execute(query)
        for s, g_str in res.all():
            features.append({
                "type": "Feature",
                "id": str(s.id),
                "geometry": json.loads(g_str) if g_str else None,
                "properties": {
                    "code": s.code,
                    "name": s.name,
                    "voltage": s.voltage,
                    "capacity_mva": float(s.capacity_mva),
                    "num_feeders": s.num_feeders,
                    "status": s.status,
                    "layer": "substations"
                }
            })
            
    elif layer_name == "feeders":
        query = select(Feeder, ST_AsGeoJSON(Feeder.geom).label("geom_json"))
        res = await db.execute(query)
        for f, g_str in res.all():
            features.append({
                "type": "Feature",
                "id": str(f.id),
                "geometry": json.loads(g_str) if g_str else None,
                "properties": {
                    "code": f.code,
                    "name": f.name,
                    "voltage": f.voltage,
                    "length_km": float(f.length_km),
                    "peak_load_amps": float(f.peak_load_amps),
                    "conductor": f.conductor_type,
                    "status": f.status,
                    "substation_id": str(f.substation_id),
                    "layer": "feeders"
                }
            })
            
    elif layer_name == "transformers":
        query = select(Transformer, ST_AsGeoJSON(Transformer.geom).label("geom_json"))
        res = await db.execute(query)
        for t, g_str in res.all():
            features.append({
                "type": "Feature",
                "id": str(t.id),
                "geometry": json.loads(g_str) if g_str else None,
                "properties": {
                    "code": t.code,
                    "name": t.name,
                    "capacity_kva": t.capacity_kva,
                    "load_pct": float(t.load_pct),
                    "feeder_id": str(t.feeder_id) if t.feeder_id else None,
                    "status": t.status,
                    "layer": "transformers"
                }
            })

    elif layer_name == "poles":
        query = select(Pole, ST_AsGeoJSON(Pole.geom).label("geom_json"))
        res = await db.execute(query)
        for p, g_str in res.all():
            features.append({
                "type": "Feature",
                "id": str(p.id),
                "geometry": json.loads(g_str) if g_str else None,
                "properties": {
                    "code": p.code,
                    "pole_number": p.pole_number,
                    "pole_type": p.pole_type,
                    "status": p.status,
                    "layer": "poles"
                }
            })

    elif layer_name == "lines":
        query = select(Line, ST_AsGeoJSON(Line.geom).label("geom_json"))
        res = await db.execute(query)
        for l, g_str in res.all():
            features.append({
                "type": "Feature",
                "id": str(l.id),
                "geometry": json.loads(g_str) if g_str else None,
                "properties": {
                    "code": l.code,
                    "voltage": l.voltage,
                    "length_meters": float(l.length_meters),
                    "feeder_id": str(l.feeder_id),
                    "layer": "lines"
                }
            })

    elif layer_name == "consumers":
        query = select(Consumer, ST_AsGeoJSON(Consumer.geom).label("geom_json"))
        res = await db.execute(query)
        for c, g_str in res.all():
            features.append({
                "type": "Feature",
                "id": str(c.id),
                "geometry": json.loads(g_str) if g_str else None,
                "properties": {
                    "consumer_number": c.consumer_number,
                    "name": c.name,
                    "category": c.category,
                    "sanctioned_load_kw": float(c.sanctioned_load_kw),
                    "meter_number": c.meter_number,
                    "transformer_id": str(c.transformer_id) if c.transformer_id else None,
                    "layer": "consumers"
                }
            })

    elif layer_name == "jurisdictions":
        query = select(OrgUnit, ST_AsGeoJSON(OrgUnit.jurisdiction_geom).label("geom_json")).where(OrgUnit.jurisdiction_geom.isnot(None))
        res = await db.execute(query)
        for u, g_str in res.all():
            features.append({
                "type": "Feature",
                "id": str(u.id),
                "geometry": json.loads(g_str) if g_str else None,
                "properties": {
                    "code": u.code,
                    "name": u.name,
                    "level": u.level,
                    "layer": "jurisdictions"
                }
            })
    else:
        raise HTTPException(status_code=404, detail=f"Layer '{layer_name}' not found")

    return {
        "type": "FeatureCollection",
        "name": layer_name,
        "features": features
    }

# --- Network Tracing ---
@router.get("/trace")
async def trace_network(
    feeder_id: Optional[str] = None,
    transformer_id: Optional[str] = None,
    direction: str = Query(default="downstream", regex="^(upstream|downstream)$"),
    db: AsyncSession = Depends(get_db)
):
    """
    Performs topological network tracing:
    - Downstream from feeder: returns parent substation, all connected DTRs, LT lines, and consumers.
    - Upstream from transformer: returns feeding 11kV feeder and source grid substation.
    """
    if not feeder_id and not transformer_id:
        raise HTTPException(status_code=400, detail="Must provide feeder_id or transformer_id")

    trace_results: Dict[str, Any] = {
        "direction": direction,
        "substation": None,
        "feeder": None,
        "transformers": [],
        "consumers": [],
        "metrics": {
            "total_dtrs": 0,
            "total_consumers": 0,
            "total_connected_load_kw": 0.0,
            "total_line_length_km": 0.0
        }
    }

    target_feeder_id = feeder_id
    if transformer_id:
        t_uuid = uuid.UUID(transformer_id)
        dtr_res = await db.execute(select(Transformer).where(Transformer.id == t_uuid))
        dtr = dtr_res.scalar_one_or_none()
        if dtr and dtr.feeder_id:
            target_feeder_id = str(dtr.feeder_id)

    if target_feeder_id:
        f_uuid = uuid.UUID(target_feeder_id)
        f_res = await db.execute(select(Feeder).where(Feeder.id == f_uuid))
        f_obj = f_res.scalar_one_or_none()
        if f_obj:
            trace_results["feeder"] = {
                "id": str(f_obj.id),
                "code": f_obj.code,
                "name": f_obj.name,
                "voltage": f_obj.voltage,
                "length_km": float(f_obj.length_km),
                "peak_load_amps": float(f_obj.peak_load_amps)
            }
            trace_results["metrics"]["total_line_length_km"] = float(f_obj.length_km)

            # Upstream substation
            sub_res = await db.execute(select(Substation).where(Substation.id == f_obj.substation_id))
            sub_obj = sub_res.scalar_one_or_none()
            if sub_obj:
                trace_results["substation"] = {
                    "id": str(sub_obj.id),
                    "code": sub_obj.code,
                    "name": sub_obj.name,
                    "voltage": sub_obj.voltage,
                    "capacity_mva": float(sub_obj.capacity_mva)
                }

            # Downstream DTRs
            dtrs_res = await db.execute(select(Transformer).where(Transformer.feeder_id == f_uuid))
            dtrs = dtrs_res.scalars().all()
            trace_results["metrics"]["total_dtrs"] = len(dtrs)
            trace_results["transformers"] = [
                {
                    "id": str(t.id),
                    "code": t.code,
                    "name": t.name,
                    "capacity_kva": t.capacity_kva,
                    "load_pct": float(t.load_pct)
                }
                for t in dtrs
            ]

            # Downstream Consumers
            cons_res = await db.execute(select(Consumer).where(Consumer.feeder_id == f_uuid))
            cons = cons_res.scalars().all()
            trace_results["metrics"]["total_consumers"] = len(cons)
            total_load = sum(float(c.sanctioned_load_kw) for c in cons)
            trace_results["metrics"]["total_connected_load_kw"] = round(total_load, 2)
            trace_results["consumers"] = [
                {
                    "id": str(c.id),
                    "consumer_number": c.consumer_number,
                    "name": c.name,
                    "category": c.category,
                    "sanctioned_load_kw": float(c.sanctioned_load_kw)
                }
                for c in cons[:100]  # Cap at 100 for response size
            ]

    return trace_results

# --- Global Search ---
@router.get("/search")
async def search_assets(q: str = Query(..., min_length=2), db: AsyncSession = Depends(get_db)):
    search_term = f"%{q}%"
    results = []

    # 1. Substations
    subs = (await db.execute(
        select(Substation).where(or_(Substation.code.ilike(search_term), Substation.name.ilike(search_term)))
    )).scalars().all()
    for s in subs:
        results.append({"type": "substation", "id": str(s.id), "code": s.code, "label": f"Substation: {s.name} ({s.code})"})

    # 2. Feeders
    feeders = (await db.execute(
        select(Feeder).where(or_(Feeder.code.ilike(search_term), Feeder.name.ilike(search_term)))
    )).scalars().all()
    for f in feeders:
        results.append({"type": "feeder", "id": str(f.id), "code": f.code, "label": f"Feeder: {f.name} ({f.code})"})

    # 3. Transformers
    dtrs = (await db.execute(
        select(Transformer).where(or_(Transformer.code.ilike(search_term), Transformer.name.ilike(search_term)))
    )).scalars().all()
    for t in dtrs:
        results.append({"type": "transformer", "id": str(t.id), "code": t.code, "label": f"DTR: {t.name} ({t.code})"})

    # 4. Consumers
    cons = (await db.execute(
        select(Consumer).where(or_(Consumer.consumer_number.ilike(search_term), Consumer.name.ilike(search_term)))
    )).scalars().all()
    for c in cons:
        results.append({"type": "consumer", "id": str(c.id), "code": c.consumer_number, "label": f"Consumer: {c.name} ({c.consumer_number})"})

    return results
