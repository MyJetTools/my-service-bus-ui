use std::rc::Rc;

use dioxus::prelude::EventHandler;

#[derive(Debug, Clone)]
pub enum DialogState {
    Confirmation {
        content: Rc<String>,
        on_ok: EventHandler<()>,
    },
}
