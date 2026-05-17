#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleState {
    Idle,
    Loading,
    DomContentLoaded,
    Loaded,
    NetworkIdle,
    Failed,
}

impl LifecycleState {
    pub fn is_loading(&self) -> bool {
        matches!(self, LifecycleState::Loading)
    }

    pub fn is_loaded(&self) -> bool {
        matches!(self, LifecycleState::Loaded | LifecycleState::NetworkIdle)
    }

    pub fn is_network_idle(&self) -> bool {
        matches!(self, LifecycleState::NetworkIdle)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaitUntil {
    Load,
    DomContentLoaded,
    NetworkIdle0,
    NetworkIdle2,
}

impl WaitUntil {
    pub fn from_str(s: &str) -> Self {
        match s {
            "domcontentloaded" => WaitUntil::DomContentLoaded,
            "networkidle0" | "networkIdle" | "networkidle" => WaitUntil::NetworkIdle0,
            "networkidle2" => WaitUntil::NetworkIdle2,
            _ => WaitUntil::Load,
        }
    }
}
