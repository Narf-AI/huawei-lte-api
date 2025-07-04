//! Authentication models

use serde::{Deserialize, Serialize};
use super::enums::{LoginStatus, LockStatus};

/// Login state response from `/api/user/state-login`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginState {
    /// Password encoding type (0=BASE64, 3=BASE64_after_change, 4=SHA256)
    #[serde(rename = "password_type")]
    pub password_type: String,
    
    /// External password type
    #[serde(rename = "extern_password_type")]
    pub extern_password_type: String,
    
    /// History login flag
    #[serde(rename = "history_login_flag")]
    pub history_login_flag: String,
    
    /// Current login state (-1=not_logged_in, 0=logged_in, -2=repeat_login_required)
    #[serde(rename = "State")]
    pub state: LoginStatus,
    
    /// Guide modify password page flag
    #[serde(rename = "guidemodifypwdpageflag")]
    pub guide_modify_pwd_page_flag: String,
    
    /// RSA padding type
    #[serde(rename = "rsapadingtype")]
    pub rsa_padding_type: String,
    
    /// Number of accounts
    #[serde(rename = "accounts_number")]
    pub accounts_number: String,
    
    /// WiFi password same with web password
    #[serde(rename = "wifipwdsamewithwebpwd")]
    pub wifi_pwd_same_with_web_pwd: String,
    
    /// Remaining wait time
    #[serde(rename = "remainwaittime")]
    pub remain_wait_time: String,
    
    /// Lock status (0=unlocked, >0=locked)
    #[serde(rename = "lockstatus")]
    pub lock_status: LockStatus,
    
    /// Force skip guide
    #[serde(rename = "forceskipguide")]
    pub force_skip_guide: String,
    
    /// Username
    #[serde(rename = "username")]
    pub username: String,
    
    /// First login flag
    #[serde(rename = "firstlogin")]
    pub first_login: String,
    
    /// User level
    #[serde(rename = "userlevel")]
    pub user_level: String,
}

/// Login request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    /// Username (typically "admin")
    #[serde(rename = "Username")]
    pub username: String,
    
    /// Encoded password (BASE64 or SHA256)
    #[serde(rename = "Password")]
    pub password: String,
    
    /// Password type from login state
    #[serde(rename = "password_type")]
    pub password_type: String,
}

/// Logout request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoutRequest {
    /// Logout type (usually "1")
    #[serde(rename = "Logout")]
    pub logout: String,
}

impl LoginState {
    /// Check if user is currently logged in
    pub fn is_logged_in(&self) -> bool {
        self.state.is_logged_in()
    }
    
    /// Check if account is locked
    pub fn is_locked(&self) -> bool {
        self.lock_status.is_locked()
    }
    
    /// Get password encoding type
    pub fn password_encoding(&self) -> PasswordEncoding {
        match self.password_type.as_str() {
            "0" => PasswordEncoding::Base64,
            "3" => PasswordEncoding::Base64AfterChange,
            "4" => PasswordEncoding::Sha256,
            _ => PasswordEncoding::Unknown,
        }
    }
}

/// Password encoding types
#[derive(Debug, Clone, PartialEq)]
pub enum PasswordEncoding {
    /// BASE64 encoding
    Base64,
    /// BASE64 encoding after password change
    Base64AfterChange,
    /// SHA256 encoding (most common)
    Sha256,
    /// Unknown encoding type
    Unknown,
}

impl LoginRequest {
    /// Create a new login request
    pub fn new(username: String, password: String, password_type: String) -> Self {
        Self {
            username,
            password,
            password_type,
        }
    }
}

impl LogoutRequest {
    /// Create a new logout request
    pub fn new() -> Self {
        Self {
            logout: "1".to_string(),
        }
    }
}

impl Default for LogoutRequest {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_state_parsing() {
        let xml = r#"
        <response>
            <password_type>4</password_type>
            <extern_password_type>1</extern_password_type>
            <history_login_flag>0</history_login_flag>
            <State>-1</State>
            <guidemodifypwdpageflag>0</guidemodifypwdpageflag>
            <rsapadingtype>1</rsapadingtype>
            <accounts_number>1</accounts_number>
            <wifipwdsamewithwebpwd>0</wifipwdsamewithwebpwd>
            <remainwaittime>0</remainwaittime>
            <lockstatus>0</lockstatus>
            <forceskipguide>0</forceskipguide>
            <username></username>
            <firstlogin>0</firstlogin>
            <userlevel></userlevel>
        </response>"#;

        let state: LoginState = serde_xml_rs::from_str(xml).unwrap();
        assert_eq!(state.password_type, "4");
        assert_eq!(state.state, LoginStatus::NotLoggedIn);
        assert!(!state.is_logged_in());
        assert!(!state.is_locked());
        assert_eq!(state.password_encoding(), PasswordEncoding::Sha256);
    }

    #[test]
    fn test_login_request_serialization() {
        let request = LoginRequest::new(
            "admin".to_string(),
            "encoded_password".to_string(),
            "4".to_string(),
        );

        let xml = serde_xml_rs::to_string(&request).unwrap();
        assert!(xml.contains("<Username>admin</Username>"));
        assert!(xml.contains("<Password>encoded_password</Password>"));
        assert!(xml.contains("<password_type>4</password_type>"));
    }

    #[test]
    fn test_password_encoding_detection() {
        let mut state = LoginState {
            password_type: "0".to_string(),
            state: LoginStatus::NotLoggedIn,
            lock_status: LockStatus::Unlocked,
            extern_password_type: "1".to_string(),
            history_login_flag: "0".to_string(),
            guide_modify_pwd_page_flag: "0".to_string(),
            rsa_padding_type: "1".to_string(),
            accounts_number: "1".to_string(),
            wifi_pwd_same_with_web_pwd: "0".to_string(),
            remain_wait_time: "0".to_string(),
            force_skip_guide: "0".to_string(),
            username: "".to_string(),
            first_login: "0".to_string(),
            user_level: "".to_string(),
        };

        assert_eq!(state.password_encoding(), PasswordEncoding::Base64);

        state.password_type = "4".to_string();
        assert_eq!(state.password_encoding(), PasswordEncoding::Sha256);
    }
}