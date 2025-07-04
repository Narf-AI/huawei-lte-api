//! Network API endpoints

use crate::{
    client::Client,
    error::{Error, Result},
    models::{common::Response, network::*},
};
use tracing::{debug, trace};

/// Network API for network configuration and status
pub struct NetworkApi<'a> {
    client: &'a Client,
}

impl<'a> NetworkApi<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// This endpoint does not require authentication.
    /// Returns the current network mode, network bands, and LTE bands.
    pub async fn get_mode(&self) -> Result<NetworkMode> {
        debug!("Fetching network mode configuration");

        let response = self.client.get("/api/net/net-mode").await?;
        let text = response.text().await?;

        trace!("Network mode response: {}", text);

        self.client.check_xml_for_errors(&text).await?;

        let mode: NetworkMode = serde_xml_rs::from_str(&text)
            .map_err(|e| Error::generic(format!("Failed to parse network mode: {}", e)))?;

        debug!(
            "Current network mode: {} ({})",
            mode.network_mode,
            mode.mode_text()
        );
        Ok(mode)
    }

    /// This endpoint requires authentication and a valid CSRF token.
    /// **Warning**: This will temporarily disconnect the device while it reconnects.
    pub async fn set_mode(&self, request: &NetworkModeRequest) -> Result<()> {
        debug!(
            "Setting network mode to: {} ({})",
            request.network_mode,
            NetworkMode {
                network_mode: request.network_mode.clone(),
                network_band: request.network_band.clone(),
                lte_band: request.lte_band.clone(),
            }
            .mode_text()
        );

        let xml = serde_xml_rs::to_string(request).map_err(|e| {
            Error::generic(format!("Failed to serialize network mode request: {}", e))
        })?;

        let response = self.client.post_xml("/api/net/net-mode", &xml).await?;
        let text = response.text().await?;

        trace!("Network mode set response: {}", text);

        self.client.check_xml_for_errors(&text).await?;

        let result: Response = serde_xml_rs::from_str(&text)
            .map_err(|e| Error::generic(format!("Failed to parse network mode response: {}", e)))?;

        if !result.is_success() {
            return Err(Error::api(
                result.error_code().unwrap_or(-1),
                result
                    .error_message()
                    .unwrap_or("Network mode change failed")
                    .to_string(),
            ));
        }

        debug!("Network mode changed successfully");
        Ok(())
    }

    /// This endpoint does not require authentication.
    /// Returns information about the current cellular network operator.
    pub async fn current_plmn(&self) -> Result<CurrentPlmn> {
        debug!("Fetching current PLMN information");

        let response = self.client.get("/api/net/current-plmn").await?;
        let text = response.text().await?;

        trace!("Current PLMN response: {}", text);

        self.client.check_xml_for_errors(&text).await?;

        let plmn: CurrentPlmn = serde_xml_rs::from_str(&text)
            .map_err(|e| Error::generic(format!("Failed to parse PLMN information: {}", e)))?;

        if let Some(name) = plmn.operator_name() {
            debug!(
                "Current operator: {} ({})",
                name,
                plmn.numeric.as_deref().unwrap_or("unknown")
            );
        }

        Ok(plmn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn test_network_api_creation() {
        let config = Config::default();
        let client = crate::Client::new(config).unwrap();
        let network_api = client.network();

        assert_eq!(
            std::mem::size_of_val(&network_api),
            std::mem::size_of::<&Client>()
        );
    }
}
