use dioxus::prelude::*;
use dioxus_free_icons::{icons::bs_icons::BsPlugFill, Icon};

pub fn plug_icon(cx: Scope) -> Element {
    cx.render(rsx! { Icon { width: 10, height: 10, icon: BsPlugFill } })
}
