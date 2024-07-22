use crate::settings::SettingsReader;

pub struct AppCtx {
    pub settings: SettingsReader,
}

impl AppCtx {
    pub fn new() -> Self {
        Self {
            settings: SettingsReader::new(),
        }
    }
}
