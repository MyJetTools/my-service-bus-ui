pub struct SettingsModel;

impl SettingsModel {
    pub fn get_api(&self) -> String {
        read_env_variable("SB_API_URL")
    }

    pub fn get_persistence_api(&self) -> String {
        read_env_variable("SB_PERSISTENCE_API_URL")
    }
}

fn read_env_variable(name: &str) -> String {
    match std::env::var(name) {
        Ok(url) => return url,
        Err(_) => panic!("{} is not set", name),
    }
}
