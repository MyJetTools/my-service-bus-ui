#[derive(Debug, Clone)]
pub struct StatusBarState {
    pub connected: bool,
    pub mem_used: i64,
    pub mem_total: i64,
    pub version: String,
    pub persistence_ver: String,
}

impl StatusBarState {
    pub fn new() -> Self {
        Self {
            connected: false,
            mem_used: 0,
            mem_total: 0,
            persistence_ver: "???".to_string(),
            version: "???".to_string(),
        }
    }

    pub fn disconnected(&mut self) {
        self.connected = false;
    }
}
