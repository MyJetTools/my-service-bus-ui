use dioxus::prelude::*;
use std::rc::Rc;

use super::icons::*;
use crate::{states::MainState, views::*};

#[component]
pub fn RenderQueues(topic_id: Rc<String>) -> Element {
    let main_state = consume_context::<Signal<MainState>>();

    let main_state_read_access = main_state.read();

    if main_state_read_access.data.is_none() {
        return rsx! { "No data loaded" };
    }

    let data = main_state_read_access.data.as_ref().unwrap();

    let queues = data.queues.get(topic_id.as_str());

    if queues.is_none() {
        return rsx! {
            div { "No queues" }
        };
    }

    let queues = queues.unwrap();

    let mut sorted_queues = queues.queues.iter().map(|queue| queue).collect::<Vec<_>>();

    sorted_queues.sort_by(|a, b| a.id.cmp(&b.id));

    let mut odd = false;

    let  queues = sorted_queues.into_iter().map(|queue| {
        let bg_color: &str = if odd {
            "var(--vz-table-active-bg)"
        }else{
            "var(--vz-table-striped-bg)"
        };
        odd = !odd;
        let sessions =
        main_state_read_access.get_sessions_connected_to_queue(topic_id.as_str(), queue.id.as_str());

        let sessions_amount = sessions.len();

        let session_badge = if sessions_amount > 0 {
            "text-bg-info"
        } else {
            "text-bg-danger"
        };

        let q_size_badge = if queue.size > 1000 {
            rsx! {
                span { class: "badge text-bg-danger", "{queue.size}/{queue.on_delivery}" }
            }
        } else {
            rsx! {
                span { class: "badge text-bg-success", "{queue.size}/{queue.on_delivery}" }
            }
        };

        let queue_type = match queue.queue_type {
            0 => rsx! {
                span { class: "badge text-bg-warning", "permanent" }
            },
            1 => rsx! {
                span { class: "badge text-bg-success", "auto-delete" }
            },
            2 => {
                rsx! {
                    span { class: "badge text-bg-primary", "permanent-single-connect" }
                }
            }

            _ => rsx! {
                span { class: "badge text-bg-danger", "unknown" }
            },
        };

        let q_periods = if queue.data.len()==0{
            rsx! {
                div {}
            }
        } else {

            let mut result = Vec::new();
            for itm in queue.data.iter() {
                if result.len() > 0 {
                    result.push(rsx!{ " " } );
                }
                result.push(rsx!{
                    RenderMessageId { message_id: itm.from_id }
                    "-"
                    RenderMessageId { message_id: itm.to_id }
                });
            }

            rsx! {
                span { class: "badge text-bg-success", {result.into_iter()} }
            }
        };



        let rendered_subscribers = sessions.into_iter().map(|subscriber|{
            super::render_subscriber::render_subscriber(&main_state_read_access, subscriber)        });


        let delete_queue = if sessions_amount == 0{
            rsx!{
                span { style: "cursor:pointer", onclick: move |_| {}, " x" }
            }
        }else{
            rsx!{}
        };


        rsx! {
            tr { style: "vertical-align: top; background-color:{bg_color}; --bg-color:{bg_color}; width:100%;",
                td {
                    div { class: "info-line-bold", "{queue.id}" }
                    div { class: "info-line",
                        span { class: "badge {session_badge}",
                            PlugIcon {}
                            "{sessions_amount}"
                            {delete_queue}
                        }
                        {queue_type}
                        {q_size_badge}
                        {q_periods}
                    }
                }
                td { style: "vertical-align: top; width:300px", {rendered_subscribers} }
            }
        }
    });

    rsx! {
        table { style: "width:100%; background-color: var(--bg-color);", {queues} }
    }
}
