//! SMS management models

use serde::{Deserialize, Serialize};
use super::enums::{SmsStatus, SmsPriority, SmsType, SmsBoxType, SmsSortType};

/// SMS count response from `/api/sms/sms-count`.
/// 
/// Provides message counts for both local storage and SIM card storage,
/// broken down by message type (inbox, outbox, draft).
/// 
/// # Example
/// 
/// ```no_run
/// # use huawei_dongle_api::{Client, Config};
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new(Config::default())?;
/// let count = client.sms().count().await?;
/// 
/// println!("Total unread: {}", count.total_unread().unwrap_or(0));
/// println!("New messages: {}", count.has_new_messages());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "response")]
pub struct SmsCount {
    #[serde(rename = "LocalUnread")]
    pub local_unread: String,

    #[serde(rename = "LocalInbox")]
    pub local_inbox: String,

    #[serde(rename = "LocalOutbox")]
    pub local_outbox: String,

    #[serde(rename = "LocalDraft")]
    pub local_draft: String,

    #[serde(rename = "SimUnread")]
    pub sim_unread: String,

    #[serde(rename = "SimInbox")]
    pub sim_inbox: String,

    #[serde(rename = "SimOutbox")]
    pub sim_outbox: String,

    #[serde(rename = "SimDraft")]
    pub sim_draft: String,

    #[serde(rename = "NewMsg")]
    pub new_msg: String,
}

/// SMS list request for `/api/sms/sms-list`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "request")]
pub struct SmsListRequest {
    #[serde(rename = "PageIndex")]
    pub page_index: String,

    #[serde(rename = "ReadCount")]
    pub read_count: String,

    #[serde(rename = "BoxType")]
    pub box_type: String,

    #[serde(rename = "SortType")]
    pub sort_type: String,

    #[serde(rename = "Ascending")]
    pub ascending: String,

    #[serde(rename = "UnreadPreferred")]
    pub unread_preferred: String,
}

/// SMS message from `/api/sms/sms-list` response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "Message")]
pub struct SmsMessage {
    #[serde(rename = "Smstat")]
    pub status: SmsStatus,

    #[serde(rename = "Index")]
    pub index: String,

    #[serde(rename = "Phone")]
    pub phone: String,

    #[serde(rename = "Content")]
    pub content: String,

    #[serde(rename = "Date")]
    pub date: String,

    #[serde(rename = "Sca")]
    pub sca: Option<String>,

    #[serde(rename = "SaveType")]
    pub save_type: String,

    #[serde(rename = "Priority")]
    pub priority: SmsPriority,

    #[serde(rename = "SmsType")]
    pub sms_type: SmsType,
}

/// Messages container from SMS list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmsMessages {
    #[serde(rename = "$value", default)]
    pub messages: Vec<SmsMessage>,
}

/// SMS list response from `/api/sms/sms-list`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "response")]
pub struct SmsListResponse {
    #[serde(rename = "Count", default)]
    pub count: Option<String>,

    #[serde(rename = "Messages")]
    pub messages: SmsMessages,
}

impl SmsListResponse {
    /// Get the message count, either from the Count field or by counting messages
    pub fn message_count(&self) -> usize {
        if let Some(count_str) = &self.count {
            count_str.parse().unwrap_or_else(|_| self.messages.messages.len())
        } else {
            self.messages.messages.len()
        }
    }
}

/// SMS delete request for `/api/sms/delete-sms`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "request")]
pub struct SmsDeleteRequest {
    #[serde(rename = "Index")]
    pub index: String,
}

/// SMS set read request for `/api/sms/set-read`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "request")]
pub struct SmsSetReadRequest {
    #[serde(rename = "Index")]
    pub index: String,
}


impl SmsCount {
    /// Get total unread messages count
    pub fn total_unread(&self) -> Result<u32, std::num::ParseIntError> {
        let local: u32 = self.local_unread.parse()?;
        let sim: u32 = self.sim_unread.parse()?;
        Ok(local + sim)
    }

    /// Get total inbox messages count
    pub fn total_inbox(&self) -> Result<u32, std::num::ParseIntError> {
        let local: u32 = self.local_inbox.parse()?;
        let sim: u32 = self.sim_inbox.parse()?;
        Ok(local + sim)
    }

    /// Check if there are new messages
    pub fn has_new_messages(&self) -> bool {
        self.new_msg.parse::<u32>().unwrap_or(0) > 0
    }
}

impl SmsListRequest {
    /// Create a new SMS list request
    pub fn new(
        page_index: u32,
        read_count: u32,
        box_type: SmsBoxType,
        sort_type: SmsSortType,
        ascending: bool,
        unread_preferred: bool,
    ) -> Self {
        Self {
            page_index: page_index.to_string(),
            read_count: read_count.to_string(),
            box_type: box_type.to_string(),
            sort_type: sort_type.to_string(),
            ascending: if ascending { "1" } else { "0" }.to_string(),
            unread_preferred: if unread_preferred { "1" } else { "0" }.to_string(),
        }
    }
}

impl SmsMessage {
    /// Check if message is unread
    pub fn is_unread(&self) -> bool {
        self.status.is_unread()
    }

    /// Check if message is read
    pub fn is_read(&self) -> bool {
        self.status.is_read()
    }

    /// Get message ID for deletion
    pub fn id(&self) -> &str {
        &self.index
    }

    /// Get formatted phone number
    pub fn phone_number(&self) -> &str {
        &self.phone
    }

    /// Get message text content
    pub fn text(&self) -> &str {
        &self.content
    }

    /// Get formatted date
    pub fn date_str(&self) -> &str {
        &self.date
    }
}

impl SmsDeleteRequest {
    /// Create a new delete request
    pub fn new(message_id: &str) -> Self {
        Self {
            index: message_id.to_string(),
        }
    }
}

impl SmsSetReadRequest {
    /// Create a new set read request
    pub fn new(message_id: &str) -> Self {
        Self {
            index: message_id.to_string(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sms_count_totals() {
        let count = SmsCount {
            local_unread: "3".to_string(),
            local_inbox: "10".to_string(),
            local_outbox: "5".to_string(),
            local_draft: "1".to_string(),
            sim_unread: "2".to_string(),
            sim_inbox: "8".to_string(),
            sim_outbox: "3".to_string(),
            sim_draft: "0".to_string(),
            new_msg: "1".to_string(),
        };

        assert_eq!(count.total_unread().unwrap(), 5);
        assert_eq!(count.total_inbox().unwrap(), 18);
        assert!(count.has_new_messages());
    }

    #[test]
    fn test_sms_message_status() {
        let unread = SmsMessage {
            status: SmsStatus::Unread,
            index: "1".to_string(),
            phone: "+1234567890".to_string(),
            content: "Test message".to_string(),
            date: "2024-01-01 12:00:00".to_string(),
            sca: None,
            save_type: "3".to_string(),
            priority: SmsPriority::Normal,
            sms_type: SmsType::Single,
        };

        assert!(unread.is_unread());
        assert!(!unread.is_read());
        assert_eq!(unread.id(), "1");
        assert_eq!(unread.text(), "Test message");
    }

    #[test]
    fn test_sms_list_request_creation() {
        let request = SmsListRequest::new(
            1,
            20,
            SmsBoxType::LocalInbox,
            SmsSortType::ByTime,
            false,
            true,
        );

        assert_eq!(request.page_index, "1");
        assert_eq!(request.read_count, "20");
        assert_eq!(request.box_type, "1"); // LocalInbox
        assert_eq!(request.sort_type, "0"); // ByTime
        assert_eq!(request.ascending, "0");
        assert_eq!(request.unread_preferred, "1"); // unread preferred
    }

    #[test]
    fn test_sms_list_response_missing_count() {
        let xml_without_count = r#"<response>
    <Messages>
        <Message>
            <Smstat>0</Smstat>
            <Index>1</Index>
            <Phone>+123456789</Phone>
            <Content>Test message</Content>
            <Date>2023-01-01 12:00:00</Date>
            <Sca></Sca>
            <SaveType>0</SaveType>
            <Priority>0</Priority>
            <SmsType>1</SmsType>
        </Message>
    </Messages>
</response>"#;

        let response: SmsListResponse = serde_xml_rs::from_str(xml_without_count).unwrap();
        assert!(response.count.is_none());
        assert_eq!(response.message_count(), 1);
        assert_eq!(response.messages.messages.len(), 1);
    }

    #[test]
    fn test_sms_list_response_with_count() {
        let xml_with_count = r#"<response>
    <Count>1</Count>
    <Messages>
        <Message>
            <Smstat>0</Smstat>
            <Index>1</Index>
            <Phone>+123456789</Phone>
            <Content>Test message</Content>
            <Date>2023-01-01 12:00:00</Date>
            <Sca></Sca>
            <SaveType>0</SaveType>
            <Priority>0</Priority>
            <SmsType>1</SmsType>
        </Message>
    </Messages>
</response>"#;

        let response: SmsListResponse = serde_xml_rs::from_str(xml_with_count).unwrap();
        assert_eq!(response.count, Some("1".to_string()));
        assert_eq!(response.message_count(), 1);
        assert_eq!(response.messages.messages.len(), 1);
    }

    #[test]
    fn test_sms_list_response_multiple_messages() {
        let xml_multiple_messages = r#"<response>
    <Count>2</Count>
    <Messages>
        <Message>
            <Smstat>0</Smstat>
            <Index>40003</Index>
            <Phone>+48616673870</Phone>
            <Content>Test message 1</Content>
            <Date>2025-06-09 17:08:58</Date>
            <Sca></Sca>
            <SaveType>0</SaveType>
            <Priority>0</Priority>
            <SmsType>1</SmsType>
        </Message>
        <Message>
            <Smstat>1</Smstat>
            <Index>40002</Index>
            <Phone>3350</Phone>
            <Content>Test message 2</Content>
            <Date>2024-11-22 12:32:12</Date>
            <Sca></Sca>
            <SaveType>0</SaveType>
            <Priority>0</Priority>
            <SmsType>5</SmsType>
        </Message>
    </Messages>
</response>"#;

        let response: SmsListResponse = serde_xml_rs::from_str(xml_multiple_messages).unwrap();
        assert_eq!(response.count, Some("2".to_string()));
        assert_eq!(response.message_count(), 2);
        assert_eq!(response.messages.messages.len(), 2);
        
        assert_eq!(response.messages.messages[0].index, "40003");
        assert_eq!(response.messages.messages[0].phone, "+48616673870");
        assert!(response.messages.messages[0].is_unread());
        
        assert_eq!(response.messages.messages[1].index, "40002");
        assert_eq!(response.messages.messages[1].phone, "3350");
        assert!(response.messages.messages[1].is_read());
    }
}
