use dioxus::prelude::*;

#[component]
pub fn PlugIcon() -> Element {
    rsx! {
        span { style: "margin-right:3px",
            img { src: "/assets/img/plug.svg", style: "width:8px" }
        }
    }
}
