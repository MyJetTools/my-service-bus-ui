use crate::states::{ActiveWindow, MainState, RequestApiModel};
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

    rsx! {
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
                    placeholder: "Filter"
                }
            }
        }
    }
}

fn request_loop(mut main_state: Signal<MainState>) {
    let mut eval = eval(
        r#"

        dioxus.send("");
        
        setInterval(function(){
            dioxus.send("");
        }, 1000);
        "#,
    );

    spawn(async move {
        loop {
            eval.recv().await.unwrap();

            let result = get_metrics().await;

            match result {
                Ok(result) => {
                    println!("Updating: {:?}", result);
                    let mut main_state = main_state.write();

                    main_state.status_bar.connected = true;
                    main_state.status_bar.persistence_ver = result.persistence_version;
                    main_state.status_bar.mem_used = result.system.usedmem;
                    main_state.status_bar.mem_total = result.system.totalmem;
                    main_state.status_bar.version = if let Some(version) = result.version {
                        version
                    } else {
                        "???".to_string()
                    };

                    main_state.topics = result.topics.items;
                    main_state.sessions = result.sessions.items;
                    main_state.queues = result.queues;
                }
                Err(err) => {
                    println!("Server err: {:?}", err);
                    main_state.write().status_bar.disconnected();
                }
            }
        }
    });
}

#[server]
async fn get_metrics() -> Result<RequestApiModel, ServerFnError> {
    let url = crate::APP_CTX.settings.get_api();

    let mut result: RequestApiModel = flurl::FlUrl::new(url)
        .append_path_segment("status")
        .get()
        .await
        .unwrap()
        .get_json()
        .await
        .unwrap();

    result.topics.items.sort_by(|a, b| a.id.cmp(&b.id));

    result.sessions.items.sort_by(|a, b| a.id.cmp(&b.id));

    return Ok(result);
}
