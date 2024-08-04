use serde::*;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubscriberApiModel {
    pub id: i64,
    #[serde(rename = "sessionId")]
    pub session_id: i64,
    #[serde(rename = "queueId")]
    pub queue_id: String,
    pub active: i64,
    #[serde(rename = "deliveryState")]
    pub delivery_state: u8,
    pub history: Vec<i32>,
}
