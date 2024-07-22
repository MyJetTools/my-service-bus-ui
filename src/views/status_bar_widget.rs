use crate::{states::*, utils::format_mem};
use dioxus::prelude::*;

#[component]
pub fn StatusBarWidget() -> Element {
    let main_state = consume_context::<Signal<MainState>>();

    let main_state_read_access = &main_state.read();

    let (status_bar_data, sessions_count) = if let Some(data) = main_state_read_access.data.as_ref()
    {
        (data.status_bar.clone(), data.sessions.len())
    } else {
        (StatusBarState::new(), 0)
    };

    let calculated_values = main_state_read_access.get_status_bar_calculated_values();

    let persist_queue = if calculated_values.persist_queue < 5000 {
        rsx! {
            b { style: "color:green", "{calculated_values.persist_queue}" }
        }
    } else {
        rsx! {
            b { style: "color:red", "{calculated_values.persist_queue}" }
        }
    };

    let connected = if status_bar_data.connected {
        rsx! {
            b { style: "color:green", "Yes" }
        }
    } else {
        rsx! {
            b { style: "color:red", "No" }
        }
    };

    let session_color = if sessions_count == 0 {
        "green"
    } else {
        "black"
    };

    let mem_used = format_mem(status_bar_data.mem_used);
    let mem_total = format_mem(status_bar_data.mem_total);

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

                td { "SB: {status_bar_data.version}" }
                td {
                    div { class: "status-bar-separator" }
                }
                td { "Persistence:{status_bar_data.persistence_ver}" }
            }
        }
    }
}
