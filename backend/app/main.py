from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from datetime import datetime

from backend.app.core.config import settings
from backend.app.routers import (
    auth,
    organization,
    users,
    assets,
    gis,
    tickets,
    tasks,
    surveys,
    scada,
    reports
)

app = FastAPI(
    title="PowerGIS Enterprise GIS Backend API",
    description="Enterprise GIS Utility Planning, Asset Management and Field Operations Platform API for Electrical Utilities",
    version="1.0.0",
    docs_url="/docs",
    redoc_url="/redoc"
)

# CORS configuration allowing GitHub Pages and local development
app.add_middleware(
    CORSMiddleware,
    allow_origins=[
        "https://sv007git.github.io",
        "http://localhost:3000",
        "http://localhost:8000",
        "http://localhost:8080",
        "*"
    ],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Include Routers with /api/v1 prefix
api_prefix = settings.API_V1_PREFIX
app.include_router(auth.router, prefix=api_prefix)
app.include_router(organization.router, prefix=api_prefix)
app.include_router(users.router, prefix=api_prefix)
app.include_router(assets.router, prefix=api_prefix)
app.include_router(gis.router, prefix=api_prefix)
app.include_router(tickets.router, prefix=api_prefix)
app.include_router(tasks.router, prefix=api_prefix)
app.include_router(surveys.router, prefix=api_prefix)
app.include_router(scada.router, prefix=api_prefix)
app.include_router(reports.router, prefix=api_prefix)

@app.get("/health", tags=["System Health"])
async def health_check():
    return {
        "status": "healthy",
        "service": "powergis-backend",
        "version": "1.0.0",
        "timestamp": datetime.utcnow().isoformat()
    }

@app.get("/", tags=["Root"])
async def root():
    return {
        "name": "PowerGIS Enterprise GIS Utility API",
        "version": "1.0.0",
        "documentation": "/docs",
        "health": "/health",
        "supported_roles": 16,
        "org_hierarchy_levels": 6,
        "scada_support": "IEC-60870-5-104 / Modbus TCP"
    }

if __name__ == "__main__":
    import uvicorn
    uvicorn.run("backend.app.main:app", host="0.0.0.0", port=8000, reload=True)
