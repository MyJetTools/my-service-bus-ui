use std::collections::HashMap;

use tokio::sync::Mutex;

use crate::models::RequestApiModel;

pub struct CachedData {
    data: Mutex<HashMap<String, RequestApiModel>>,
    default: RequestApiModel,
}

impl CachedData {
    pub fn new() -> Self {
        Self {
            data: Mutex::new(HashMap::new()),
            default: RequestApiModel::default(),
        }
    }

    pub async fn update(&self, env: &str, data: RequestApiModel) {
        let mut write_access = self.data.lock().await;
        write_access.insert(env.to_string(), data.into());
    }

    pub async fn get(&self, env: &str) -> RequestApiModel {
        let read_access = self.data.lock().await;
        match read_access.get(env) {
            Some(data) => data.clone(),
            None => self.default.clone(),
        }
    }
}
