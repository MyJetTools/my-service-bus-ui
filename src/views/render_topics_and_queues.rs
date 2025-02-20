use std::rc::Rc;

use dioxus::prelude::*;

use crate::{states::MainState, views::*};

#[component]
pub fn RenderTopicsAndQueues() -> Element {
    let main_state = consume_context::<Signal<MainState>>();

    let main_state_read_access = main_state.read();

    if main_state_read_access.data.is_none() {
        return rsx! { "No data loaded" };
    }

    let mut odd = false;

    let filter_string = main_state_read_access.get_filter_string();

    let data = main_state_read_access.data.as_ref().unwrap();

    if data.topics.is_empty() {
        return rsx! {
            div {}
        };
    }

    let items = data.topics.iter().filter(|topic|topic.filter_me(filter_string.as_str())). map(|topic| {

        let bg_color = if odd {
            "--vz-table-active-bg"
        }else{
            "--vz-table-striped-bg"
        };

        odd = !odd;

        let topic_id = Rc::new(topic.id.to_string());

        let values = topic.publish_history.clone();

        let rendered_pages = topic.pages.iter().map(|page| {

            let sub_pages = page.sub_pages.clone();
            rsx!{
                RenderPage {
                    page_no: page.id,
                    amount: page.amount,
                    size: page.size,
                    sub_pages,
                }
            }
        });

        let persist = if let Some(persist) = topic.persist{
            persist
        }else{
            true
        };


        let persist_queue = if persist{
            rsx!{
                div { class: "info-line-xs", "Persist queue: {topic.persist_size}" }
            }
        }else{
          rsx!{
            div { class: "info-line-xs",
                "Persist : "
                span { style: "color:red", "disabled" }
            }
        }
        };

        rsx! {
            tr { style: "--bg-color:var({bg_color}); background-color:var({bg_color}); vertical-align: top;border-bottom: 1px solid black;box-shadow: 0 0px 3px black;",
                td { style: "width:425px",
                    div { class: "info-line-bold", "{topic.id}" }
                    div { class: "info-line-xs",
                        "MsgId: "
                        RenderMessageId { message_id: topic.message_id }
                    }
                    div { class: "info-line-xs",
                        "Msg/Sec: "
                        span { style: "color:gray", "{topic.messages_per_sec}" }
                    }
                    div { class: "info-line-xs", "Req/Sec: {topic.packet_per_sec}" }
                    {persist_queue}
                    div { style: "padding: 5px;background-color: var(--bg-color);",
                        RenderGraph { elements: values, is_amount: true }
                    }
                    {rendered_pages}
                }
                td { style: "width:350px",
                    RenderTopicConnections { topic_id: topic_id.clone() }
                }
                td {
                    RenderQueues { topic_id }
                }
            }
        }
    });

    rsx! {
        table {
            class: "table table-striped table-dark",
            style: "text-align:left;margin: 0;",
            tr { style: "background-color:black;",
                th { "Topics" }
                th { "Topic connections" }
                th { "Queues" }
            }

            {items}
        }
    }
}
