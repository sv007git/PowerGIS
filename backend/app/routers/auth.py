from fastapi import APIRouter, Depends, HTTPException, status
from pydantic import BaseModel, EmailStr
from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy import select
from typing import Optional
from backend.app.core.database import get_db
from backend.app.core.security import verify_password, create_access_token
from backend.app.models.models import User, Role

router = APIRouter(prefix="/auth", tags=["Authentication"])

class LoginRequest(BaseModel):
    username_or_email: str
    password: str

class TokenResponse(BaseModel):
    access_token: str
    token_type: str = "bearer"
    user_id: str
    username: str
    full_name: str
    role: str
    org_unit_id: Optional[str] = None
    permissions: dict

class ForgotPasswordRequest(BaseModel):
    email: EmailStr

@router.post("/login", response_model=TokenResponse)
async def login(req: LoginRequest, db: AsyncSession = Depends(get_db)):
    # Search by username or email
    query = select(User).where(
        (User.username == req.username_or_email) | (User.email == req.username_or_email)
    )
    result = await db.execute(query)
    user = result.scalar_one_or_none()

    if not user or not verify_password(req.password, user.password_hash):
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Invalid username/email or password"
        )
    
    if not user.is_active:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="Account has been deactivated. Please contact your State Administrator."
        )

    # Fetch role permissions
    role_query = select(Role).where(Role.role == user.role)
    role_res = await db.execute(role_query)
    role_obj = role_res.scalar_one_or_none()
    
    perms = {}
    if role_obj:
        perms = {
            "can_view": role_obj.can_view,
            "can_create": role_obj.can_create,
            "can_edit": role_obj.can_edit,
            "can_delete": role_obj.can_delete,
            "can_assign": role_obj.can_assign,
            "can_approve": role_obj.can_approve,
            "can_reject": role_obj.can_reject,
            "can_export": role_obj.can_export,
            "can_import": role_obj.can_import,
            "can_manage_users": role_obj.can_manage_users,
            "can_manage_gis_layers": role_obj.can_manage_gis_layers,
            "can_access_reports": role_obj.can_access_reports,
            "can_manage_scada": role_obj.can_manage_scada,
        }

    token = create_access_token(
        subject=str(user.id),
        role=user.role,
        org_unit_id=str(user.org_unit_id) if user.org_unit_id else None
    )

    return TokenResponse(
        access_token=token,
        user_id=str(user.id),
        username=user.username,
        full_name=user.full_name,
        role=user.role,
        org_unit_id=str(user.org_unit_id) if user.org_unit_id else None,
        permissions=perms
    )

@router.post("/forgot-password")
async def forgot_password(req: ForgotPasswordRequest, db: AsyncSession = Depends(get_db)):
    # Standard security practice: return success whether or not the email exists
    return {"message": "If this email is registered, password recovery instructions have been sent."}
