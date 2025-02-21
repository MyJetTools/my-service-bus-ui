use std::rc::Rc;

use crate::states::MainState;
use dioxus::prelude::*;

#[component]
pub fn RenderTopicConnections(topic_id: Rc<String>) -> Element {
    let main_state = consume_context::<Signal<MainState>>();

    let main_state = main_state.read();

    let topic = main_state.get_topic(topic_id.as_str()).unwrap();

    let mut publishers: Vec<_> = topic.publishers.iter().collect();

    publishers.sort_by(|a, b| a.session_id.cmp(&b.session_id));

    let items = publishers.into_iter().map(|publisher| {
        match main_state.get_session(publisher.session_id) {
            Some(session) => {
                let class_name = if publisher.active == 0 {
                    "led-gray"
                } else {
                    "led-green"
                };

                let env_info = if let Some(env_info) = session.env_info.as_ref() {
                    rsx! {
                        div { style: "padding: 0;",
                            span {
                                style: "background: white;color: black;",
                                class: "badge badge-light",
                                "{env_info}"
                            }
                        }
                    }
                } else {
                    rsx! {}
                };

                rsx! {
                    table {
                        class: "table table-dark",
                        style: "--bg-color:var(--vz-table-bg);box-shadow: 0 0 3px black;  margin: 5px; width: 340px;",
                        tr {
                            td {
                                div { class: "{class_name}" }
                                div { class: "info-line", "{publisher.session_id}" }
                            }
                            td {
                                div { class: "info-line-bold", "{session.name}" }
                                div {
                                    class: "info-line",
                                    style: "font-size: 10px;",
                                    b { "MY-SB-SDK ver: " }
                                    "{session.get_session_as_string()}"
                                }
                                table { style: "width: 100%",
                                    tr {
                                        td {
                                            div { class: "info-line", "{session.ip}" }
                                        }
                                        td { style: "text-align:right", {env_info} }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            None => rsx! {
                div { "Unknown publisher: {publisher.session_id}" }
            },
        }
    });

    rsx! {
        {items}
    }
}
