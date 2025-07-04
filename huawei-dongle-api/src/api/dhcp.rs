//! DHCP API endpoints

use crate::{
    client::Client,
    error::{Error, Result},
    models::{common::Response, dhcp::*},
};
use tracing::{debug, trace};

/// DHCP API for DHCP configuration management
pub struct DhcpApi<'a> {
    client: &'a Client,
}

impl<'a> DhcpApi<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// This endpoint requires authentication and a valid session.
    pub async fn settings(&self) -> Result<DhcpSettings> {
        debug!("Fetching DHCP settings");

        let response = self.client.get("/api/dhcp/settings").await?;
        let text = response.text().await?;

        trace!("DHCP settings response: {}", text);

        self.client.check_xml_for_errors(&text).await?;

        let settings: DhcpSettings = serde_xml_rs::from_str(&text)
            .map_err(|e| Error::generic(format!("Failed to parse DHCP settings: {}", e)))?;

        debug!("DHCP gateway IP: {}", settings.dhcp_ip_address);
        Ok(settings)
    }

    /// This endpoint requires authentication and a valid CSRF token.
    /// **Warning**: This will change the device's network configuration and may temporarily disconnect clients.
    pub async fn set_settings(&self, request: &DhcpSettingsRequest) -> Result<()> {
        debug!(
            "Setting DHCP gateway IP to: {}",
            request.dhcp_ip_address
        );

        let xml = serde_xml_rs::to_string(request).map_err(|e| {
            Error::generic(format!("Failed to serialize DHCP settings request: {}", e))
        })?;

        let response = self.client.post_xml("/api/dhcp/settings", &xml).await?;
        let text = response.text().await?;

        trace!("DHCP settings response: {}", text);

        self.client.check_xml_for_errors(&text).await?;

        let result: Response = serde_xml_rs::from_str(&text)
            .map_err(|e| Error::generic(format!("Failed to parse DHCP settings response: {}", e)))?;

        if !result.is_success() {
            return Err(Error::api(
                result.error_code().unwrap_or(-1),
                result
                    .error_message()
                    .unwrap_or("DHCP settings change failed")
                    .to_string(),
            ));
        }

        debug!("DHCP settings changed successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn test_dhcp_api_creation() {
        let config = Config::default();
        let client = crate::Client::new(config).unwrap();
        let dhcp_api = client.dhcp();

        assert_eq!(
            std::mem::size_of_val(&dhcp_api),
            std::mem::size_of::<&Client>()
        );
    }
}