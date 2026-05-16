use std::sync::Arc;

use serde::{Serialize, de::DeserializeOwned};
use tokio::sync::RwLock;

use crate::{FlowfullError, Result, config::ClientConfig, storage::Storage};

pub const PUBFLOW_SESSION_ID: &str = "pubflow_session_id";
pub const PUBFLOW_USER_DATA: &str = "pubflow_user_data";

const LEGACY_SESSION_KEYS: &[&str] = &["flowfull_session_id", "session_id", "sessionId"];

#[derive(Clone)]
pub struct SessionManager {
    config: Arc<ClientConfig>,
    storage: Option<Arc<dyn Storage>>,
    cached_session_id: Arc<RwLock<Option<String>>>,
}

impl SessionManager {
    pub fn new(config: Arc<ClientConfig>) -> Self {
        Self {
            storage: config.storage.clone(),
            config,
            cached_session_id: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn get_session_id(&self) -> Result<Option<String>> {
        if let Some(cached) = self.cached_session_id.read().await.clone() {
            return Ok(Some(cached));
        }

        if let Some(session_id) = &self.config.session_id {
            self.cache_session_id(session_id.clone()).await;
            return Ok(Some(session_id.clone()));
        }

        if let Some(provider) = &self.config.session_provider {
            if let Some(session_id) = provider().await? {
                if !session_id.is_empty() {
                    self.cache_session_id(session_id.clone()).await;
                    return Ok(Some(session_id));
                }
            }
        }

        if let Some(storage) = &self.storage {
            if let Some(session_id) = storage.get_item(PUBFLOW_SESSION_ID).await? {
                if !session_id.is_empty() {
                    self.cache_session_id(session_id.clone()).await;
                    return Ok(Some(session_id));
                }
            }

            for key in LEGACY_SESSION_KEYS {
                if let Some(session_id) = storage.get_item(key).await? {
                    if !session_id.is_empty() {
                        self.cache_session_id(session_id.clone()).await;
                        return Ok(Some(session_id));
                    }
                }
            }
        }

        Ok(None)
    }

    pub async fn set_session_id(&self, session_id: impl Into<String>) -> Result<()> {
        let session_id = session_id.into();
        self.cache_session_id(session_id.clone()).await;
        if let Some(storage) = &self.storage {
            storage.set_item(PUBFLOW_SESSION_ID, &session_id).await?;
        }
        Ok(())
    }

    pub async fn clear_session(&self) -> Result<()> {
        *self.cached_session_id.write().await = None;
        if let Some(storage) = &self.storage {
            storage.remove_item(PUBFLOW_SESSION_ID).await?;
        }
        Ok(())
    }

    pub async fn has_session(&self) -> Result<bool> {
        Ok(self.get_session_id().await?.is_some())
    }

    pub async fn set_user_data<T>(&self, user: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        let storage = self
            .storage
            .as_ref()
            .ok_or_else(|| FlowfullError::Storage("no storage configured".to_string()))?;
        let value = serde_json::to_string(user)?;
        storage.set_item(PUBFLOW_USER_DATA, &value).await
    }

    pub async fn get_user_data<T>(&self) -> Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        let Some(storage) = &self.storage else {
            return Ok(None);
        };
        let Some(value) = storage.get_item(PUBFLOW_USER_DATA).await? else {
            return Ok(None);
        };
        Ok(Some(serde_json::from_str(&value)?))
    }

    pub async fn clear_user_data(&self) -> Result<()> {
        if let Some(storage) = &self.storage {
            storage.remove_item(PUBFLOW_USER_DATA).await?;
        }
        Ok(())
    }

    pub async fn clear_all(&self) -> Result<()> {
        self.clear_session().await?;
        self.clear_user_data().await
    }

    async fn cache_session_id(&self, session_id: String) {
        *self.cached_session_id.write().await = Some(session_id);
    }
}
