use std::collections::HashMap;

use flurl::my_ssh::ssh_settings::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsModel {
    pub envs: Vec<EnvSettingsModel>,
    pub ssh_private_keys: Option<HashMap<String, SshPrivateKeySettingsModel>>,
}

#[async_trait::async_trait]
impl SshSecurityCredentialsResolver for SettingsModel {
    async fn resolve_ssh_private_key(&self, ssh_line: &str) -> Option<SshPrivateKey> {
        let private_keys = self.ssh_private_keys.as_ref()?;

        if let Some(ssh_credentials) = private_keys.get(ssh_line) {
            return SshPrivateKey {
                content: ssh_credentials.load_cert().await,
                pass_phrase: ssh_credentials.cert_pass_phrase.clone(),
            }
            .into();
        }

        if let Some(ssh_credentials) = private_keys.get("*") {
            return SshPrivateKey {
                content: ssh_credentials.load_cert().await,
                pass_phrase: ssh_credentials.cert_pass_phrase.clone(),
            }
            .into();
        }

        None
    }

    async fn resolve_ssh_password(&self, _ssh_line: &str) -> Option<String> {
        None
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
