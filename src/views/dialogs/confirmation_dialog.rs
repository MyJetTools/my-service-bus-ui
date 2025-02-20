use dioxus::prelude::*;

use crate::dialogs::dialog_template::DialogTemplate;

#[component]
pub fn ConfirmationDialog(content: String, on_ok: EventHandler<()>) -> Element {
    rsx! {
        DialogTemplate {
            header: "Confirmation".to_string(),
            content: rsx! {
                {content}
            },
            ok_button: rsx! {
                button { class: "btn btn-success", onclick: move |_| on_ok.call(()), "Save" }
            }
        }
    }
}
