use dioxus::prelude::{GlobalAttributes, *};

use crate::{
    states::{MainState, SessionType},
    utils::format_mem,
};

pub fn render_connections(cx: Scope) -> Element {
    let main_state = use_shared_state::<MainState>(cx).unwrap();

    let main_state_read_access = main_state.read();
    let mut odd = true;


    let filter_string = main_state_read_access.get_filter_string();

    let sessions_to_render = main_state_read_access.sessions.iter().filter(|session|session.filter_me(filter_string.as_str())). map(|session| {
        let bg_color = if odd {
            "--vz-table-active-bg"
        } else {
            "--vz-table-striped-bg"
        };

        odd = !odd;

        let session_type = match session.get_session_type() {
            SessionType::Tcp => {
                rsx! { span { class: "badge text-bg-success", "Tcp" } }
            }
            SessionType::Http => {
                rsx! {
                    span { span { class: "badge text-bg-warning", "Http" } }
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
                    span { class: "badge text-bg-success my-badge", publisher }
                }
    
            }else {
                rsx! {
                    span { class: "badge text-bg-light my-badge", publisher }
                }
                    
            }

        });


        let subscribers_to_render = subscribers.into_iter().map(|(topic, queue, active)|{
            if active>0{
                rsx! { span { class: "badge text-bg-success my-badge", "{topic}->{queue}" } }
    
            }else {
                rsx! { span { class: "badge text-bg-light my-badge", "{topic}->{queue}" } }
                    
            }
        });

        

        rsx! {
            tr { style: "--bg-color:var({bg_color}); background-color:var({bg_color}); vertical-align: top;border-bottom: 1px solid black;",
                td {
                    div { class: "info-line", "{session.id}" }
                    div { class: "info-line", session_type }
                }
                td {
                    div { class: "info-line-bold", "{session.name}" }

                    div { class: "info-line-xs",
                        b { "Client ver: " }
                        "{session.version}"
                    }

                    div { class: "info-line-xs",
                        b { "Ip: " }
                        "{session.ip}"
                    }

                    div { class: "info-line-xs",
                        b { "Connected: " }
                        "{session.connected}"
                    }
                    div { class: "info-line-xs",
                        b { "Read: " }
                        r_size
                    }
                    div { class: "info-line-xs",
                        b { "Written: " }
                        w_size
                    }
                    div { class: "info-line-xs",
                        b { "R/sec: " }
                        r_p_s
                    }
                    div { class: "info-line-xs",
                        b { "W/sec: " }
                        w_p_s
                    }
                }
                td { publishers_to_render }
                td { subscribers_to_render }
            }
        }
    });

    render! {
        table { class: "table table-dark", style: "text-align: left;margin: 0; padding: 0;",
            tr { style: "background: black; ",
                th { "Id" }
                th { "Info" }
                th { "Publisher" }

                th { "Subscriber" }
            }
            sessions_to_render
        }
    }
}
