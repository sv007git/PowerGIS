//! Role-Based Access Control (RBAC) and engineer profiles for PowerGIS.

use serde::{Deserialize, Serialize};

/// The 6 engineering and administrative roles supported by PowerGIS.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Admin,
    SuperintendentEngineer,
    #[default]
    ExecutiveEngineer,
    AssistantEngineer,
    JuniorEngineer,
    Guest,
}

impl UserRole {
    pub const ALL: [UserRole; 6] = [
        UserRole::Admin,
        UserRole::SuperintendentEngineer,
        UserRole::ExecutiveEngineer,
        UserRole::AssistantEngineer,
        UserRole::JuniorEngineer,
        UserRole::Guest,
    ];

    pub fn code(&self) -> &'static str {
        match self {
            UserRole::Admin => "admin",
            UserRole::SuperintendentEngineer => "superintendent_engineer",
            UserRole::ExecutiveEngineer => "executive_engineer",
            UserRole::AssistantEngineer => "assistant_engineer",
            UserRole::JuniorEngineer => "junior_engineer",
            UserRole::Guest => "guest",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            UserRole::Admin => "Admin (System Administrator)",
            UserRole::SuperintendentEngineer => "Superintendent Engineer (SE)",
            UserRole::ExecutiveEngineer => "Executive Engineer (EE)",
            UserRole::AssistantEngineer => "Assistant Engineer (AE)",
            UserRole::JuniorEngineer => "Junior Engineer (JE)",
            UserRole::Guest => "Guest (Read-Only)",
        }
    }

    pub fn short_label(&self) -> &'static str {
        match self {
            UserRole::Admin => "Admin",
            UserRole::SuperintendentEngineer => "SE",
            UserRole::ExecutiveEngineer => "EE",
            UserRole::AssistantEngineer => "AE",
            UserRole::JuniorEngineer => "JE",
            UserRole::Guest => "Guest",
        }
    }

    pub fn can_manage_users(&self) -> bool {
        matches!(self, UserRole::Admin)
    }

    pub fn can_admin_system(&self) -> bool {
        matches!(self, UserRole::Admin | UserRole::SuperintendentEngineer)
    }

    pub fn can_approve_work_orders(&self) -> bool {
        matches!(self, UserRole::Admin | UserRole::SuperintendentEngineer | UserRole::ExecutiveEngineer)
    }

    pub fn can_dispatch_crews(&self) -> bool {
        matches!(
            self,
            UserRole::Admin
                | UserRole::SuperintendentEngineer
                | UserRole::ExecutiveEngineer
                | UserRole::AssistantEngineer
        )
    }

    pub fn can_edit_layers(&self) -> bool {
        !matches!(self, UserRole::Guest)
    }

    pub fn can_report_faults(&self) -> bool {
        !matches!(self, UserRole::Guest)
    }

    pub fn can_export(&self) -> bool {
        !matches!(self, UserRole::Guest)
    }

    pub fn is_guest(&self) -> bool {
        matches!(self, UserRole::Guest)
    }
}

/// A user profile in PowerGIS.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserProfile {
    pub username: String,
    pub full_name: String,
    pub role: UserRole,
    pub department: String,
    pub division: String,
}

impl Default for UserProfile {
    fn default() -> Self {
        UserProfile {
            username: "ee_urban".to_string(),
            full_name: "Er. M. Sivakumar".to_string(),
            role: UserRole::ExecutiveEngineer,
            department: "Electricity Department".to_string(),
            division: "Pondicherry Operations".to_string(),
        }
    }
}

impl UserProfile {
    pub fn demo_profile(role: UserRole) -> Self {
        match role {
            UserRole::Admin => UserProfile {
                username: "admin_harish".into(),
                full_name: "Harish Vyas (Admin)".into(),
                role,
                department: "IT & GIS Center".into(),
                division: "State Headquarters".into(),
            },
            UserRole::SuperintendentEngineer => UserProfile {
                username: "se_pondicherry".into(),
                full_name: "Er. R. Sundararajan (SE)".into(),
                role,
                department: "Electricity Department".into(),
                division: "Pondicherry Circle".into(),
            },
            UserRole::ExecutiveEngineer => UserProfile {
                username: "ee_urban".into(),
                full_name: "Er. M. Sivakumar (EE)".into(),
                role,
                department: "Electricity Department".into(),
                division: "Urban Division".into(),
            },
            UserRole::AssistantEngineer => UserProfile {
                username: "ae_marapalam".into(),
                full_name: "Er. K. Anand (AE)".into(),
                role,
                department: "Electricity Department".into(),
                division: "Marapalam Sub-Division".into(),
            },
            UserRole::JuniorEngineer => UserProfile {
                username: "je_field".into(),
                full_name: "Er. V. Murugan (JE)".into(),
                role,
                department: "Electricity Department".into(),
                division: "Field Operations".into(),
            },
            UserRole::Guest => UserProfile {
                username: "guest_citizen".into(),
                full_name: "Public Citizen Viewer".into(),
                role,
                department: "Public Portal".into(),
                division: "Citizen Services".into(),
            },
        }
    }
}
