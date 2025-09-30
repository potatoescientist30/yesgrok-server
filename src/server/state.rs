use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

pub type AgentUrl = String;
pub type WildcardUrl = String;

#[derive(Clone)]
pub struct AppState {
    pub routes: Arc<RwLock<HashMap<WildcardUrl, AgentUrl>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            routes: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
