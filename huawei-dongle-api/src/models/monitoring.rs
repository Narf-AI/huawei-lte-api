//! Monitoring models for connection status and signal information

use serde::{Deserialize, Serialize};
use super::enums::{ConnectionStatus, NetworkType, SimStatus, RoamingStatus, ServiceStatus};

/// Connection status response from `/api/monitoring/status`.
/// 
/// Contains comprehensive information about the device's current state including
/// connection status, network type, signal strength, and service availability.
/// 
/// # Example
/// 
/// ```no_run
/// # use huawei_dongle_api::{Client, Config};
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new(Config::default())?;
/// let status = client.monitoring().status().await?;
/// 
/// if status.is_connected() {
///     println!("Connected to {}", status.network_type_text());
///     println!("Signal: {}/5", status.signal_level().unwrap_or(0));
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "response")]
pub struct MonitoringStatus {
    #[serde(rename = "ConnectionStatus")]
    pub connection_status: ConnectionStatus,

    #[serde(rename = "WifiConnectionStatus")]
    pub wifi_connection_status: Option<String>,

    #[serde(rename = "SignalStrength")]
    pub signal_strength: Option<String>,

    #[serde(rename = "SignalIcon")]
    pub signal_icon: Option<String>,

    #[serde(rename = "CurrentNetworkType")]
    pub current_network_type: NetworkType,

    #[serde(rename = "CurrentServiceDomain")]
    pub current_service_domain: Option<String>,

    #[serde(rename = "RoamingStatus")]
    pub roaming_status: RoamingStatus,

    #[serde(rename = "BatteryStatus")]
    pub battery_status: Option<String>,

    #[serde(rename = "BatteryLevel")]
    pub battery_level: Option<String>,

    #[serde(rename = "BatteryPercent")]
    pub battery_percent: Option<String>,

    #[serde(rename = "simlockStatus")]
    pub simlock_status: String,

    #[serde(rename = "PrimaryDns")]
    pub primary_dns: Option<String>,

    #[serde(rename = "SecondaryDns")]
    pub secondary_dns: Option<String>,

    #[serde(rename = "wififrequence")]
    pub wifi_frequency: Option<String>,

    #[serde(rename = "flymode")]
    pub fly_mode: String,

    #[serde(rename = "PrimaryIPv6Dns")]
    pub primary_ipv6_dns: Option<String>,

    #[serde(rename = "SecondaryIPv6Dns")]
    pub secondary_ipv6_dns: Option<String>,

    #[serde(rename = "CurrentWifiUser")]
    pub current_wifi_user: Option<String>,

    #[serde(rename = "TotalWifiUser")]
    pub total_wifi_user: Option<String>,

    #[serde(rename = "currenttotalwifiuser")]
    pub current_total_wifi_user: String,

    #[serde(rename = "ServiceStatus")]
    pub service_status: ServiceStatus,

    #[serde(rename = "SimStatus")]
    pub sim_status: SimStatus,

    #[serde(rename = "WifiStatus")]
    pub wifi_status: Option<String>,

    #[serde(rename = "CurrentNetworkTypeEx")]
    pub current_network_type_ex: Option<NetworkType>,

    #[serde(rename = "maxsignal")]
    pub max_signal: String,

    #[serde(rename = "wifiindooronly")]
    pub wifi_indoor_only: String,

    #[serde(rename = "classify")]
    pub classify: Option<String>,

    #[serde(rename = "usbup")]
    pub usb_up: String,

    #[serde(rename = "wifiswitchstatus")]
    pub wifi_switch_status: String,

    #[serde(rename = "WifiStatusExCustom")]
    pub wifi_status_ex_custom: Option<String>,

    #[serde(rename = "hvdcp_online")]
    pub hvdcp_online: Option<String>,

    #[serde(rename = "speedLimitStatus")]
    pub speed_limit_status: Option<String>,

    #[serde(rename = "poorSignalStatus")]
    pub poor_signal_status: Option<String>,
}

impl MonitoringStatus {
    pub fn is_connected(&self) -> bool {
        self.connection_status.is_connected()
    }

    /// Get connection status as human-readable string
    pub fn connection_status_text(&self) -> String {
        self.connection_status.to_string()
    }

    /// Get network type as human-readable string
    pub fn network_type_text(&self) -> String {
        self.current_network_type.to_string()
    }

    /// Get extended network type as human-readable string
    pub fn network_type_ex_text(&self) -> String {
        match &self.current_network_type_ex {
            Some(network_type) => network_type.extended_text().to_string(),
            None => "N/A".to_string(),
        }
    }

    pub fn is_sim_ready(&self) -> bool {
        self.sim_status.is_ready()
    }

    pub fn is_roaming(&self) -> bool {
        self.roaming_status.is_roaming()
    }

    /// Get signal strength level (0-5)
    pub fn signal_level(&self) -> Option<u8> {
        self.signal_icon.as_ref().and_then(|s| s.parse().ok())
    }

    /// Get signal strength as percentage (0-100%)
    pub fn signal_percentage(&self) -> Option<u8> {
        self.signal_level().map(|level| match level {
            0 => 0,
            1 => 20,
            2 => 40,
            3 => 60,
            4 => 80,
            5 => 100,
            _ => 0,
        })
    }

    pub fn is_service_available(&self) -> bool {
        self.service_status.is_available()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_status_parsing() {
        let status = MonitoringStatus {
            connection_status: ConnectionStatus::Connected,
            current_network_type: NetworkType::Lte,
            signal_icon: Some("5".to_string()),
            sim_status: SimStatus::Ready,
            roaming_status: RoamingStatus::NotRoaming,
            service_status: ServiceStatus::FullService,
                wifi_connection_status: None,
            signal_strength: None,
            current_service_domain: None,
            battery_status: None,
            battery_level: None,
            battery_percent: None,
            simlock_status: "0".to_string(),
            primary_dns: None,
            secondary_dns: None,
            wifi_frequency: None,
            fly_mode: "0".to_string(),
            primary_ipv6_dns: None,
            secondary_ipv6_dns: None,
            current_wifi_user: None,
            total_wifi_user: None,
            current_total_wifi_user: "0".to_string(),
            wifi_status: None,
            current_network_type_ex: Some(NetworkType::FiveGNsa),
            max_signal: "5".to_string(),
            wifi_indoor_only: "0".to_string(),
            classify: Some("hilink".to_string()),
            usb_up: "0".to_string(),
            wifi_switch_status: "0".to_string(),
            wifi_status_ex_custom: None,
            hvdcp_online: None,
            speed_limit_status: None,
            poor_signal_status: None,
        };

        assert_eq!(status.connection_status, ConnectionStatus::Connected);
        assert_eq!(status.connection_status_text(), "CONNECTED");
        assert_eq!(status.network_type_text(), "LTE (4G)");
        assert_eq!(status.network_type_ex_text(), "5G Non-Standalone");
        assert!(status.is_connected());
        assert!(status.is_sim_ready());
        assert!(!status.is_roaming());
        assert_eq!(status.signal_level(), Some(5));
        assert_eq!(status.signal_percentage(), Some(100));
        assert!(status.is_service_available());
    }
}
