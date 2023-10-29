use dioxus::prelude::*;

#[inline_props]
pub fn render_message_id(cx: Scope, message_id: i64) -> Element {
    let as_str = message_id.to_string();

    if as_str.len() < 5 {
        return render! {as_str};
    }

    render! {
        span { style: "color:gray", &as_str[..as_str.len()-5] }
        &as_str[as_str.len()-5..]
    }
}
