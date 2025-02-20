use dialogs::*;
use dioxus::prelude::*;

mod states;

mod views;

mod utils;

use views::*;

use crate::states::*;

#[cfg(feature = "server")]
mod server;

pub const METRICS_HISTORY_SIZE: usize = 150;

fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(server_only!(ServeConfig::builder().incremental(
            IncrementalRendererConfig::default()
                .invalidate_after(std::time::Duration::from_secs(120)),
        )))
        .launch(App)
}

#[component]
fn App() -> Element {
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

#[component]
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
            RenderDialog {}
        }
    }
}

#[server]
async fn get_envs() -> Result<Vec<String>, ServerFnError> {
    let settings = crate::server::APP_CTX.settings_reader.get_settings().await;
    Ok(settings.envs.iter().map(|env| env.id.clone()).collect())
}
