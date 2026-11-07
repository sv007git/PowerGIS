from fastapi import APIRouter, Depends, HTTPException, status
from pydantic import BaseModel, EmailStr
from typing import List, Optional
from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy import select
import uuid

from backend.app.core.database import get_db
from backend.app.core.security import get_password_hash
from backend.app.models.models import User, Role, OrgUnit

router = APIRouter(prefix="/users", tags=["Users & Access Control"])

class RoleOut(BaseModel):
    role: str
    name: str
    description: Optional[str] = None
    hierarchy_order: int
    can_view: bool
    can_create: bool
    can_edit: bool
    can_delete: bool
    can_assign: bool
    can_approve: bool
    can_reject: bool
    can_export: bool
    can_import: bool
    can_manage_users: bool
    can_manage_gis_layers: bool
    can_access_reports: bool
    can_manage_scada: bool

    class Config:
        from_attributes = True

class UserOut(BaseModel):
    id: str
    username: str
    email: str
    full_name: str
    phone: Optional[str] = None
    designation: Optional[str] = None
    role: str
    org_unit_id: Optional[str] = None
    org_unit_name: Optional[str] = None
    is_active: bool

    class Config:
        from_attributes = True

class UserCreate(BaseModel):
    username: str
    email: EmailStr
    password: str
    full_name: str
    role: str
    phone: Optional[str] = None
    designation: Optional[str] = None
    org_unit_id: Optional[str] = None

@router.get("/roles", response_model=List[RoleOut])
async def list_roles(db: AsyncSession = Depends(get_db)):
    query = select(Role).order_by(Role.hierarchy_order)
    res = await db.execute(query)
    return res.scalars().all()

@router.get("", response_model=List[UserOut])
async def list_users(
    role: Optional[str] = None,
    org_unit_id: Optional[str] = None,
    db: AsyncSession = Depends(get_db)
):
    query = select(User, OrgUnit.name.label("org_name")).outerjoin(OrgUnit, User.org_unit_id == OrgUnit.id)
    if role:
        query = query.where(User.role == role)
    if org_unit_id:
        query = query.where(User.org_unit_id == uuid.UUID(org_unit_id))
    query = query.order_by(User.full_name)
    res = await db.execute(query)
    rows = res.all()
    
    users_out = []
    for user, org_name in rows:
        users_out.append(
            UserOut(
                id=str(user.id),
                username=user.username,
                email=user.email,
                full_name=user.full_name,
                phone=user.phone,
                designation=user.designation,
                role=user.role,
                org_unit_id=str(user.org_unit_id) if user.org_unit_id else None,
                org_unit_name=org_name,
                is_active=user.is_active
            )
        )
    return users_out

@router.post("", response_model=UserOut)
async def create_user(req: UserCreate, db: AsyncSession = Depends(get_db)):
    # Check if username or email already exists
    dup_query = select(User).where((User.username == req.username) | (User.email == req.email))
    dup_res = await db.execute(dup_query)
    if dup_res.scalar_one_or_none():
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="A user with this username or email already exists"
        )

    user = User(
        username=req.username,
        email=req.email,
        password_hash=get_password_hash(req.password),
        full_name=req.full_name,
        role=req.role,
        phone=req.phone,
        designation=req.designation,
        org_unit_id=uuid.UUID(req.org_unit_id) if req.org_unit_id else None,
        is_active=True
    )
    db.add(user)
    await db.commit()
    await db.refresh(user)

    return UserOut(
        id=str(user.id),
        username=user.username,
        email=user.email,
        full_name=user.full_name,
        phone=user.phone,
        designation=user.designation,
        role=user.role,
        org_unit_id=str(user.org_unit_id) if user.org_unit_id else None,
        org_unit_name=None,
        is_active=user.is_active
    )

@router.put("/{user_id}/status")
async def toggle_user_status(user_id: str, is_active: bool, db: AsyncSession = Depends(get_db)):
    u_uuid = uuid.UUID(user_id)
    query = select(User).where(User.id == u_uuid)
    res = await db.execute(query)
    user = res.scalar_one_or_none()
    if not user:
        raise HTTPException(status_code=404, detail="User not found")
    user.is_active = is_active
    await db.commit()
    return {"status": "success", "user_id": user_id, "is_active": is_active}
