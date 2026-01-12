use serde::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageApiModel {
    pub id: i64,
    pub amount: i64,
    pub size: i64,
    pub persist_size: i64,
    #[serde(rename = "subPages")]
    pub sub_pages: Vec<i32>,
}
