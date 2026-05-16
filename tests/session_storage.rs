use flowfull::{
    ClientConfigBuilder, FileStorage, MemoryStorage, PUBFLOW_SESSION_ID, PUBFLOW_USER_DATA,
    SessionManager, Storage,
};

#[tokio::test]
async fn memory_storage_round_trips_values() {
    let storage = MemoryStorage::new();
    storage
        .set_item(PUBFLOW_SESSION_ID, "ses_123")
        .await
        .unwrap();
    assert_eq!(
        storage.get_item(PUBFLOW_SESSION_ID).await.unwrap(),
        Some("ses_123".to_string())
    );
    storage.remove_item(PUBFLOW_SESSION_ID).await.unwrap();
    assert_eq!(storage.get_item(PUBFLOW_SESSION_ID).await.unwrap(), None);
}

#[tokio::test]
async fn file_storage_round_trips_values() {
    let dir = tempfile::tempdir().unwrap();
    let storage = FileStorage::new(dir.path());
    storage
        .set_item(PUBFLOW_SESSION_ID, "ses_file")
        .await
        .unwrap();
    assert_eq!(
        storage.get_item(PUBFLOW_SESSION_ID).await.unwrap(),
        Some("ses_file".to_string())
    );
}

#[tokio::test]
async fn session_manager_prefers_static_session() {
    let config = ClientConfigBuilder::new("https://api.example.com")
        .unwrap()
        .session_id("static_session")
        .storage(MemoryStorage::new())
        .build()
        .unwrap();
    let manager = SessionManager::new(std::sync::Arc::new(config));
    assert_eq!(
        manager.get_session_id().await.unwrap(),
        Some("static_session".to_string())
    );
}

#[tokio::test]
async fn session_manager_reads_legacy_keys() {
    let storage = MemoryStorage::new();
    storage.set_item("session_id", "legacy").await.unwrap();
    let config = ClientConfigBuilder::new("https://api.example.com")
        .unwrap()
        .storage(storage)
        .build()
        .unwrap();
    let manager = SessionManager::new(std::sync::Arc::new(config));
    assert_eq!(
        manager.get_session_id().await.unwrap(),
        Some("legacy".to_string())
    );
}

#[tokio::test]
async fn session_manager_stores_user_data() {
    #[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
    struct StoredUser {
        id: String,
    }

    let config = ClientConfigBuilder::new("https://api.example.com")
        .unwrap()
        .storage(MemoryStorage::new())
        .build()
        .unwrap();
    let manager = SessionManager::new(std::sync::Arc::new(config));
    manager
        .set_user_data(&StoredUser {
            id: "usr_1".to_string(),
        })
        .await
        .unwrap();
    let user: StoredUser = manager.get_user_data().await.unwrap().unwrap();
    assert_eq!(
        user,
        StoredUser {
            id: "usr_1".to_string()
        }
    );
    manager.clear_user_data().await.unwrap();
    assert_eq!(
        manager.get_user_data::<serde_json::Value>().await.unwrap(),
        None
    );
}

#[tokio::test]
async fn clear_all_removes_session_and_user() {
    let storage = MemoryStorage::new();
    let config = ClientConfigBuilder::new("https://api.example.com")
        .unwrap()
        .storage(storage.clone())
        .build()
        .unwrap();
    let manager = SessionManager::new(std::sync::Arc::new(config));
    manager.set_session_id("ses").await.unwrap();
    manager
        .set_user_data(&serde_json::json!({"id": "usr"}))
        .await
        .unwrap();
    manager.clear_all().await.unwrap();

    assert_eq!(storage.get_item(PUBFLOW_SESSION_ID).await.unwrap(), None);
    assert_eq!(storage.get_item(PUBFLOW_USER_DATA).await.unwrap(), None);
}
