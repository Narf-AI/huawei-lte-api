//! Device information models

use serde::{Deserialize, Serialize};
use super::enums::DeviceControlType;

/// Device information response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "response")]
pub struct DeviceInformation {
    #[serde(rename = "DeviceName")]
    pub device_name: String,

    #[serde(rename = "SerialNumber")]
    pub serial_number: String,

    #[serde(rename = "Imei")]
    pub imei: String,

    #[serde(rename = "Imsi")]
    pub imsi: Option<String>,

    #[serde(rename = "Iccid")]
    pub iccid: Option<String>,

    #[serde(rename = "Msisdn")]
    pub msisdn: Option<String>,

    #[serde(rename = "HardwareVersion")]
    pub hardware_version: String,

    #[serde(rename = "SoftwareVersion")]
    pub software_version: String,

    #[serde(rename = "WebUIVersion")]
    pub webui_version: Option<String>,

    #[serde(rename = "MacAddress1")]
    pub mac_address1: Option<String>,

    #[serde(rename = "MacAddress2")]
    pub mac_address2: Option<String>,

    #[serde(rename = "ProductFamily")]
    pub product_family: Option<String>,

    #[serde(rename = "Classify")]
    pub classify: Option<String>,

    #[serde(rename = "supportmode")]
    pub support_mode: Option<String>,

    #[serde(rename = "workmode")]
    pub work_mode: Option<String>,
}

/// Device control request for operations like reboot
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "request")]
pub struct DeviceControlRequest {
    #[serde(rename = "Control")]
    pub control: DeviceControlType,
}

impl DeviceControlRequest {
    /// Create a reboot request
    pub fn reboot() -> Self {
        Self { control: DeviceControlType::Reboot }
    }

    /// Create a power off request  
    pub fn power_off() -> Self {
        Self { control: DeviceControlType::PowerOff }
    }

    /// Create a factory reset request
    pub fn factory_reset() -> Self {
        Self { control: DeviceControlType::FactoryReset }
    }

    /// Create a backup configuration request
    pub fn backup_configuration() -> Self {
        Self { control: DeviceControlType::BackupConfiguration }
    }
}
