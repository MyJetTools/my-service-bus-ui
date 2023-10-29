use crate::settings::SettingsModel;

pub struct AppCtx {
    pub settings: SettingsModel,
}

impl AppCtx {
    pub fn new() -> Self {
        Self {
            settings: SettingsModel,
        }
    }
}
