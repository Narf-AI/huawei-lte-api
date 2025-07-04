//! DHCP configuration models

use serde::{Deserialize, Serialize};
use super::{DhcpStatus, DnsStatus};

/// DHCP settings response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DhcpSettings {
    /// DNS status (1=enabled, 0=disabled)
    #[serde(rename = "DnsStatus")]
    pub dns_status: DnsStatus,
    
    /// DHCP pool start IP address
    #[serde(rename = "DhcpStartIPAddress")]
    pub dhcp_start_ip_address: String,
    
    /// DHCP gateway IP address
    #[serde(rename = "DhcpIPAddress")]
    pub dhcp_ip_address: String,
    
    /// DHCP server status (1=enabled, 0=disabled)
    #[serde(rename = "DhcpStatus")]
    pub dhcp_status: DhcpStatus,
    
    /// DHCP subnet mask
    #[serde(rename = "DhcpLanNetmask")]
    pub dhcp_lan_netmask: String,
    
    /// Secondary DNS server
    #[serde(rename = "SecondaryDns")]
    pub secondary_dns: String,
    
    /// Primary DNS server
    #[serde(rename = "PrimaryDns")]
    pub primary_dns: String,
    
    /// DHCP pool end IP address
    #[serde(rename = "DhcpEndIPAddress")]
    pub dhcp_end_ip_address: String,
    
    /// DHCP lease time in seconds
    #[serde(rename = "DhcpLeaseTime")]
    pub dhcp_lease_time: String,
}

/// DHCP settings request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DhcpSettingsRequest {
    /// DHCP gateway IP address
    #[serde(rename = "DhcpIPAddress")]
    pub dhcp_ip_address: String,
    
    /// DHCP subnet mask
    #[serde(rename = "DhcpLanNetmask")]
    pub dhcp_lan_netmask: String,
    
    /// DHCP server status (1=enabled, 0=disabled)
    #[serde(rename = "DhcpStatus")]
    pub dhcp_status: DhcpStatus,
    
    /// DHCP pool start IP address
    #[serde(rename = "DhcpStartIPAddress")]
    pub dhcp_start_ip_address: String,
    
    /// DHCP pool end IP address
    #[serde(rename = "DhcpEndIPAddress")]
    pub dhcp_end_ip_address: String,
    
    /// DHCP lease time in seconds
    #[serde(rename = "DhcpLeaseTime")]
    pub dhcp_lease_time: String,
    
    /// DNS status (1=enabled, 0=disabled)
    #[serde(rename = "DnsStatus")]
    pub dns_status: DnsStatus,
    
    /// Primary DNS server
    #[serde(rename = "PrimaryDns")]
    pub primary_dns: String,
    
    /// Secondary DNS server
    #[serde(rename = "SecondaryDns")]
    pub secondary_dns: String,
}

impl DhcpSettingsRequest {
    /// Create a new DHCP settings request
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        dhcp_ip_address: String,
        dhcp_lan_netmask: String,
        dhcp_status: DhcpStatus,
        dhcp_start_ip_address: String,
        dhcp_end_ip_address: String,
        dhcp_lease_time: String,
        dns_status: DnsStatus,
        primary_dns: String,
        secondary_dns: String,
    ) -> Self {
        Self {
            dhcp_ip_address,
            dhcp_lan_netmask,
            dhcp_status,
            dhcp_start_ip_address,
            dhcp_end_ip_address,
            dhcp_lease_time,
            dns_status,
            primary_dns,
            secondary_dns,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dhcp_settings_request_creation() {
        let request = DhcpSettingsRequest::new(
            "192.168.62.1".to_string(),
            "255.255.255.0".to_string(),
            DhcpStatus::Enabled,
            "192.168.62.100".to_string(),
            "192.168.62.200".to_string(),
            "86400".to_string(),
            DnsStatus::Enabled,
            "192.168.62.1".to_string(),
            "192.168.62.1".to_string(),
        );

        assert_eq!(request.dhcp_ip_address, "192.168.62.1");
        assert_eq!(request.dhcp_status, DhcpStatus::Enabled);
        assert_eq!(request.dhcp_lease_time, "86400");
    }

    #[test]
    fn test_dhcp_settings_serialization() {
        let request = DhcpSettingsRequest::new(
            "192.168.8.1".to_string(),
            "255.255.255.0".to_string(),
            DhcpStatus::Enabled,
            "192.168.8.100".to_string(),
            "192.168.8.200".to_string(),
            "86400".to_string(),
            DnsStatus::Enabled,
            "192.168.8.1".to_string(),
            "192.168.8.1".to_string(),
        );

        let xml = serde_xml_rs::to_string(&request).unwrap();
        assert!(xml.contains("<DhcpIPAddress>192.168.8.1</DhcpIPAddress>"));
        assert!(xml.contains("<DhcpStatus>1</DhcpStatus>"));
    }
}