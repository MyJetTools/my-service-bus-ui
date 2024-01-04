use serde::*;

use super::{PageApiModel, SubscriberApiModel, TopicPublisherApiModel};

#[derive(Debug, Serialize, Deserialize)]
pub struct TopicApiModel {
    pub id: String,
    #[serde(rename = "messageId")]
    pub message_id: i64,
    #[serde(rename = "packetPerSec")]
    pub packet_per_sec: i64,
    #[serde(rename = "messagesPerSec")]
    pub messages_per_sec: i64,
    #[serde(rename = "persistSize")]
    pub persist_size: i64,
    #[serde(rename = "publishHistory")]
    pub publish_history: Vec<i32>,

    pub pages: Vec<PageApiModel>,

    pub publishers: Vec<TopicPublisherApiModel>,

    pub subscribers: Vec<SubscriberApiModel>,

    pub persist: Option<bool>,
}

impl TopicApiModel {
    pub fn filter_me(&self, filter_string: &str) -> bool {
        if filter_string.is_empty() {
            return true;
        }

        if self.id.to_lowercase().contains(filter_string) {
            return true;
        }

        for subscriber in &self.subscribers {
            if subscriber.queue_id.to_lowercase().contains(filter_string) {
                return true;
            }
        }

        false
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopicsApiModel {
    pub items: Vec<TopicApiModel>,
}
