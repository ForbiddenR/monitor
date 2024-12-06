use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Data {
    #[serde(rename = "State")]
    pub state: HostState,
    #[serde(rename = "Host")]
    pub host: Host,
    #[serde(rename = "Timestamp")]
    pub timestamp: i64,
}

impl Data {
    pub fn new() -> Self {
        Self {
            host: Host::default(),
            state: HostState::default(),
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    pub fn add_host_state(mut self, state: HostState) -> Self {
        self.state = state;
        self
    }

    pub fn add_host(mut self, host: Host) -> Self {
        self.host = host;
        self
    }
}

#[derive(Debug, Serialize, Default)]
pub struct HostState {
    #[serde(rename = "CPU")]
    pub cpu: f64,
    #[serde(rename = "MemUsed")]
    pub mem_used: u64,
    #[serde(rename = "SwapUsed")]
    pub swap_used: u64,
}

#[derive(Debug, Serialize, Default, Clone)]
pub struct Host {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "MemTotal")]
    pub mem_total: u64,
    #[serde(rename = "SwapTotal")]
    pub swap_total: u64,
    #[serde(rename = "Arch")]
    pub arch: String,
}
