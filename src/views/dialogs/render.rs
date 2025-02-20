use dioxus::prelude::*;

use crate::{dialogs::confirmation_dialog::ConfirmationDialog, DialogState, MainState};

#[component]
pub fn RenderDialog() -> Element {
    let main_state = consume_context::<Signal<MainState>>();
    let dialog_state = main_state.read().dialog.clone();

    let content = if let Some(dialog_state) = dialog_state {
        match dialog_state {
            DialogState::Confirmation { content, on_ok } => {
                rsx! {
                    ConfirmationDialog { content, on_ok }
                }
            }
        }
    } else {
        rsx! {
            div {}
        }
    };

    rsx! {
        div { id: "dialog-background", {content} }
    }
}
