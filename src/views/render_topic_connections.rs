use std::rc::Rc;

use crate::states::MainState;
use dioxus::prelude::*;

#[inline_props]
pub fn render_topic_connections(cx: Scope, topic_id: Rc<String>) -> Element {
    let main_state = use_shared_state::<MainState>(cx).unwrap();

    let main_state = main_state.read();

    let topic = main_state.get_topic(topic_id).unwrap();

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
                                div { class: "info-line", "{session.version}" }
                                div { class: "info-line", "{session.ip}" }
                            }
                        }
                    }
                }
            }
            None => rsx! { div { "Unknown publisher: {publisher.session_id}" } },
        }
    });

    render! {items}
}
