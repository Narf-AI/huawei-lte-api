//! CLI configuration management

use serde::{Deserialize, Serialize};

/// CLI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliConfig {
    /// Default device URL
    pub default_url: Option<String>,
    /// Default timeout in seconds
    pub default_timeout: Option<u64>,
    /// Default output format
    pub default_format: Option<String>,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            default_url: Some("http://192.168.8.1".to_string()),
            default_timeout: Some(30),
            default_format: Some("table".to_string()),
        }
    }
}

