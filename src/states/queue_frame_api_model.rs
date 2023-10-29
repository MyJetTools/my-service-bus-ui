use serde::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct QueueFrameApiModel {
    #[serde(rename = "fromId")]
    pub from_id: i64,
    #[serde(rename = "toId")]
    pub to_id: i64,
}
