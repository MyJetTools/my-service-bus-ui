use dioxus::prelude::*;

use crate::MainState;
#[component]
pub fn DialogTemplate(header: String, content: Element, ok_button: Element) -> Element {
    rsx! {
        div { id: "dialog-window",
            div { id: "dialog-header",
                table { style: "width:100%",
                    tr {
                        td {

                            h2 { {header} }
                        }
                        td { style: "vertical-align:top;text-align:right;cursor:pointer",
                            div {
                                onclick: move |_| {
                                    let mut main_state = consume_context::<Signal<MainState>>();
                                    main_state.write().hide_dialog();
                                },
                                "X"
                            }
                        }
                    }
                }
            }

            div { {content} }

            hr {}

            div { style: "text-align:right",

                div { class: "btn-group",
                    {ok_button}
                    button {
                        class: "btn btn-outline-primary",
                        onclick: move |_| {
                            let mut main_state = consume_context::<Signal<MainState>>();
                            main_state.write().hide_dialog();
                        },
                        "Cancel"
                    }
                }
            }
        }
    }
}
