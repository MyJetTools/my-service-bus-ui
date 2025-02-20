use app_ctx::AppCtx;

mod app_ctx;
mod settings;

lazy_static::lazy_static! {
    pub static ref APP_CTX: std::sync::Arc<AppCtx> = {
        use std::{sync::Arc, time::Duration};
        use app_ctx::UpdateTimer;
        use rust_extensions::MyTimer;

        let app_ctx = Arc::new(AppCtx::new());

        let mut timer = MyTimer::new(Duration::from_secs(1));

        timer.register_timer("Background request", Arc::new(UpdateTimer::new(app_ctx.clone())));

        timer.start(app_ctx.app_states.clone(), my_logger::LOGGER.clone());



        app_ctx
    };
}
