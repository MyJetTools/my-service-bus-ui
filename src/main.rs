#![allow(non_snake_case)]

#[cfg(feature = "server")]
use app_ctx::*;

use dioxus::prelude::*;

#[cfg(feature = "server")]
mod app_ctx;
#[cfg(feature = "server")]
mod settings;
mod states;

mod views;

mod utils;

use views::*;

use crate::states::*;

#[cfg(feature = "server")]
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

pub const METRICS_HISTORY_SIZE: usize = 150;

fn main() {
    let cfg = dioxus::fullstack::Config::new();

    #[cfg(feature = "server")]
    let cfg = cfg.addr(([0, 0, 0, 0], 9001));

    LaunchBuilder::fullstack().with_cfg(cfg).launch(Home)
}

fn Home() -> Element {
    use_context_provider(|| Signal::new(MainState::new()));

    let mut main_state = consume_context::<Signal<MainState>>();

    let has_envs = main_state.read().has_envs();

    if has_envs {
        return ActiveApp();
    }

    let resource = use_resource(|| get_envs());

    let data = resource.read_unchecked();

    match &*data {
        Some(data) => match data {
            Ok(result) => {
                main_state.write().set_environments(result.clone());
                return ActiveApp();
            }
            Err(err) => {
                let err = format!("Error loading environments. Err: {}", err);
                return rsx! {
                    {err}
                };
            }
        },

        None => {
            return rsx! { "Loading environments..." };
        }
    }
}

fn ActiveApp() -> Element {
    let main_state = consume_context::<Signal<MainState>>();

    let active_window = main_state.read().get_active_window();

    let content = match active_window {
        ActiveWindow::TopicsAndQueues => rsx! {
            RenderTopicsAndQueues {}
        },
        ActiveWindow::Connections => RenderConnections(),
    };

    rsx! {
        div { id: "layout",
            div { id: "menu-bar", MenuPanel {} }

            div { id: "main-content", {content} }
            StatusBarWidget {}
        }
    }
}

#[server]
async fn get_envs() -> Result<Vec<String>, ServerFnError> {
    let settings = crate::APP_CTX.settings_reader.get_settings().await;
    Ok(settings.envs.iter().map(|env| env.id.clone()).collect())
}
