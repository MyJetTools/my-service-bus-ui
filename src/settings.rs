use std::{collections::HashMap, sync::Arc};

use flurl::{
    my_ssh::{SshCredentialsSettingsModel, SshSessionsPool},
    FlUrl,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsModel {
    pub envs: Vec<EnvSettingsModel>,
    pub ssh_credentials: Option<HashMap<String, SshCredentialsSettingsModel>>,
}

impl SettingsModel {
    pub async fn get_fl_url(
        &self,
        env_id: &str,
        ssh_sessions_pool: &Arc<SshSessionsPool>,
    ) -> FlUrl {
        let env = self.envs.iter().find(|x| x.id == env_id);

        if env.is_none() {
            panic!("Configuration is not found for env '{}'", env_id);
        }
        env.unwrap()
            .get_fl_url(self.ssh_credentials.as_ref(), ssh_sessions_pool)
            .await
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
    pub async fn get_fl_url(
        &self,
        ssh_credentials: Option<&HashMap<String, SshCredentialsSettingsModel>>,
        ssh_sessions_pool: &Arc<SshSessionsPool>,
    ) -> FlUrl {
        let mut over_ssh = flurl::my_ssh::OverSshConnectionSettings::parse(
            self.sb_api_url.as_str(),
            ssh_credentials,
        )
        .await;

        if let Some(ssh_creds) = over_ssh.ssh_credentials.take() {
            return FlUrl::new(over_ssh.remote_resource_string)
                .set_ssh_credentials(Arc::new(ssh_creds))
                .set_ssh_sessions_pool(ssh_sessions_pool.clone());
        }

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
