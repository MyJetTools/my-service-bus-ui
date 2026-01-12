use dioxus::prelude::*;

use crate::models::*;

#[get("/api/envs")]
pub async fn get_envs() -> Result<Vec<String>, ServerFnError> {
    let settings = crate::server::APP_CTX.settings_reader.get_settings().await;
    Ok(settings.envs.iter().map(|env| env.id.clone()).collect())
}

#[get("/api/envs?env")]
pub async fn get_metrics(env: String) -> Result<RequestApiModel, ServerFnError> {
    let result = crate::server::APP_CTX.cached_data.get(env.as_str()).await;
    Ok(result)
}
