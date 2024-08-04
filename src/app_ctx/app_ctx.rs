use std::sync::Arc;

use flurl::{my_ssh::SshSessionsPool, FlUrl};
use my_settings_reader::SettingsReader;
use rust_extensions::AppStates;

use crate::settings::SettingsModel;

use super::CachedData;

pub struct AppCtx {
    pub settings_reader: SettingsReader<SettingsModel>,
    pub ssh_sessions_pool: Arc<SshSessionsPool>,
    pub cached_data: CachedData,
    pub app_states: Arc<AppStates>,
}

impl AppCtx {
    pub fn new() -> Self {
        Self {
            settings_reader: SettingsReader::new("~/.my-sb-ui"),
            ssh_sessions_pool: Arc::new(SshSessionsPool::new()),
            cached_data: CachedData::new(),
            app_states: Arc::new(AppStates::create_initialized()),
        }
    }

    pub async fn get_fl_url(&self, env: &str) -> FlUrl {
        let settings = self.settings_reader.get_settings().await;
        settings.get_fl_url(env, &self.ssh_sessions_pool).await
    }
}
