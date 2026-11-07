import os
from typing import List
from pydantic_settings import BaseSettings

class Settings(BaseSettings):
    PROJECT_NAME: str = "PowerGIS Enterprise GIS Utility Portal"
    API_V1_STR: str = "/api/v1"
    SECRET_KEY: str = os.getenv("SECRET_KEY", "powergis-super-secure-jwt-secret-key-production-2026")
    ALGORITHM: str = "HS256"
    ACCESS_TOKEN_EXPIRE_MINUTES: int = 60 * 24  # 24 hours
    
    # Database
    DATABASE_URL: str = os.getenv(
        "DATABASE_URL", 
        "postgresql+asyncpg://powergis_user:PowerGIS_Secure_Pass_2026@localhost:5432/powergis_db"
    )
    SYNC_DATABASE_URL: str = os.getenv(
        "SYNC_DATABASE_URL", 
        "postgresql://powergis_user:PowerGIS_Secure_Pass_2026@localhost:5432/powergis_db"
    )
    
    # Redis Cache & Message Broker
    REDIS_URL: str = os.getenv("REDIS_URL", "redis://localhost:6379/0")
    
    # CORS
    BACKEND_CORS_ORIGINS: List[str] = [
        "https://sv007git.github.io",
        "http://localhost:3000",
        "http://localhost:8000",
        "http://localhost:8080",
        "*"
    ]
    
    # SCADA Integration Settings (Module 27)
    SCADA_SIMULATION_MODE: bool = True
    SCADA_POLL_INTERVAL_SECONDS: int = 5

    class Config:
        case_sensitive = True
        env_file = ".env"

settings = Settings()
