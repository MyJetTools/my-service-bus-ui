use serde::*;
#[derive(Debug, Serialize, Deserialize)]
pub struct TopicPublisherApiModel {
    #[serde(rename = "sessionId")]
    pub session_id: i64,
    pub active: i64,
}
