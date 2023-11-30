use std::collections::HashMap;

use serde::*;

use super::{SessionApiModel, TopicQueueWrapperApiModel, TopicsApiModel};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestApiModel {
    pub topics: TopicsApiModel,
    pub sessions: SessionsApiModel,
    pub version: Option<String>,
    #[serde(rename = "persistenceVersion")]
    pub persistence_version: String,
    pub system: SystemApiModel,
    pub queues: HashMap<String, TopicQueueWrapperApiModel>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionsApiModel {
    #[serde(rename = "snapshotId")]
    pub snapshot_id: i64,
    pub items: Vec<SessionApiModel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemApiModel {
    pub usedmem: i64,
    pub totalmem: i64,
}
