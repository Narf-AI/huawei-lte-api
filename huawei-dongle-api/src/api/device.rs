//! Device API endpoints

use crate::{
    client::Client,
    error::{Error, Result},
    models::{common::Response, device::*},
};
use tracing::{debug, trace};

/// Device API for device information and control operations
pub struct DeviceApi<'a> {
    client: &'a Client,
}

impl<'a> DeviceApi<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn information(&self) -> Result<DeviceInformation> {
        debug!("Fetching device information");

        let response = self.client.get("/api/device/information").await?;
        let text = response.text().await?;

        trace!("Device information response: {}", text);

        self.client.check_xml_for_errors(&text).await?;

        let device_info: DeviceInformation = serde_xml_rs::from_str(&text)
            .map_err(|e| Error::generic(format!("Failed to parse device information: {}", e)))?;

        Ok(device_info)
    }

    pub async fn reboot(&self) -> Result<()> {
        debug!("Rebooting device");

        let request = DeviceControlRequest::reboot();
        let xml = serde_xml_rs::to_string(&request)
            .map_err(|e| Error::generic(format!("Failed to serialize reboot request: {}", e)))?;

        let response = self.client.post_xml("/api/device/control", &xml).await?;
        let text = response.text().await?;

        trace!("Device reboot response: {}", text);

        self.client.check_xml_for_errors(&text).await?;

        let result: Response = serde_xml_rs::from_str(&text)
            .map_err(|e| Error::generic(format!("Failed to parse reboot response: {}", e)))?;

        if !result.is_success() {
            return Err(Error::api(
                result.error_code().unwrap_or(-1),
                result
                    .error_message()
                    .unwrap_or("Device reboot failed")
                    .to_string(),
            ));
        }

        debug!("Device reboot initiated successfully");
        Ok(())
    }

    pub async fn power_off(&self) -> Result<()> {
        debug!("Powering off device");

        let request = DeviceControlRequest::power_off();
        let xml = serde_xml_rs::to_string(&request)
            .map_err(|e| Error::generic(format!("Failed to serialize power off request: {}", e)))?;

        let response = self.client.post_xml("/api/device/control", &xml).await?;
        let text = response.text().await?;

        trace!("Device power off response: {}", text);

        self.client.check_xml_for_errors(&text).await?;

        let result: Response = serde_xml_rs::from_str(&text)
            .map_err(|e| Error::generic(format!("Failed to parse power off response: {}", e)))?;

        if !result.is_success() {
            return Err(Error::api(
                result.error_code().unwrap_or(-1),
                result
                    .error_message()
                    .unwrap_or("Device power off failed")
                    .to_string(),
            ));
        }

        debug!("Device power off initiated successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_device_control_serialization() {
        let reboot_request = DeviceControlRequest::reboot();
        let xml = serde_xml_rs::to_string(&reboot_request).unwrap();

        assert!(xml.contains("<Control>1</Control>"));

        let power_off_request = DeviceControlRequest::power_off();
        let xml = serde_xml_rs::to_string(&power_off_request).unwrap();

        assert!(xml.contains("<Control>4</Control>"));
    }
}
