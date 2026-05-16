use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::auth::types::User;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SessionData {
    pub user_id: String,
    pub email: String,
    pub name: Option<String>,
    pub user_type: Option<String>,
    pub organization_id: Option<String>,
    #[serde(default)]
    pub permissions: Vec<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub validated_at: DateTime<Utc>,
    pub user: Option<User>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidationOptions {
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub device_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationMode {
    Disabled,
    Standard,
    Advanced,
    Strict,
}

impl ValidationMode {
    pub fn build_options(
        self,
        ip: Option<String>,
        user_agent: Option<String>,
        device_id: Option<String>,
    ) -> ValidationOptions {
        match self {
            Self::Disabled => ValidationOptions::default(),
            Self::Standard => ValidationOptions {
                ip,
                user_agent: None,
                device_id: None,
            },
            Self::Advanced => ValidationOptions {
                ip,
                user_agent,
                device_id: None,
            },
            Self::Strict => ValidationOptions {
                ip,
                user_agent,
                device_id,
            },
        }
    }
}
