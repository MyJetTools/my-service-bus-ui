use std::sync::Arc;

use rust_extensions::{date_time::DateTimeAsMicroseconds, MyTimerTick};

use crate::RequestApiModel;

use super::AppCtx;

pub struct UpdateTimer {
    pub ctx: Arc<AppCtx>,
}

impl UpdateTimer {
    pub fn new(ctx: Arc<AppCtx>) -> Self {
        Self { ctx }
    }
}

#[async_trait::async_trait]
impl MyTimerTick for UpdateTimer {
    async fn tick(&self) {
        let setings = self.ctx.settings_reader.get_settings().await;
        for env in &setings.envs {
            println!("Loading data for env '{}'", env.id);
            let started_moment = DateTimeAsMicroseconds::now();
            let fl_url = env
                .get_fl_url(
                    setings.ssh_credentials.as_ref(),
                    &self.ctx.ssh_sessions_pool,
                )
                .await;

            let result: Result<RequestApiModel, _> = fl_url
                .append_path_segment("status")
                .get()
                .await
                .unwrap()
                .get_json()
                .await;

            if let Err(err) = &result {
                eprintln!("Error loading data from env: {}. Err: {:?}", env.id, err);
                continue;
            }

            let mut result = result.unwrap();

            result.topics.items.sort_by(|a, b| a.id.cmp(&b.id));

            result.sessions.items.sort_by(|a, b| a.id.cmp(&b.id));

            crate::APP_CTX.cached_data.update(&env.id, result).await;

            let now = DateTimeAsMicroseconds::now();
            println!(
                "Data loaded for env '{}' in {:?}",
                env.id,
                now.duration_since(started_moment).as_positive_or_zero()
            );
        }
    }
}
