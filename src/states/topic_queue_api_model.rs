use super::QueueFrameApiModel;
use serde::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct TopicQueueApiModel {
    pub id: String,
    #[serde(rename = "queueType")]
    pub queue_type: u8,
    pub size: i64,
    #[serde(rename = "onDelivery")]
    pub on_delivery: i64,
    pub data: Vec<QueueFrameApiModel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopicQueueWrapperApiModel {
    pub queues: Vec<TopicQueueApiModel>,
    #[serde(rename = "snapshotId")]
    pub snapshot_id: i64,
}
