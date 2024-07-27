use my_settings_reader::SettingsReader;

use crate::settings::SettingsModel;

pub struct AppCtx {
    pub settings: SettingsReader<SettingsModel>,
}

impl AppCtx {
    pub fn new() -> Self {
        Self {
            settings: SettingsReader::new("~/.my-sb-ui"),
        }
    }
}
