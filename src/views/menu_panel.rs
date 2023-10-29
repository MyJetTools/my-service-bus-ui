use crate::states::{ActiveWindow, MainState, RequestApiModel};
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

pub fn menu_panel(cx: Scope) -> Element {
    let main_state = use_shared_state::<MainState>(cx).unwrap();

    let (tcp_count, http_count) = main_state.read().get_tcp_http_connections_amount();

    let started = use_state(cx, || false);

    let active_window = main_state.read().get_active_window();

    let (topics_and_queues, connections) = match active_window {
        ActiveWindow::TopicsAndQueues => {
            let topics_and_queue = rsx! { a { class: "nav-link active", "Topics and Queues" } };

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

    render! {
        ul {
            class: "nav nav-tabs",
            style: "border-bottom: none;",
            onmounted: move |_| {
                if !started.get() {
                    request_loop(&cx, main_state);
                    started.set(true);
                }
            },
            li { topics_and_queues }
            li { connections }
            li {
                input {
                    style: "height: 35px;",
                    class: "form-control",
                    oninput: |ctx| {
                        main_state.write().set_filter_string(ctx.value.clone());
                    },
                    r#type: "text",
                    placeholder: "Filter"
                }
            }
        }
    }
}

fn request_loop(cx: &Scope, main_state: &UseSharedState<MainState>) {
    let main_state = main_state.to_owned();

    cx.spawn(async move {
        let mut no = 0;
        loop {
            let result = get_metrics(no).await;
            no += 1;

            match result {
                Ok(result) => {
                    println!("Updating: {:?}", result);
                    let mut main_state = main_state.write();

                    main_state.status_bar.connected = true;
                    main_state.status_bar.persistence_ver = result.persistence_version;
                    main_state.status_bar.mem_used = result.system.usedmem;
                    main_state.status_bar.mem_total = result.system.totalmem;

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
    })
}

#[server]
async fn get_metrics(no: i32) -> Result<RequestApiModel, ServerFnError> {
    if no > 0 {
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    }

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
