//! Network configuration models

use serde::{Deserialize, Serialize};
use super::enums::{NetworkModeType, NetworkType};

/// Network mode configuration response from `/api/net/net-mode`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "response")]
pub struct NetworkMode {
    #[serde(rename = "NetworkMode")]
    pub network_mode: NetworkModeType,

    #[serde(rename = "NetworkBand")]
    pub network_band: String,

    #[serde(rename = "LTEBand")]
    pub lte_band: String,
}

/// Network mode configuration request for `/api/net/net-mode`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "request")]
pub struct NetworkModeRequest {
    #[serde(rename = "NetworkMode")]
    pub network_mode: NetworkModeType,

    #[serde(rename = "NetworkBand")]
    pub network_band: String,

    #[serde(rename = "LTEBand")]
    pub lte_band: String,
}

/// Current PLMN (network operator) information from `/api/net/current-plmn`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "response")]
pub struct CurrentPlmn {
    #[serde(rename = "State")]
    pub state: String,

    #[serde(rename = "FullName")]
    pub full_name: Option<String>,

    #[serde(rename = "ShortName")]
    pub short_name: Option<String>,

    #[serde(rename = "Numeric")]
    pub numeric: Option<String>,

    #[serde(rename = "Rat")]
    pub rat: Option<NetworkType>,
}

impl NetworkMode {
    /// Get network mode as human-readable string
    pub fn mode_text(&self) -> String {
        self.network_mode.to_string()
    }

    /// Check if current mode is 4G only
    pub fn is_4g_only(&self) -> bool {
        matches!(self.network_mode, NetworkModeType::FourGOnly)
    }

    /// Check if current mode is auto
    pub fn is_auto(&self) -> bool {
        matches!(self.network_mode, NetworkModeType::Auto)
    }
}

impl NetworkModeRequest {
    /// Create a new network mode request
    pub fn new(mode: NetworkModeType, network_band: String, lte_band: String) -> Self {
        Self {
            network_mode: mode,
            network_band,
            lte_band,
        }
    }

    /// Create a 4G only mode request with common bands
    pub fn lte_only() -> Self {
        Self::new(
            NetworkModeType::FourGOnly,
            "3fffffff".to_string(), // All 2G/3G bands
            "80800C5".to_string(),  // Common LTE bands
        )
    }

    /// Create a 4G preferred with 3G fallback request
    pub fn lte_preferred() -> Self {
        Self::new(
            NetworkModeType::FourGPreferredThreeGFallback,
            "3fffffff".to_string(),
            "80800C5".to_string(),
        )
    }

    /// Create an auto mode request
    pub fn auto() -> Self {
        Self::new(
            NetworkModeType::Auto,
            "3fffffff".to_string(),
            "80800C5".to_string(),
        )
    }
}

impl CurrentPlmn {
    /// Get operator name (full name if available, otherwise short name)
    pub fn operator_name(&self) -> Option<&str> {
        self.full_name.as_deref().or(self.short_name.as_deref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_mode_text() {
        let mode = NetworkMode {
            network_mode: NetworkModeType::FourGOnly,
            network_band: "3fffffff".to_string(),
            lte_band: "80800C5".to_string(),
        };

        assert_eq!(mode.mode_text(), "4G Only (LTE)");
        assert!(mode.is_4g_only());
        assert!(!mode.is_auto());
    }

    #[test]
    fn test_request_creation() {
        let request = NetworkModeRequest::lte_only();
        assert_eq!(request.network_mode, NetworkModeType::FourGOnly);
        assert_eq!(request.network_band, "3fffffff");
        assert_eq!(request.lte_band, "80800C5");
    }
}
