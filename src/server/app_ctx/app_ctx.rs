use std::sync::Arc;

use flurl::{my_ssh::SshSessionsPool, FlUrl};
use my_settings_reader::SettingsReader;
use rust_extensions::AppStates;

use crate::server::settings::{EnvSettingsModel, SettingsModel};

use super::CachedData;

pub struct AppCtx {
    pub settings_reader: SettingsReader<SettingsModel>,
    pub cached_data: CachedData,
    pub app_states: Arc<AppStates>,
}

impl AppCtx {
    pub fn new() -> Self {
        Self {
            settings_reader: SettingsReader::new("~/.my-sb-ui"),
            cached_data: CachedData::new(),
            app_states: Arc::new(AppStates::create_initialized()),
        }
    }

    pub async fn get_fl_url(&self, env_settings: &EnvSettingsModel) -> FlUrl {
        let settings = self.settings_reader.get_settings().await;
        let fl_url = FlUrl::new(env_settings.sb_api_url.as_str())
            .set_ssh_security_credentials_resolver(settings);
        fl_url
    }
}
