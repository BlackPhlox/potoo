pub struct PotooServer;

impl PotooServer {
    /// Creates a new [`PotooServer`].
    pub fn new() -> PotooServer {
        PotooServer
    }
}

impl Default for PotooServer {
    fn default() -> Self {
        Self::new()
    }
}
