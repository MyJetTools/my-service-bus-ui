pub struct StatusBarState {
    pub connected: bool,
    pub persist_queue: i64,
    pub rw_per_seq: i64,
    pub mem_used: i64,
    pub mem_total: i64,
    pub total_pages_size: i64,
    pub persistence_ver: String,
    pub started: bool,
}

impl StatusBarState {
    pub fn new() -> Self {
        Self {
            connected: false,
            persist_queue: 0,
            rw_per_seq: 0,
            mem_used: 0,
            mem_total: 0,
            total_pages_size: 0,
            persistence_ver: "???".to_string(),
            started: false,
        }
    }

    pub fn disconnected(&mut self) {
        self.connected = false;
    }
}
