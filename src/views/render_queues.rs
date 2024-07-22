use dioxus::prelude::*;
use std::rc::Rc;

use super::icons::*;
use crate::{states::MainState, views::*};

#[component]
pub fn RenderQueues(topic_id: Rc<String>) -> Element {
    let main_state = consume_context::<Signal<MainState>>();

    let main_state_read_access = main_state.read();


    if main_state_read_access.data.is_none(){
        return rsx!{ "No data loaded" };
    }

    let data = main_state_read_access.data.as_ref().unwrap();

    let queues = data.queues.get(topic_id.as_str());

    if queues.is_none() {
        return rsx! {
            div { "No queues" }
        };
    }

    let queues = queues.unwrap();


    let mut sorted_queues = queues.queues.iter().map(|queue|{
        queue
    }).collect::<Vec<_>>();


    sorted_queues.sort_by(|a,b|{
        a.id.cmp(&b.id)
    });

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
            let subscriber_led = if subscriber.active==0{
                rsx!{
                    div { class: "led-gray" }
                }
            }else{
                rsx!{
                    div { class: "led-blue" }
                }
            };

            let subscriber_style = if subscriber.delivery_state==0{
                "text-bg-primary"
            }else{
                "text-bg-danger"
            };

            let values = subscriber.history.clone();

            let session = main_state_read_access.get_session(subscriber.session_id);

            if session.is_none(){
                return rsx!{
                    div { "Unknown session" }
                };
            }

            let session = session.unwrap();
            

            rsx!{
                table {
                    class: "table table-dark",
                    style: "--bg-color:var(--vz-table-bg);box-shadow: 0 0 3px black;  margin: 5px; width: 340px; background-color: var(--vz-table-bg);",
                    tr {
                        td {
                            {subscriber_led},
                            div {
                                span { class: "badge text-bg-dark", "{session.id}" }
                            }
                            div {
                                span { class: "badge {subscriber_style}", "{subscriber.id}" }
                            }
                        }
                        td {
                            div { class: "info-line-xs", "{session.name}" }
                            div { class: "info-line-xs", "{session.get_session_as_string()}" }
                            div { class: "info-line-xs", "{session.ip}" }
                            RenderGraph { elements: values, is_amount: false }
                        }
                    }
                }
            }
        });



        rsx! {
            tr { style: "vertical-align: top; background-color:{bg_color}; --bg-color:{bg_color}; width:100%;",
                td {
                    div { class: "info-line-bold", "{queue.id}" }
                    div { class: "info-line",
                        span { class: "badge {session_badge}",
                            PlugIcon {}
                            "{sessions_amount}"
                        }
                        {queue_type},
                        {q_size_badge},
                        {q_periods}
                    }
                }
                td { style: "vertical-align: top; width:350px", {rendered_subscribers} }
            }
        }
    });




    rsx! {
        table { style: "width:100%; background-color: var(--bg-color);", {queues} }
    }
}
