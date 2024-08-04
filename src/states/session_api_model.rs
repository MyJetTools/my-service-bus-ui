use serde::*;

pub enum SessionType {
    Tcp,
    Http,
}

impl SessionType {
    pub fn is_tcp(&self) -> bool {
        match self {
            SessionType::Tcp => true,
            _ => false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionApiModel {
    pub id: i64,
    pub name: String,
    pub ip: String,
    pub version: Option<String>,
    pub connected: String,
    #[serde(rename = "lastIncoming")]
    pub last_incoming: String,
    #[serde(rename = "readSize")]
    pub read_size: i64,
    #[serde(rename = "writtenSize")]
    pub written_size: i64,
    #[serde(rename = "readPerSec")]
    pub read_per_sec: i64,
    #[serde(rename = "writtenPerSec")]
    pub written_per_sec: i64,
    #[serde(rename = "type")]
    pub session_type: Option<String>,
}

impl SessionApiModel {
    pub fn get_session_as_string(&self) -> &str {
        match &self.version {
            Some(version) => version,
            None => "???",
        }
    }
    pub fn get_session_type(&self) -> SessionType {
        match &self.session_type {
            Some(session_type) => {
                if session_type.starts_with("tcp") {
                    SessionType::Tcp
                } else {
                    SessionType::Http
                }
            }
            None => SessionType::Tcp,
        }
    }

    pub fn filter_me(&self, filter_string: &str) -> bool {
        if filter_string.is_empty() {
            return true;
        }

        if self.name.to_lowercase().contains(filter_string) {
            return true;
        }

        false
    }
}
