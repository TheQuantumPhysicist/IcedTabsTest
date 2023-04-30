#[derive(Debug, Clone)]
pub struct NodeBackendController {}

impl NodeBackendController {
    pub async fn initialize() -> anyhow::Result<NodeBackendController> {
        Ok(NodeBackendController {})
    }

    /// Triggers shutdown process synchronously
    /// Returns the subsystem manager join handle ONLY ONCE.
    /// If the shutdown was already triggered, returns None.
    pub fn trigger_shutdown(&mut self) {}
}

// TODO: tests, especially startup and shutdown
