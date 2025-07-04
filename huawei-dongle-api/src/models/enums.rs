//! Strong types and enums for Huawei API values
//!
//! This module provides type-safe enums for all API values instead of using
//! string literals or magic numbers. This improves type safety, provides
//! better IDE support, and reduces the chance of typos.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Connection status values from `/api/monitoring/status`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionStatus {
    #[serde(rename = "900")]
    Connecting,
    #[serde(rename = "901")]
    Connected,
    #[serde(rename = "902")]
    Disconnected,
    #[serde(rename = "903")]
    Disconnecting,
    #[serde(rename = "904")]
    ConnectFailed,
    #[serde(rename = "905")]
    ConnectStatusNull,
    #[serde(rename = "906")]
    ConnectStatusError,
}

impl fmt::Display for ConnectionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            ConnectionStatus::Connecting => "CONNECTING",
            ConnectionStatus::Connected => "CONNECTED",
            ConnectionStatus::Disconnected => "DISCONNECTED",
            ConnectionStatus::Disconnecting => "DISCONNECTING",
            ConnectionStatus::ConnectFailed => "CONNECT_FAILED",
            ConnectionStatus::ConnectStatusNull => "CONNECT_STATUS_NULL",
            ConnectionStatus::ConnectStatusError => "CONNECT_STATUS_ERROR",
        };
        write!(f, "{}", text)
    }
}

impl ConnectionStatus {
    /// Check if the connection is established
    pub fn is_connected(&self) -> bool {
        matches!(self, ConnectionStatus::Connected)
    }

    /// Check if the connection is in progress
    pub fn is_connecting(&self) -> bool {
        matches!(self, ConnectionStatus::Connecting)
    }

    /// Check if the connection is disconnected
    pub fn is_disconnected(&self) -> bool {
        matches!(self, ConnectionStatus::Disconnected | ConnectionStatus::Disconnecting)
    }

    /// Check if the connection has failed
    pub fn is_failed(&self) -> bool {
        matches!(
            self,
            ConnectionStatus::ConnectFailed
                | ConnectionStatus::ConnectStatusNull
                | ConnectionStatus::ConnectStatusError
        )
    }
}

/// Network type values from monitoring status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkType {
    #[serde(rename = "7")]
    Hspa,
    #[serde(rename = "19")]
    Lte,
    #[serde(rename = "41")]
    LteCarrierAggregation,
    #[serde(rename = "101")]
    FiveGNsa,
    #[serde(rename = "102")]
    FiveGSa,
}

impl fmt::Display for NetworkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            NetworkType::Hspa => "HSPA (3G)",
            NetworkType::Lte => "LTE (4G)",
            NetworkType::LteCarrierAggregation => "LTE CA (4G+)",
            NetworkType::FiveGNsa => "5G NSA",
            NetworkType::FiveGSa => "5G SA",
        };
        write!(f, "{}", text)
    }
}

impl NetworkType {
    /// Get extended display text for the network type
    pub fn extended_text(&self) -> &'static str {
        match self {
            NetworkType::Hspa => "HSPA",
            NetworkType::Lte => "LTE",
            NetworkType::LteCarrierAggregation => "LTE Carrier Aggregation",
            NetworkType::FiveGNsa => "5G Non-Standalone",
            NetworkType::FiveGSa => "5G Standalone",
        }
    }

    /// Check if this is a 5G network type
    pub fn is_5g(&self) -> bool {
        matches!(self, NetworkType::FiveGNsa | NetworkType::FiveGSa)
    }

    /// Check if this is a 4G/LTE network type
    pub fn is_4g(&self) -> bool {
        matches!(self, NetworkType::Lte | NetworkType::LteCarrierAggregation)
    }

    /// Check if this is a 3G network type
    pub fn is_3g(&self) -> bool {
        matches!(self, NetworkType::Hspa)
    }
}

/// Network mode configuration values from `/api/net/net-mode`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkModeType {
    #[serde(rename = "00")]
    Auto,
    #[serde(rename = "01")]
    TwoGOnly,
    #[serde(rename = "02")]
    ThreeGOnly,
    #[serde(rename = "03")]
    FourGOnly,
    #[serde(rename = "0201")]
    ThreeGPreferredTwoGFallback,
    #[serde(rename = "0301")]
    FourGPreferredTwoGFallback,
    #[serde(rename = "0302")]
    FourGPreferredThreeGFallback,
}

impl fmt::Display for NetworkModeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            NetworkModeType::Auto => "Auto (2G/3G/4G)",
            NetworkModeType::TwoGOnly => "2G Only (GSM/EDGE)",
            NetworkModeType::ThreeGOnly => "3G Only (UMTS/HSPA)",
            NetworkModeType::FourGOnly => "4G Only (LTE)",
            NetworkModeType::ThreeGPreferredTwoGFallback => "3G Preferred, 2G Fallback",
            NetworkModeType::FourGPreferredTwoGFallback => "4G Preferred, 2G Fallback",
            NetworkModeType::FourGPreferredThreeGFallback => "4G Preferred, 3G Fallback",
        };
        write!(f, "{}", text)
    }
}

/// SIM status values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimStatus {
    #[serde(rename = "0")]
    NotReady,
    #[serde(rename = "1")]
    Ready,
}

impl SimStatus {
    /// Check if SIM is ready
    pub fn is_ready(&self) -> bool {
        matches!(self, SimStatus::Ready)
    }
}

/// Roaming status values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoamingStatus {
    #[serde(rename = "0")]
    NotRoaming,
    #[serde(rename = "1")]
    Roaming,
}

impl RoamingStatus {
    /// Check if currently roaming
    pub fn is_roaming(&self) -> bool {
        matches!(self, RoamingStatus::Roaming)
    }
}

/// Service status values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceStatus {
    #[serde(rename = "0")]
    NoService,
    #[serde(rename = "1")]
    LimitedService,
    #[serde(rename = "2")]
    FullService,
}

impl ServiceStatus {
    /// Check if service is available (limited or full)
    pub fn is_available(&self) -> bool {
        matches!(self, ServiceStatus::LimitedService | ServiceStatus::FullService)
    }

    /// Check if full service is available
    pub fn is_full_service(&self) -> bool {
        matches!(self, ServiceStatus::FullService)
    }
}

/// SMS status values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SmsStatus {
    #[serde(rename = "0")]
    Unread,
    #[serde(rename = "1")]
    Read,
    #[serde(rename = "2")]
    PendingSend,
    #[serde(rename = "3")]
    Sent,
    #[serde(rename = "4")]
    SendFailed,
}

impl SmsStatus {
    pub fn is_unread(&self) -> bool {
        matches!(self, SmsStatus::Unread)
    }

    pub fn is_read(&self) -> bool {
        matches!(self, SmsStatus::Read)
    }

    pub fn is_sent(&self) -> bool {
        matches!(self, SmsStatus::Sent)
    }
}

/// SMS priority values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SmsPriority {
    #[serde(rename = "0")]
    Normal,
    #[serde(rename = "1")]
    High,
}

/// SMS message type values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SmsType {
    #[serde(rename = "1")]
    Single,
    #[serde(rename = "2")]
    Multipart,
    #[serde(rename = "5")]
    Unicode,
    #[serde(rename = "7")]
    DeliveryConfirmationSuccess,
    #[serde(rename = "8")]
    DeliveryConfirmationFailure,
}

/// SMS box types for message storage locations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SmsBoxType {
    #[serde(rename = "1")]
    LocalInbox,
    #[serde(rename = "2")]
    LocalOutbox,
    #[serde(rename = "3")]
    LocalDraft,
    #[serde(rename = "4")]
    SimInbox,
    #[serde(rename = "5")]
    SimOutbox,
    #[serde(rename = "6")]
    SimDraft,
}

impl fmt::Display for SmsBoxType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            SmsBoxType::LocalInbox => "1",
            SmsBoxType::LocalOutbox => "2",
            SmsBoxType::LocalDraft => "3",
            SmsBoxType::SimInbox => "4",
            SmsBoxType::SimOutbox => "5",
            SmsBoxType::SimDraft => "6",
        };
        write!(f, "{}", text)
    }
}

/// SMS sort types for message ordering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SmsSortType {
    #[serde(rename = "0")]
    ByTime,
    #[serde(rename = "1")]
    ByName,
}

impl fmt::Display for SmsSortType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            SmsSortType::ByTime => "0",
            SmsSortType::ByName => "1",
        };
        write!(f, "{}", text)
    }
}

/// Login status values from authentication
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoginStatus {
    #[serde(rename = "0")]
    LoggedIn,
    #[serde(rename = "-1")]
    NotLoggedIn,
    #[serde(rename = "-2")]
    RepeatLoginRequired,
}

impl LoginStatus {
    /// Check if user is logged in
    pub fn is_logged_in(&self) -> bool {
        matches!(self, LoginStatus::LoggedIn)
    }
}

/// Lock status values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LockStatus {
    #[serde(rename = "0")]
    Unlocked,
    #[serde(rename = "1")]
    Locked,
}

impl LockStatus {
    /// Check if account is locked
    pub fn is_locked(&self) -> bool {
        matches!(self, LockStatus::Locked)
    }
}

/// DHCP status values (enabled/disabled)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DhcpStatus {
    Disabled,
    Enabled,
}

impl Serialize for DhcpStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            DhcpStatus::Disabled => "0",
            DhcpStatus::Enabled => "1",
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for DhcpStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "0" => Ok(DhcpStatus::Disabled),
            "1" => Ok(DhcpStatus::Enabled),
            _ => Err(serde::de::Error::custom(format!("Invalid DHCP status: {}", value))),
        }
    }
}

impl DhcpStatus {
    /// Check if DHCP is enabled
    pub fn is_enabled(&self) -> bool {
        matches!(self, DhcpStatus::Enabled)
    }
}

/// DNS status values (enabled/disabled)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnsStatus {
    Disabled,
    Enabled,
}

impl Serialize for DnsStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            DnsStatus::Disabled => "0",
            DnsStatus::Enabled => "1",
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for DnsStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "0" => Ok(DnsStatus::Disabled),
            "1" => Ok(DnsStatus::Enabled),
            _ => Err(serde::de::Error::custom(format!("Invalid DNS status: {}", value))),
        }
    }
}

impl DnsStatus {
    /// Check if DNS is enabled
    pub fn is_enabled(&self) -> bool {
        matches!(self, DnsStatus::Enabled)
    }
}

/// Device control operation types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceControlType {
    Reboot,
    FactoryReset,
    BackupConfiguration,
    PowerOff,
}

impl Serialize for DeviceControlType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            DeviceControlType::Reboot => 1,
            DeviceControlType::FactoryReset => 2,
            DeviceControlType::BackupConfiguration => 3,
            DeviceControlType::PowerOff => 4,
        };
        serializer.serialize_i32(value)
    }
}

impl<'de> Deserialize<'de> for DeviceControlType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        match value {
            1 => Ok(DeviceControlType::Reboot),
            2 => Ok(DeviceControlType::FactoryReset),
            3 => Ok(DeviceControlType::BackupConfiguration),
            4 => Ok(DeviceControlType::PowerOff),
            _ => Err(serde::de::Error::custom(format!("Invalid device control type: {}", value))),
        }
    }
}

impl fmt::Display for DeviceControlType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            DeviceControlType::Reboot => "Reboot",
            DeviceControlType::FactoryReset => "Factory Reset",
            DeviceControlType::BackupConfiguration => "Backup Configuration",
            DeviceControlType::PowerOff => "Power Off",
        };
        write!(f, "{}", text)
    }
}

/// API error codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiErrorCode {
    // Token and session errors
    WrongToken,
    CsrfTokenInvalid,
    WrongSessionToken,

    // Authentication errors
    UsernameWrong,
    PasswordWrong,
    AlreadyLoggedIn,
    UsernameOrPasswordWrong,
    TooManyLoginAttempts,
    PasswordChangeRequired,

    // System errors
    SystemUnknown,
    SystemNoSupport,
    NoRights,
    SystemBusy,
    FormatError,
}

impl Serialize for ApiErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            ApiErrorCode::WrongToken => "125001",
            ApiErrorCode::CsrfTokenInvalid => "125002",
            ApiErrorCode::WrongSessionToken => "125003",
            ApiErrorCode::UsernameWrong => "108001",
            ApiErrorCode::PasswordWrong => "108002",
            ApiErrorCode::AlreadyLoggedIn => "108003",
            ApiErrorCode::UsernameOrPasswordWrong => "108006",
            ApiErrorCode::TooManyLoginAttempts => "108007",
            ApiErrorCode::PasswordChangeRequired => "115002",
            ApiErrorCode::SystemUnknown => "100001",
            ApiErrorCode::SystemNoSupport => "100002",
            ApiErrorCode::NoRights => "100003",
            ApiErrorCode::SystemBusy => "100004",
            ApiErrorCode::FormatError => "100005",
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for ApiErrorCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "125001" => Ok(ApiErrorCode::WrongToken),
            "125002" => Ok(ApiErrorCode::CsrfTokenInvalid),
            "125003" => Ok(ApiErrorCode::WrongSessionToken),
            "108001" => Ok(ApiErrorCode::UsernameWrong),
            "108002" => Ok(ApiErrorCode::PasswordWrong),
            "108003" => Ok(ApiErrorCode::AlreadyLoggedIn),
            "108006" => Ok(ApiErrorCode::UsernameOrPasswordWrong),
            "108007" => Ok(ApiErrorCode::TooManyLoginAttempts),
            "115002" => Ok(ApiErrorCode::PasswordChangeRequired),
            "100001" => Ok(ApiErrorCode::SystemUnknown),
            "100002" => Ok(ApiErrorCode::SystemNoSupport),
            "100003" => Ok(ApiErrorCode::NoRights),
            "100004" => Ok(ApiErrorCode::SystemBusy),
            "100005" => Ok(ApiErrorCode::FormatError),
            _ => Err(serde::de::Error::custom(format!("Invalid API error code: {}", value))),
        }
    }
}

impl fmt::Display for ApiErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            ApiErrorCode::WrongToken => "Wrong token",
            ApiErrorCode::CsrfTokenInvalid => "CSRF token invalid",
            ApiErrorCode::WrongSessionToken => "Wrong session token",
            ApiErrorCode::UsernameWrong => "Username wrong",
            ApiErrorCode::PasswordWrong => "Password wrong",
            ApiErrorCode::AlreadyLoggedIn => "Already logged in",
            ApiErrorCode::UsernameOrPasswordWrong => "Username or password wrong",
            ApiErrorCode::TooManyLoginAttempts => "Too many login attempts",
            ApiErrorCode::PasswordChangeRequired => "Password change required",
            ApiErrorCode::SystemUnknown => "System unknown error",
            ApiErrorCode::SystemNoSupport => "System does not support this operation",
            ApiErrorCode::NoRights => "No rights (login required)",
            ApiErrorCode::SystemBusy => "System busy",
            ApiErrorCode::FormatError => "Format error",
        };
        write!(f, "{}", text)
    }
}

impl ApiErrorCode {
    /// Check if this is a CSRF/token related error
    pub fn is_csrf_error(&self) -> bool {
        matches!(
            self,
            ApiErrorCode::CsrfTokenInvalid | ApiErrorCode::WrongToken
        )
    }

    /// Check if this is a session related error
    pub fn is_session_error(&self) -> bool {
        matches!(self, ApiErrorCode::WrongSessionToken)
    }

    /// Check if this is an authentication error
    pub fn is_auth_error(&self) -> bool {
        matches!(
            self,
            ApiErrorCode::UsernameWrong
                | ApiErrorCode::PasswordWrong
                | ApiErrorCode::UsernameOrPasswordWrong
                | ApiErrorCode::TooManyLoginAttempts
                | ApiErrorCode::PasswordChangeRequired
        )
    }

    /// Get the error code as an integer
    pub fn as_int(&self) -> i32 {
        match self {
            ApiErrorCode::WrongToken => 125001,
            ApiErrorCode::CsrfTokenInvalid => 125002,
            ApiErrorCode::WrongSessionToken => 125003,
            ApiErrorCode::UsernameWrong => 108001,
            ApiErrorCode::PasswordWrong => 108002,
            ApiErrorCode::AlreadyLoggedIn => 108003,
            ApiErrorCode::UsernameOrPasswordWrong => 108006,
            ApiErrorCode::TooManyLoginAttempts => 108007,
            ApiErrorCode::PasswordChangeRequired => 115002,
            ApiErrorCode::SystemUnknown => 100001,
            ApiErrorCode::SystemNoSupport => 100002,
            ApiErrorCode::NoRights => 100003,
            ApiErrorCode::SystemBusy => 100004,
            ApiErrorCode::FormatError => 100005,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_status_display() {
        assert_eq!(ConnectionStatus::Connected.to_string(), "CONNECTED");
        assert_eq!(ConnectionStatus::Connecting.to_string(), "CONNECTING");
        assert_eq!(ConnectionStatus::Disconnected.to_string(), "DISCONNECTED");
    }

    #[test]
    fn test_connection_status_methods() {
        assert!(ConnectionStatus::Connected.is_connected());
        assert!(!ConnectionStatus::Connecting.is_connected());
        
        assert!(ConnectionStatus::Connecting.is_connecting());
        assert!(!ConnectionStatus::Connected.is_connecting());
        
        assert!(ConnectionStatus::Disconnected.is_disconnected());
        assert!(!ConnectionStatus::Connected.is_disconnected());
        
        assert!(ConnectionStatus::ConnectFailed.is_failed());
        assert!(!ConnectionStatus::Connected.is_failed());
    }

    #[test]
    fn test_network_type_display() {
        assert_eq!(NetworkType::Lte.to_string(), "LTE (4G)");
        assert_eq!(NetworkType::FiveGNsa.to_string(), "5G NSA");
        assert_eq!(NetworkType::Hspa.to_string(), "HSPA (3G)");
    }

    #[test]
    fn test_network_type_methods() {
        assert!(NetworkType::FiveGNsa.is_5g());
        assert!(!NetworkType::Lte.is_5g());
        
        assert!(NetworkType::Lte.is_4g());
        assert!(!NetworkType::Hspa.is_4g());
        
        assert!(NetworkType::Hspa.is_3g());
        assert!(!NetworkType::Lte.is_3g());
    }

    #[test]
    fn test_network_mode_type_display() {
        assert_eq!(NetworkModeType::Auto.to_string(), "Auto (2G/3G/4G)");
        assert_eq!(NetworkModeType::FourGOnly.to_string(), "4G Only (LTE)");
    }

    #[test]
    fn test_status_methods() {
        assert!(SimStatus::Ready.is_ready());
        assert!(!SimStatus::NotReady.is_ready());
        
        assert!(RoamingStatus::Roaming.is_roaming());
        assert!(!RoamingStatus::NotRoaming.is_roaming());
        
        assert!(ServiceStatus::FullService.is_available());
        assert!(ServiceStatus::FullService.is_full_service());
        assert!(!ServiceStatus::NoService.is_available());
    }

    #[test]
    fn test_sms_status_methods() {
        assert!(SmsStatus::Unread.is_unread());
        assert!(!SmsStatus::Read.is_unread());
        
        assert!(SmsStatus::Read.is_read());
        assert!(!SmsStatus::Unread.is_read());
        
        assert!(SmsStatus::Sent.is_sent());
        assert!(!SmsStatus::Unread.is_sent());
    }

    #[test]
    fn test_api_error_code_methods() {
        assert!(ApiErrorCode::CsrfTokenInvalid.is_csrf_error());
        assert!(!ApiErrorCode::UsernameWrong.is_csrf_error());
        
        assert!(ApiErrorCode::WrongSessionToken.is_session_error());
        assert!(!ApiErrorCode::CsrfTokenInvalid.is_session_error());
        
        assert!(ApiErrorCode::UsernameWrong.is_auth_error());
        assert!(!ApiErrorCode::CsrfTokenInvalid.is_auth_error());
    }
}
