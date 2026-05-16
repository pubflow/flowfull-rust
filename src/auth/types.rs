use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, de};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub struct FlexibleBool(pub bool);

impl<'de> Deserialize<'de> for FlexibleBool {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        match value {
            serde_json::Value::Bool(value) => Ok(Self(value)),
            serde_json::Value::Number(value) => Ok(Self(value.as_i64().unwrap_or_default() != 0)),
            serde_json::Value::String(value) => {
                let normalized = value.to_ascii_lowercase();
                Ok(Self(matches!(normalized.as_str(), "1" | "true" | "yes")))
            }
            _ => Err(de::Error::custom("invalid flexible bool")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: String,
    pub email: String,
    #[serde(default)]
    pub name: String,
    pub last_name: Option<String>,
    pub user_name: Option<String>,
    pub user_type: Option<String>,
    pub picture: Option<String>,
    pub phone: Option<String>,
    pub mobile: Option<String>,
    #[serde(default)]
    pub is_verified: FlexibleBool,
    #[serde(default)]
    pub two_factor: FlexibleBool,
    pub lang: Option<String>,
    pub tmz: Option<String>,
    pub bio: Option<String>,
    pub dob: Option<String>,
    pub gender: Option<String>,
    pub display_name: Option<String>,
    pub recovery_email: Option<String>,
    pub reference_id: Option<String>,
    pub first_time: Option<bool>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deletion_reason: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub last_used_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoginCredentials {
    pub password: String,
    pub email: Option<String>,
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegisterData {
    pub email: String,
    pub password: String,
    pub name: String,
    pub last_name: Option<String>,
    pub user_name: Option<String>,
    pub phone: Option<String>,
    pub user_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoginResult {
    #[serde(default)]
    pub success: bool,
    pub user: User,
    #[serde(default)]
    pub session_id: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub message: Option<String>,
    pub is_new_user: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PasswordResetRequest {
    pub email: String,
    pub reset_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PasswordResetComplete {
    pub token: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PasswordChange {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResendVerification {
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TokenValidation {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidationResult {
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub valid: bool,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TokenCreate {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub method: String,
    pub login_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TokenCreateResult {
    #[serde(default)]
    pub success: bool,
    pub message: Option<String>,
    pub method: Option<String>,
    pub expires_in: Option<u64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SocialProvider {
    Google,
    Github,
    Facebook,
    Apple,
    Discord,
    Microsoft,
}

impl std::fmt::Display for SocialProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Google => "google",
            Self::Github => "github",
            Self::Facebook => "facebook",
            Self::Apple => "apple",
            Self::Discord => "discord",
            Self::Microsoft => "microsoft",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SocialProviderInfo {
    pub provider: SocialProvider,
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "loginUrl", alias = "login_url")]
    pub login_url: Option<String>,
    #[serde(rename = "apiLoginUrl", alias = "api_login_url")]
    pub api_login_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SocialProvidersResult {
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub providers: Vec<SocialProviderInfo>,
    #[serde(rename = "allProviders", alias = "all_providers", default)]
    pub all_providers: Vec<SocialProviderInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SocialLoginData {
    pub access_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SocialAccount {
    pub provider: SocialProvider,
    pub provider_user_id: Option<String>,
    pub email: Option<String>,
    pub linked_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SocialAccountsResult {
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub accounts: Vec<SocialAccount>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SessionValidationResult {
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub valid: bool,
    pub user: Option<User>,
    pub session: Option<Session>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProfileUpdate {
    pub name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub user_type: Option<String>,
}
