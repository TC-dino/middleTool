use crate::db::ConnectionConfig;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub connections: Vec<ConnectionConfig>,
    pub last_connection: Option<usize>,
    pub window_width: f32,
    pub window_height: f32,
    pub sidebar_width: f32,
}

impl AppConfig {
    pub fn config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("dbx");
        path.push("config.json");
        path
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Self {
                window_width: 1200.0,
                window_height: 800.0,
                sidebar_width: 250.0,
                ..Default::default()
            }
        }
    }

    pub fn save(&self) {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(content) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(path, content);
        }
    }

    pub fn add_connection(&mut self, config: ConnectionConfig) {
        self.connections.push(config);
        self.save();
    }

    pub fn remove_connection(&mut self, index: usize) {
        if index < self.connections.len() {
            self.connections.remove(index);
            self.save();
        }
    }
}
