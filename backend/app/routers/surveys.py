from fastapi import APIRouter, Depends, HTTPException, status
from pydantic import BaseModel
from typing import List, Optional, Dict, Any
from datetime import datetime
from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy import select, func
from geoalchemy2.functions import ST_GeomFromGeoJSON, ST_AsGeoJSON
import json
import uuid

from backend.app.core.database import get_db
from backend.app.models.models import SurveySubmission, SurveyPhoto, User

router = APIRouter(prefix="/surveys", tags=["Field Surveys & Mobile Inspection"])

class PhotoCreate(BaseModel):
    photo_type: str = "asset_overview"
    file_url: str
    caption: Optional[str] = None
    latitude: Optional[float] = None
    longitude: Optional[float] = None

class SurveyCreate(BaseModel):
    surveyor_id: str
    asset_type: str
    asset_code: Optional[str] = None
    latitude: float
    longitude: float
    gps_accuracy_meters: Optional[float] = 3.5
    attributes: Dict[str, Any] = {}
    photos: List[PhotoCreate] = []

@router.get("")
async def list_surveys(db: AsyncSession = Depends(get_db)):
    query = (
        select(SurveySubmission, User.full_name, ST_AsGeoJSON(SurveySubmission.geom).label("geom_json"))
        .outerjoin(User, SurveySubmission.surveyor_id == User.id)
        .order_by(SurveySubmission.created_at.desc())
    )
    res = await db.execute(query)
    
    surveys_out = []
    for s, surveyor_name, g_json in res.all():
        surveys_out.append({
            "id": str(s.id),
            "survey_code": s.survey_code,
            "asset_type": s.asset_type,
            "asset_code": s.asset_code,
            "surveyor_name": surveyor_name,
            "gps_accuracy_meters": float(s.gps_accuracy_meters) if s.gps_accuracy_meters else None,
            "attributes": s.attributes,
            "is_approved": s.is_approved,
            "sync_status": s.sync_status,
            "geometry": json.loads(g_json) if g_json else None,
            "created_at": s.created_at
        })
    return surveys_out

@router.post("")
async def submit_survey(req: SurveyCreate, db: AsyncSession = Depends(get_db)):
    count = (await db.execute(select(func.count(SurveySubmission.id)))).scalar() or 0
    code = f"SRV-2026-{(count + 1):04d}"
    
    geom_json = json.dumps({"type": "Point", "coordinates": [req.longitude, req.latitude]})
    geom = ST_GeomFromGeoJSON(geom_json)

    sub = SurveySubmission(
        survey_code=code,
        surveyor_id=uuid.UUID(req.surveyor_id),
        asset_type=req.asset_type,
        asset_code=req.asset_code,
        gps_accuracy_meters=req.gps_accuracy_meters,
        attributes=req.attributes,
        is_approved=None,
        geom=geom,
        sync_status="synced"
    )
    db.add(sub)
    await db.flush()

    for p in req.photos:
        photo = SurveyPhoto(
            submission_id=sub.id,
            photo_type=p.photo_type,
            file_url=p.file_url,
            caption=p.caption,
            latitude=p.latitude or req.latitude,
            longitude=p.longitude or req.longitude
        )
        db.add(photo)

    await db.commit()
    return {"status": "success", "survey_code": code, "submission_id": str(sub.id)}

@router.post("/{survey_id}/review")
async def review_survey(survey_id: str, approved: bool, db: AsyncSession = Depends(get_db)):
    s_uuid = uuid.UUID(survey_id)
    res = await db.execute(select(SurveySubmission).where(SurveySubmission.id == s_uuid))
    survey = res.scalar_one_or_none()
    if not survey:
        raise HTTPException(status_code=404, detail="Survey submission not found")
        
    survey.is_approved = approved
    await db.commit()
    return {"status": "success", "is_approved": approved}
