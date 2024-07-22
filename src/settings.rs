use std::sync::Arc;

use flurl::FlUrl;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsModel {
    pub envs: Vec<EnvSettingsModel>,
}

impl SettingsModel {
    pub async fn get_fl_url(&self, env_id: &str) -> FlUrl {
        let env = self.envs.iter().find(|x| x.id == env_id);

        if env.is_none() {
            panic!("Configuration is not found for env '{}'", env_id);
        }
        env.unwrap().get_fl_url().await
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvSettingsModel {
    pub id: String,
    pub sb_api_url: String,
    pub host: Option<String>,
    pub cert_location: Option<String>,
    pub cert_password: Option<String>,
}

impl EnvSettingsModel {
    pub async fn get_fl_url(&self) -> FlUrl {
        if let Some(cert_location) = self.cert_location.as_ref() {
            if let Some(cert_password) = self.cert_password.as_ref() {
                let client_certificate = flurl::my_tls::ClientCertificate::load_pks12_from_file(
                    cert_location.as_str(),
                    cert_password.as_str(),
                )
                .await
                .unwrap();

                let result = FlUrl::new(self.sb_api_url.as_str())
                    .with_client_certificate(client_certificate);

                if let Some(host) = self.host.as_ref() {
                    return result.set_tls_domain(host.to_string());
                }

                return result;
            }
        }

        FlUrl::new(self.sb_api_url.as_str())
    }
}

pub struct SettingsReader {
    settings: Mutex<Option<Arc<SettingsModel>>>,
}

impl SettingsReader {
    pub fn new() -> Self {
        Self {
            settings: Mutex::new(None),
        }
    }

    pub async fn get_settings(&self) -> Arc<SettingsModel> {
        let mut settings_access = self.settings.lock().await;

        loop {
            if let Some(settings_access) = settings_access.clone() {
                return settings_access;
            }

            let file_name = rust_extensions::file_utils::format_path("~/.my-sb-ui");

            let content = tokio::fs::read_to_string(file_name.as_str()).await;

            if let Err(err) = &content {
                panic!(
                    "Can not read settings file '{}'. Err:{}",
                    file_name.as_str(),
                    err
                );
            }

            let content = content.unwrap();

            let model: SettingsModel = serde_yaml::from_str(content.as_str()).unwrap();

            let model = Arc::new(model);

            *settings_access = Some(model.clone());
        }
    }
}

fn read_env_variable(name: &str) -> String {
    match std::env::var(name) {
        Ok(url) => return url,
        Err(_) => panic!("{} is not set", name),
    }
}
