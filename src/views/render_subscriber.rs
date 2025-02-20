use dioxus::prelude::*;

use crate::{
    states::{MainState, SubscriberApiModel},
    views::RenderGraph,
};

pub fn render_subscriber(main_state: &MainState, subscriber: &SubscriberApiModel) -> Element {
    let subscriber_led = if subscriber.active == 0 {
        rsx! {
            div { class: "led-gray" }
        }
    } else {
        rsx! {
            div { class: "led-blue" }
        }
    };

    let subscriber_style = if subscriber.delivery_state == 0 {
        "text-bg-primary"
    } else {
        "text-bg-danger"
    };

    let values = subscriber.history.clone();

    let session = main_state.get_session(subscriber.session_id);

    if session.is_none() {
        return rsx! {
            div { "Unknown session" }
        };
    }

    let session = session.unwrap();

    let subscriber_info = if let Some(str) = subscriber.delivery_state_str.as_ref() {
        rsx! {
            div { style: "text-align:right; padding: 0",
                span { class: "badge {subscriber_style}", {str.as_str()} }
            }
        }
    } else {
        rsx! {}
    };

    rsx! {
        table {
            class: "table table-dark",
            style: "--bg-color:var(--vz-table-bg);box-shadow: 0 0 3px black;  margin: 5px; width: 290px; background-color: var(--vz-table-bg);",
            tr {
                td {
                    {subscriber_led}
                    div {
                        span { class: "badge text-bg-dark", "{session.id}" }
                    }
                    div {
                        span { class: "badge {subscriber_style}", "{subscriber.id}" }
                    }
                }
                td {
                    {subscriber_info}
                    div { class: "info-line-xs", "{session.name}" }
                    div { class: "info-line-xs", "{session.get_session_as_string()}" }
                    div { class: "info-line-xs", "{session.ip}" }
                    RenderGraph { elements: values, is_amount: false }
                }
            }
        }
    }
}
