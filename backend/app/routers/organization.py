from fastapi import APIRouter, Depends, HTTPException, status
from pydantic import BaseModel
from typing import List, Optional
from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy import select
from geoalchemy2.functions import ST_AsGeoJSON
import json
import uuid

from backend.app.core.database import get_db
from backend.app.models.models import OrgUnit

router = APIRouter(prefix="/organization", tags=["Organization Hierarchy"])

class OrgUnitOut(BaseModel):
    id: str
    code: str
    name: str
    level: str
    parent_id: Optional[str] = None
    headquarter: Optional[str] = None
    contact_email: Optional[str] = None
    contact_phone: Optional[str] = None
    is_active: bool

    class Config:
        from_attributes = True

class OrgUnitCreate(BaseModel):
    code: str
    name: str
    level: str
    parent_id: Optional[str] = None
    headquarter: Optional[str] = None
    contact_email: Optional[str] = None
    contact_phone: Optional[str] = None

@router.get("/units", response_model=List[OrgUnitOut])
async def list_org_units(
    level: Optional[str] = None,
    parent_id: Optional[str] = None,
    db: AsyncSession = Depends(get_db)
):
    query = select(OrgUnit).where(OrgUnit.is_active == True)
    if level:
        query = query.where(OrgUnit.level == level)
    if parent_id:
        query = query.where(OrgUnit.parent_id == uuid.UUID(parent_id))
    query = query.order_by(OrgUnit.code)
    res = await db.execute(query)
    units = res.scalars().all()
    return [
        OrgUnitOut(
            id=str(u.id),
            code=u.code,
            name=u.name,
            level=u.level,
            parent_id=str(u.parent_id) if u.parent_id else None,
            headquarter=u.headquarter,
            contact_email=u.contact_email,
            contact_phone=u.contact_phone,
            is_active=u.is_active
        )
        for u in units
    ]

@router.get("/tree")
async def get_org_tree(db: AsyncSession = Depends(get_db)):
    query = select(OrgUnit).where(OrgUnit.is_active == True).order_by(OrgUnit.level, OrgUnit.name)
    res = await db.execute(query)
    units = res.scalars().all()
    
    # Map into hierarchical tree
    nodes = {
        str(u.id): {
            "id": str(u.id),
            "code": u.code,
            "name": u.name,
            "level": u.level,
            "headquarter": u.headquarter,
            "children": []
        }
        for u in units
    }
    
    root_nodes = []
    for u in units:
        node = nodes[str(u.id)]
        if u.parent_id and str(u.parent_id) in nodes:
            nodes[str(u.parent_id)]["children"].append(node)
        else:
            root_nodes.append(node)
            
    return {"tree": root_nodes, "total_units": len(units)}

@router.post("/units", response_model=OrgUnitOut)
async def create_org_unit(req: OrgUnitCreate, db: AsyncSession = Depends(get_db)):
    parent_uuid = uuid.UUID(req.parent_id) if req.parent_id else None
    unit = OrgUnit(
        code=req.code,
        name=req.name,
        level=req.level,
        parent_id=parent_uuid,
        headquarter=req.headquarter,
        contact_email=req.contact_email,
        contact_phone=req.contact_phone,
        is_active=True
    )
    db.add(unit)
    await db.commit()
    await db.refresh(unit)
    return OrgUnitOut(
        id=str(unit.id),
        code=unit.code,
        name=unit.name,
        level=unit.level,
        parent_id=str(unit.parent_id) if unit.parent_id else None,
        headquarter=unit.headquarter,
        contact_email=unit.contact_email,
        contact_phone=unit.contact_phone,
        is_active=unit.is_active
    )
