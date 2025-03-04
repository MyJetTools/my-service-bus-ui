use std::collections::HashMap;

use super::{
    DialogState, SessionApiModel, StatusBarState, SubscriberApiModel, TopicApiModel,
    TopicQueueWrapperApiModel,
};

#[derive(Debug, Clone)]
pub enum ActiveWindow {
    TopicsAndQueues,
    Connections,
}

pub struct StatusBarCalculatedValue {
    pub msg_per_sec: i64,
    pub packets_per_sec: i64,
    pub persist_queue: i64,
    pub total_pages_size: i64,
}

pub struct DataToRender {
    pub topics: Vec<TopicApiModel>,
    pub sessions: Vec<SessionApiModel>,
    pub status_bar: StatusBarState,
    pub queues: HashMap<String, TopicQueueWrapperApiModel>,
}

pub struct MainState {
    pub active_window: ActiveWindow,
    pub filter_string: String,

    pub envs: Option<Vec<String>>,
    pub active_env: String,

    pub data: Option<DataToRender>,

    pub dialog: Option<DialogState>,
}

impl MainState {
    pub fn new() -> Self {
        Self {
            data: None,
            active_window: ActiveWindow::TopicsAndQueues,
            filter_string: String::new(),
            envs: None,
            active_env: "".to_string(),
            dialog: None,
        }
    }

    pub fn get_status_bar_calculated_values(&self) -> StatusBarCalculatedValue {
        let mut result = StatusBarCalculatedValue {
            msg_per_sec: 0,
            persist_queue: 0,
            packets_per_sec: 0,
            total_pages_size: 0,
        };

        if let Some(data) = &self.data {
            for topic in &data.topics {
                result.persist_queue += topic.persist_size;
                result.msg_per_sec += topic.messages_per_sec;
                result.packets_per_sec += topic.packet_per_sec;

                for page in &topic.pages {
                    result.total_pages_size += page.size;
                }
            }
        }

        result
    }

    pub fn get_topic(&self, topic_id: &str) -> Option<&TopicApiModel> {
        let data = self.data.as_ref()?;
        for topic in &data.topics {
            if topic.id == topic_id {
                return Some(topic);
            }
        }

        None
    }

    pub fn get_session(&self, id: i64) -> Option<&SessionApiModel> {
        let data = self.data.as_ref()?;
        for session in &data.sessions {
            if session.id == id {
                return Some(session);
            }
        }

        None
    }

    pub fn get_sessions_connected_to_queue(
        &self,
        topic_id: &str,
        queue_id: &str,
    ) -> Vec<&SubscriberApiModel> {
        let mut result = Vec::new();

        if let Some(data) = &self.data {
            for topic in &data.topics {
                if topic.id != topic_id {
                    continue;
                }

                for subscriber in &topic.subscribers {
                    if subscriber.queue_id == queue_id {
                        result.push(subscriber)
                    }
                }

                break;
            }
        }

        result
    }

    pub fn get_active_window(&self) -> ActiveWindow {
        self.active_window.clone()
    }

    pub fn set_active_window(&mut self, active_window: ActiveWindow) {
        self.active_window = active_window;
    }

    pub fn has_envs(&self) -> bool {
        self.envs.is_some()
    }

    pub fn set_environments(&mut self, envs: Vec<String>) {
        self.active_env = envs[0].clone();
        self.envs = Some(envs);
    }

    pub fn set_active_env(&mut self, env: String) {
        self.active_env = env;
        self.data = None;
    }

    pub fn get_active_env_id(&self) -> String {
        self.active_env.clone()
    }

    pub fn get_tcp_http_connections_amount(&self) -> (usize, usize) {
        let mut tcp = 0;
        let mut http = 0;

        if let Some(data) = &self.data {
            for session in &data.sessions {
                if session.get_session_type().is_tcp() {
                    tcp += 1;
                } else {
                    http += 1;
                }
            }
        }

        (tcp, http)
    }

    pub fn get_filter_string(&self) -> String {
        self.filter_string.trim().to_lowercase()
    }
    pub fn set_filter_string(&mut self, filter_string: String) {
        self.filter_string = filter_string;
    }

    pub fn get_publishers_and_subscribers(
        &self,
        session_id: i64,
    ) -> (Vec<(String, i64)>, Vec<(String, String, i64)>) {
        let mut publishers = Vec::new();
        let mut subscribers = Vec::new();

        if let Some(data) = &self.data {
            for topic in &data.topics {
                for publisher in &topic.publishers {
                    if publisher.session_id == session_id {
                        publishers.push((topic.id.clone(), publisher.active));
                    }
                }

                for subscriber in &topic.subscribers {
                    if subscriber.session_id == session_id {
                        subscribers.push((
                            topic.id.clone(),
                            subscriber.queue_id.clone(),
                            subscriber.active,
                        ));
                    }
                }
            }
        }

        (publishers, subscribers)
    }

    pub fn hide_dialog(&mut self) {
        self.dialog = None;
    }
}
