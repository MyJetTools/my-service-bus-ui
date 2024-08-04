use std::collections::HashMap;

use serde::*;

use super::{SessionApiModel, TopicQueueWrapperApiModel, TopicsApiModel};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestApiModel {
    pub topics: TopicsApiModel,
    pub sessions: SessionsApiModel,
    pub version: Option<String>,
    #[serde(rename = "persistenceVersion")]
    pub persistence_version: String,
    pub system: SystemApiModel,
    pub queues: HashMap<String, TopicQueueWrapperApiModel>,
}

impl RequestApiModel {
    pub fn default() -> Self {
        Self {
            topics: TopicsApiModel { items: vec![] },
            sessions: SessionsApiModel {
                snapshot_id: 0,
                items: vec![],
            },
            version: None,
            persistence_version: "".to_string(),
            system: SystemApiModel {
                usedmem: 0,
                totalmem: 0,
            },
            queues: HashMap::new(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionsApiModel {
    #[serde(rename = "snapshotId")]
    pub snapshot_id: i64,
    pub items: Vec<SessionApiModel>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemApiModel {
    pub usedmem: i64,
    pub totalmem: i64,
}
