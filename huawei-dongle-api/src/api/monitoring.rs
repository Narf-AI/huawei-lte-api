//! Monitoring API endpoints

use crate::{
    client::Client,
    error::{Error, Result},
    models::monitoring::MonitoringStatus,
};
use tracing::{debug, trace};

/// Monitoring API for status and signal monitoring
pub struct MonitoringApi<'a> {
    client: &'a Client,
}

impl<'a> MonitoringApi<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn status(&self) -> Result<MonitoringStatus> {
        debug!("Fetching monitoring status");

        self.client.get_authenticated_with_retry("/api/monitoring/status", |text| {
            trace!("Monitoring status response: {}", text);
            let status: MonitoringStatus = serde_xml_rs::from_str(text)
                .map_err(|e| Error::generic(format!("Failed to parse monitoring status: {}", e)))?;

            debug!(
                "Monitoring status parsed: connection={}, network={}, signal={}",
                status.connection_status_text(),
                status.network_type_text(),
                status.signal_level().unwrap_or(0)
            );

            Ok(status)
        }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn test_monitoring_api_creation() {
        let config = Config::default();
        let client = crate::Client::new(config).unwrap();
        let monitoring_api = client.monitoring();

        assert_eq!(
            std::mem::size_of_val(&monitoring_api),
            std::mem::size_of::<&Client>()
        );
    }
}
