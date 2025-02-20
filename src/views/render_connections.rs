use dioxus::prelude::*;

use crate::{
    states::{MainState, SessionType},
    utils::format_mem,
};

#[component]
pub fn RenderConnections() -> Element {
    let main_state = consume_context::<Signal<MainState>>();

    let main_state_read_access = main_state.read();


    if main_state_read_access.data.is_none(){
        return rsx!{ "No data loaded" };
    }

    let mut odd = true;

    let filter_string = main_state_read_access.get_filter_string();

    let data = main_state_read_access.data.as_ref().unwrap();

    let sessions_to_render = data.sessions.iter().filter(|session|session.filter_me(filter_string.as_str())). map(|session| {
        let bg_color = if odd {
            "--vz-table-active-bg"
        } else {
            "--vz-table-striped-bg"
        };

        odd = !odd;

        let session_type = match session.get_session_type() {
            SessionType::Tcp => {
                rsx! {
                    span { class: "badge text-bg-success", "Tcp" }
                }
            }
            SessionType::Http => {
                rsx! {
                    span {
                        span { class: "badge text-bg-warning", "Http" }
                    }
                }
            }
        };

        let r_size = format_mem(session.read_size);
        let w_size = format_mem(session.written_size);

        let r_p_s = format_mem(session.read_per_sec);
        let w_p_s = format_mem(session.written_per_sec);


        
        let (publishers, subscribers) =  main_state_read_access.get_publishers_and_subscribers(session.id);


        let publishers_to_render = publishers.into_iter().map(|(publisher, active)|{


            if active>0{
                rsx! {
                    span { class: "badge text-bg-success my-badge", {publisher} }
                }
    
            }else {
                rsx! {
                    span { class: "badge text-bg-light my-badge", {publisher} }
                }
                    
            }

        });


        let subscribers_to_render = subscribers.into_iter().map(|(topic, queue, active)|{
            if active>0{
                rsx! {
                    span { class: "badge text-bg-success my-badge", "{topic}->{queue}" }
                }
    
            }else {
                rsx! {
                    span { class: "badge text-bg-light my-badge", "{topic}->{queue}" }
                }
                    
            }
        });


        
        let env_info = if let Some(env_info) = session.env_info.as_ref() {
            rsx! {
                span {
                    style: "background: white;color: black;",
                    class: "badge badge-light",
                    "{env_info}"
                }
            }
        } else {
            rsx! {}
        };

        rsx! {
            tr { style: "--bg-color:var({bg_color}); background-color:var({bg_color}); vertical-align: top;border-bottom: 1px solid black;",
                td {
                    div { class: "info-line", "{session.id}" }
                    div { class: "info-line", {session_type} }
                }
                td {
                    div { class: "info-line-bold", "{session.name}" }

                    div { class: "info-line-xs",
                        b { "Client ver: " }
                        "{session.get_session_as_string()}"
                    }

                    div { class: "info-line-xs",
                        b { "Ip: " }
                        "{session.ip}"
                    }

                    div { class: "info-line-xs", {env_info} }

                    div { class: "info-line-xs",
                        b { "Connected: " }
                        "{session.connected}"
                    }
                    div { class: "info-line-xs",
                        b { "Read: " }
                        {r_size}
                    }
                    div { class: "info-line-xs",
                        b { "Written: " }
                        {w_size}
                    }
                    div { class: "info-line-xs",
                        b { "R/sec: " }
                        {r_p_s}
                    }
                    div { class: "info-line-xs",
                        b { "W/sec: " }
                        {w_p_s}
                    }
                }
                td { {publishers_to_render} }
                td { {subscribers_to_render} }
            }
        }
    });

    rsx! {
        table {
            class: "table table-dark",
            style: "text-align: left;margin: 0; padding: 0;",
            tr { style: "background: black; ",
                th { "Id" }
                th { "Info" }
                th { "Publisher" }

                th { "Subscriber" }
            }
            {sessions_to_render}
        }
    }
}
