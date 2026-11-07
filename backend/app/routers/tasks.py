from fastapi import APIRouter, Depends, HTTPException, status
from pydantic import BaseModel
from typing import List, Optional, Dict
from datetime import datetime, date
from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy import select, func
import uuid

from backend.app.core.database import get_db
from backend.app.models.models import Task, User

router = APIRouter(prefix="/tasks", tags=["Tasks & Work Orders Management"])

class TaskOut(BaseModel):
    id: str
    task_code: str
    ticket_id: Optional[str]
    title: str
    instructions: Optional[str]
    priority: str
    status: str
    assigned_to_user: Optional[str]
    assigned_to_name: Optional[str]
    assigned_team: Optional[str]
    contractor_name: Optional[str]
    due_date: Optional[date]
    created_at: datetime

    class Config:
        from_attributes = True

class TaskCreate(BaseModel):
    title: str
    instructions: Optional[str] = None
    priority: str = "medium"
    ticket_id: Optional[str] = None
    assigned_to_user: Optional[str] = None
    assigned_team: Optional[str] = None
    contractor_name: Optional[str] = None
    due_date: Optional[date] = None

class TaskStatusUpdate(BaseModel):
    status: str
    completion_notes: Optional[str] = None

@router.get("", response_model=List[TaskOut])
async def list_tasks(
    status_filter: Optional[str] = None,
    assigned_to: Optional[str] = None,
    db: AsyncSession = Depends(get_db)
):
    query = select(Task, User.full_name.label("user_name")).outerjoin(User, Task.assigned_to_user == User.id)
    if status_filter:
        query = query.where(Task.status == status_filter)
    if assigned_to:
        query = query.where(Task.assigned_to_user == uuid.UUID(assigned_to))
    query = query.order_by(Task.created_at.desc())
    res = await db.execute(query)
    
    tasks_out = []
    for t, u_name in res.all():
        tasks_out.append(TaskOut(
            id=str(t.id),
            task_code=t.task_code,
            ticket_id=str(t.ticket_id) if t.ticket_id else None,
            title=t.title,
            instructions=t.instructions,
            priority=t.priority,
            status=t.status,
            assigned_to_user=str(t.assigned_to_user) if t.assigned_to_user else None,
            assigned_to_name=u_name,
            assigned_team=t.assigned_team,
            contractor_name=t.contractor_name,
            due_date=t.due_date,
            created_at=t.created_at
        ))
    return tasks_out

@router.get("/kanban")
async def get_kanban_board(db: AsyncSession = Depends(get_db)):
    columns = {
        "created": [],
        "assigned": [],
        "in_progress": [],
        "completed": [],
        "verified": []
    }
    
    query = select(Task, User.full_name.label("user_name")).outerjoin(User, Task.assigned_to_user == User.id).order_by(Task.created_at.desc())
    res = await db.execute(query)
    for t, u_name in res.all():
        col = t.status if t.status in columns else "created"
        columns[col].append({
            "id": str(t.id),
            "task_code": t.task_code,
            "title": t.title,
            "priority": t.priority,
            "assigned_to": u_name or t.assigned_team or "Unassigned",
            "due_date": str(t.due_date) if t.due_date else None
        })
        
    return columns

@router.post("", response_model=TaskOut)
async def create_task(req: TaskCreate, db: AsyncSession = Depends(get_db)):
    count = (await db.execute(select(func.count(Task.id)))).scalar() or 0
    code = f"TSK-2026-{(count + 1):04d}"

    task = Task(
        task_code=code,
        title=req.title,
        instructions=req.instructions,
        priority=req.priority,
        status="created",
        ticket_id=uuid.UUID(req.ticket_id) if req.ticket_id else None,
        assigned_to_user=uuid.UUID(req.assigned_to_user) if req.assigned_to_user else None,
        assigned_team=req.assigned_team,
        contractor_name=req.contractor_name,
        due_date=req.due_date
    )
    db.add(task)
    await db.commit()
    await db.refresh(task)

    return TaskOut(
        id=str(task.id),
        task_code=task.task_code,
        ticket_id=str(task.ticket_id) if task.ticket_id else None,
        title=task.title,
        instructions=task.instructions,
        priority=task.priority,
        status=task.status,
        assigned_to_user=str(task.assigned_to_user) if task.assigned_to_user else None,
        assigned_to_name=None,
        assigned_team=task.assigned_team,
        contractor_name=task.contractor_name,
        due_date=task.due_date,
        created_at=task.created_at
    )

@router.put("/{task_id}/status")
async def update_task_status(task_id: str, req: TaskStatusUpdate, db: AsyncSession = Depends(get_db)):
    t_uuid = uuid.UUID(task_id)
    t_res = await db.execute(select(Task).where(Task.id == t_uuid))
    task = t_res.scalar_one_or_none()
    if not task:
        raise HTTPException(status_code=404, detail="Task not found")

    task.status = req.status
    if req.completion_notes:
        task.completion_notes = req.completion_notes
    await db.commit()
    return {"status": "success", "task_code": task.task_code, "new_status": task.status}
