use std::{collections::HashMap, path::PathBuf, sync::Arc};

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::{FlowfullError, Result};

#[async_trait]
pub trait Storage: Send + Sync {
    async fn get_item(&self, key: &str) -> Result<Option<String>>;
    async fn set_item(&self, key: &str, value: &str) -> Result<()>;
    async fn remove_item(&self, key: &str) -> Result<()>;
    async fn clear(&self) -> Result<()>;
}

#[derive(Debug, Clone, Default)]
pub struct MemoryStorage {
    data: Arc<RwLock<HashMap<String, String>>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl Storage for MemoryStorage {
    async fn get_item(&self, key: &str) -> Result<Option<String>> {
        Ok(self.data.read().await.get(key).cloned())
    }

    async fn set_item(&self, key: &str, value: &str) -> Result<()> {
        self.data
            .write()
            .await
            .insert(key.to_string(), value.to_string());
        Ok(())
    }

    async fn remove_item(&self, key: &str) -> Result<()> {
        self.data.write().await.remove(key);
        Ok(())
    }

    async fn clear(&self) -> Result<()> {
        self.data.write().await.clear();
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct FileStorage {
    base_path: PathBuf,
}

impl FileStorage {
    pub fn new(base_path: impl Into<PathBuf>) -> Self {
        Self {
            base_path: base_path.into(),
        }
    }

    fn path_for_key(&self, key: &str) -> Result<PathBuf> {
        if key.is_empty()
            || key.contains('/')
            || key.contains('\\')
            || key.contains("..")
            || key.contains(':')
        {
            return Err(FlowfullError::Storage(format!(
                "invalid storage key: {key}"
            )));
        }
        Ok(self.base_path.join(key))
    }
}

#[async_trait]
impl Storage for FileStorage {
    async fn get_item(&self, key: &str) -> Result<Option<String>> {
        let path = self.path_for_key(key)?;
        match tokio::fs::read_to_string(path).await {
            Ok(value) => Ok(Some(value)),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err.into()),
        }
    }

    async fn set_item(&self, key: &str, value: &str) -> Result<()> {
        tokio::fs::create_dir_all(&self.base_path).await?;
        let path = self.path_for_key(key)?;
        tokio::fs::write(path, value).await?;
        Ok(())
    }

    async fn remove_item(&self, key: &str) -> Result<()> {
        let path = self.path_for_key(key)?;
        match tokio::fs::remove_file(path).await {
            Ok(()) => Ok(()),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(err) => Err(err.into()),
        }
    }

    async fn clear(&self) -> Result<()> {
        match tokio::fs::remove_dir_all(&self.base_path).await {
            Ok(()) => Ok(()),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(err) => Err(err.into()),
        }
    }
}
