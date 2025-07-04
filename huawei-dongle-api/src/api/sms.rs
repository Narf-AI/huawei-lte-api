//! SMS API endpoints

use crate::{
    client::Client,
    error::{Error, Result},
    models::{common::Response, sms::*},
};
use tracing::{debug, trace};

/// SMS API for SMS management
pub struct SmsApi<'a> {
    client: &'a Client,
}

impl<'a> SmsApi<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn count(&self) -> Result<SmsCount> {
        debug!("Fetching SMS count");

        let response = self.client.get("/api/sms/sms-count").await?;
        let text = response.text().await?;

        trace!("SMS count response: {}", text);

        self.client.check_xml_for_errors(&text).await?;

        let count: SmsCount = serde_xml_rs::from_str(&text)
            .map_err(|e| Error::generic(format!("Failed to parse SMS count: {}", e)))?;

        debug!(
            "SMS count - Local unread: {}, SIM unread: {}, Total unread: {}",
            count.local_unread,
            count.sim_unread,
            count.total_unread().unwrap_or(0)
        );

        Ok(count)
    }

    pub async fn list(&self, request: &SmsListRequest) -> Result<SmsListResponse> {
        debug!(
            "Fetching SMS list - Page: {}, Count: {}, Box: {}",
            request.page_index, request.read_count, request.box_type
        );

        let xml = serde_xml_rs::to_string(request)
            .map_err(|e| Error::generic(format!("Failed to serialize SMS list request: {}", e)))?;

        self.client.post_xml_with_retry("/api/sms/sms-list", &xml, |text| {
            debug!("SMS list response XML: {}", text);
            let sms_list: SmsListResponse = serde_xml_rs::from_str(text)
                .map_err(|e| Error::generic(format!("Failed to parse SMS list: {}", e)))?;
            debug!(
                "Retrieved {} SMS messages",
                sms_list.messages.messages.len()
            );
            Ok(sms_list)
        }).await
    }

    pub async fn delete(&self, message_id: &str) -> Result<()> {
        debug!("Deleting SMS message with ID: {}", message_id);

        let request = SmsDeleteRequest::new(message_id);
        let xml = serde_xml_rs::to_string(&request).map_err(|e| {
            Error::generic(format!("Failed to serialize SMS delete request: {}", e))
        })?;

        let response = self.client.post_xml("/api/sms/delete-sms", &xml).await?;
        let text = response.text().await?;

        trace!("SMS delete response: {}", text);

        self.client.check_xml_for_errors(&text).await?;

        let result: Response = serde_xml_rs::from_str(&text)
            .map_err(|e| Error::generic(format!("Failed to parse SMS delete response: {}", e)))?;

        if !result.is_success() {
            return Err(Error::api(
                result.error_code().unwrap_or(-1),
                result
                    .error_message()
                    .unwrap_or("SMS deletion failed")
                    .to_string(),
            ));
        }

        debug!("SMS message deleted successfully");
        Ok(())
    }

    pub async fn mark_read(&self, message_id: &str) -> Result<()> {
        debug!("Marking SMS message as read: {}", message_id);

        let request = SmsSetReadRequest::new(message_id);
        let xml = serde_xml_rs::to_string(&request).map_err(|e| {
            Error::generic(format!("Failed to serialize SMS set read request: {}", e))
        })?;

        let response = self.client.post_xml("/api/sms/set-read", &xml).await?;
        let text = response.text().await?;

        trace!("SMS set read response: {}", text);

        self.client.check_xml_for_errors(&text).await?;

        let result: Response = serde_xml_rs::from_str(&text)
            .map_err(|e| Error::generic(format!("Failed to parse SMS set read response: {}", e)))?;

        if !result.is_success() {
            return Err(Error::api(
                result.error_code().unwrap_or(-1),
                result
                    .error_message()
                    .unwrap_or("SMS mark read failed")
                    .to_string(),
            ));
        }

        debug!("SMS message marked as read successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn test_sms_api_creation() {
        let config = Config::default();
        let client = crate::Client::new(config).unwrap();
        let sms_api = client.sms();

        assert_eq!(
            std::mem::size_of_val(&sms_api),
            std::mem::size_of::<&Client>()
        );
    }
}
