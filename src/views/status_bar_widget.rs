use crate::{states::*, utils::format_mem};
use dioxus::prelude::*;

#[component]
pub fn StatusBarWidget() -> Element {
    let main_state = consume_context::<Signal<MainState>>();

    let main_state = &main_state.read();

    let calculated_values = main_state.get_status_bar_calculated_values();

    let persist_queue = if calculated_values.persist_queue < 5000 {
        rsx! {
            b { style: "color:green", "{calculated_values.persist_queue}" }
        }
    } else {
        rsx! {
            b { style: "color:red", "{calculated_values.persist_queue}" }
        }
    };

    let connected = if main_state.status_bar.connected {
        rsx! {
            b { style: "color:green", "Yes" }
        }
    } else {
        rsx! {
            b { style: "color:red", "No" }
        }
    };

    let sessions_count = main_state.sessions.len();

    let session_color = if sessions_count == 0 {
        "green"
    } else {
        "black"
    };

    let mem_used = format_mem(main_state.status_bar.mem_used);
    let mem_total = format_mem(main_state.status_bar.mem_total);

    let total_pages_size = format_mem(calculated_values.total_pages_size);

    rsx! {
        table { id: "status-bar",
            tr {
                td {
                    "Connected: "
                    {connected}
                }
                td {
                    div { class: "status-bar-separator" }
                }
                td {
                    "Sessions: "
                    span { style: "color:{session_color}", "{sessions_count}" }
                }

                td {
                    div { class: "status-bar-separator" }
                }
                td {
                    "Persist queue: "
                    {persist_queue}
                }
                td {
                    div { class: "status-bar-separator" }
                }

                td { "Msgs/sec: {calculated_values.msg_per_sec}" }
                td {
                    div { class: "status-bar-separator" }
                }

                td { "Req/sec: {calculated_values.packets_per_sec}" }
                td {
                    div { class: "status-bar-separator" }
                }

                td { "Mem: {mem_used} of {mem_total}" }
                td {
                    div { class: "status-bar-separator" }
                }

                td { "Total Pages Size: {total_pages_size}" }
                td {
                    div { class: "status-bar-separator" }
                }

                td { "SB: {main_state.status_bar.version}" }
                td {
                    div { class: "status-bar-separator" }
                }
                td { "Persistence:{main_state.status_bar.persistence_ver}" }
            }
        }
    }
}
