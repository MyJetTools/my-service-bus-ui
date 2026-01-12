use std::time::Duration;

use crate::{
    api::*,
    states::{ActiveWindow, MainState},
    DataToRender, StatusBarState,
};
use dioxus::prelude::*;

#[component]
pub fn MenuPanel() -> Element {
    let mut main_state = consume_context::<Signal<MainState>>();

    let (tcp_count, http_count) = main_state.read().get_tcp_http_connections_amount();

    let mut started = use_signal(|| false);

    let active_window = main_state.read().get_active_window();

    let (topics_and_queues, connections) = match active_window {
        ActiveWindow::TopicsAndQueues => {
            let topics_and_queue = rsx! {
                a { class: "nav-link active", "Topics and Queues" }
            };

            let connections = rsx! {
                a {
                    class: "nav-link",
                    style: "cursor:pointer",
                    onclick: move |_| {
                        main_state.write().set_active_window(ActiveWindow::Connections);
                    },
                    "Connections "
                    span { class: "badge text-bg-success", "Tcp:{tcp_count}" }
                    span { class: "badge text-bg-warning", "Http:{http_count}" }
                }
            };

            (topics_and_queue, connections)
        }
        ActiveWindow::Connections => {
            let topics_and_queue = rsx! {
                a {
                    class: "nav-link",
                    style: "cursor:pointer",
                    onclick: move |_| {
                        main_state.write().set_active_window(ActiveWindow::TopicsAndQueues);
                    },
                    style: "cursor:pointer",
                    "Topics and Queues"
                }
            };

            let connections = rsx! {
                a { class: "nav-link active",
                    "Connections "
                    span { class: "badge text-bg-success", "Tcp:{tcp_count}" }
                    span { class: "badge text-bg-warning", "Http:{http_count}" }
                }
            };
            (topics_and_queue, connections)
        }
    };

    let started_value = *started.read();

    let tabs = rsx! {
        ul {
            class: "nav nav-tabs",
            style: "border-bottom: none;",
            onmounted: move |_| {
                if !started_value {
                    request_loop(main_state);
                    *started.write() = true;
                }
            },
            li { {topics_and_queues} }
            li { {connections} }
            li {
                input {
                    style: "height: 35px;",
                    class: "form-control",
                    oninput: |ctx| {
                        let value = ctx.value();
                        consume_context::<Signal<MainState>>().write().set_filter_string(value);
                    },
                    r#type: "text",
                    placeholder: "Filter",
                }
            }
        }
    };

    let envs = main_state.read().envs.clone().unwrap();

    let envs_options = envs.into_iter().map(|env| {
        rsx! {
            option { {env} }
        }
    });

    rsx! {
        table { style: "width:100%",

            tr {
                td { {tabs} }
                td {

                    select {
                        class: "form-select",
                        oninput: |ctx| {
                            let value = ctx.value();
                            consume_context::<Signal<MainState>>().write().set_active_env(value);
                        },
                        {envs_options}
                    }
                }
            }
        }
    }
}

fn request_loop(mut main_state: Signal<MainState>) {
    spawn(async move {
        loop {
            let env_id = main_state.read().get_active_env_id();

            let result = get_metrics(env_id).await;

            match result {
                Ok(result) => {
                    println!("Updating: {:?}", result);
                    let mut main_state = main_state.write();

                    let data = DataToRender {
                        status_bar: StatusBarState {
                            connected: true,
                            persistence_ver: result.persistence_version,
                            mem_used: result.system.usedmem,
                            mem_total: result.system.totalmem,
                            version: if let Some(version) = result.version {
                                version
                            } else {
                                "???".to_string()
                            },
                        },
                        topics: result.topics.items,
                        sessions: result.sessions.items,
                        queues: result.queues,
                    };
                    main_state.data = Some(data);
                }
                Err(err) => {
                    println!("Server err: {:?}", err);
                    main_state.write().data = None;
                }
            }

            dioxus_utils::js::sleep(Duration::from_secs(1)).await;
        }
    });
}
