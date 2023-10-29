#![allow(non_snake_case)]

#[cfg(feature = "ssr")]
use app_ctx::AppCtx;

use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

#[cfg(feature = "ssr")]
mod app_ctx;
#[cfg(feature = "ssr")]
mod settings;
mod states;

mod views;

mod utils;
use views::*;

use crate::states::*;
mod router;
pub use router::*;

#[cfg(feature = "ssr")]
lazy_static::lazy_static! {
    pub static ref APP_CTX: AppCtx = {
        AppCtx::new()
    };
}

pub const METRICS_HISTORY_SIZE: usize = 150;

fn main() {
    let config = LaunchBuilder::<FullstackRouterConfig<AppRoute>>::router();

    #[cfg(feature = "ssr")]
    let config = config.addr(std::net::SocketAddr::from(([0, 0, 0, 0], 8080)));

    config.launch();
}

fn Home(cx: Scope) -> Element {
    use_shared_state_provider(cx, || MainState::new());

    let active_window = use_shared_state::<MainState>(cx)
        .unwrap()
        .read()
        .get_active_window();

    let content = match active_window {
        ActiveWindow::TopicsAndQueues => render_topics_and_queues(cx),
        ActiveWindow::Connections => render_connections(cx),
    };
    //
    //
    render! {
        div { id: "layout",
            div { id: "menu-bar", menu_panel {} }

            div { id: "main-content", content }
            status_bar_widget {}
        }
    }
}
