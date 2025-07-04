//! Common models and types

use serde::{Deserialize, Serialize};
use super::enums::ApiErrorCode;

/// Standard API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    #[serde(flatten)]
    pub data: T,
}

/// Error response from the API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: Option<i32>,
    pub message: Option<String>,
}

/// Huawei API error response in the format: `<error><code>X</code><message/></error>`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "error")]
pub struct ApiError {
    /// Error code (e.g., 125002, 125003)
    pub code: ApiErrorCode,
    /// Error message (usually empty)
    #[serde(default)]
    pub message: Option<String>,
}

impl ApiError {
    /// Get the error code as the enum variant
    pub fn error_code(&self) -> ApiErrorCode {
        self.code
    }

    /// Check if this is a CSRF token error
    pub fn is_csrf_error(&self) -> bool {
        self.code.is_csrf_error()
    }

    /// Check if this is a session error
    pub fn is_session_error(&self) -> bool {
        self.code.is_session_error()
    }

    /// Check if this is an authentication error
    pub fn is_auth_error(&self) -> bool {
        self.code.is_auth_error()
    }

    /// Get a human-readable error message
    pub fn error_message(&self) -> String {
        if let Some(msg) = &self.message {
            if !msg.is_empty() {
                return msg.clone();
            }
        }
        self.code.to_string()
    }
}

/// Generic success/error response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "response")]
pub struct Response {
    #[serde(rename = "OK", default)]
    pub ok: Option<String>,
    #[serde(rename = "ErrorCode", default)]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage", default)]
    pub error_message: Option<String>,
}

impl Response {
    /// Check if the response indicates success
    pub fn is_success(&self) -> bool {
        self.ok.is_some() || self.error_code.as_deref() == Some("0") || self.error_code.is_none()
    }

    /// Get the error code as an integer
    pub fn error_code(&self) -> Option<i32> {
        self.error_code.as_ref().and_then(|code| code.parse().ok())
    }

    /// Get the error message
    pub fn error_message(&self) -> Option<&str> {
        self.error_message.as_deref()
    }
}

/// Check if XML text contains an error response and parse it
pub fn check_for_api_error(xml_text: &str) -> Option<ApiError> {
    if xml_text.contains("<error>") && xml_text.contains("<code>") {
        if let Ok(error) = serde_xml_rs::from_str::<ApiError>(xml_text) {
            return Some(error);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_parsing() {
        let error_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<error>
    <code>125002</code>
    <message></message>
</error>"#;

        let error: ApiError = serde_xml_rs::from_str(error_xml).unwrap();
        assert_eq!(error.code, ApiErrorCode::CsrfTokenInvalid);
        assert!(error.is_csrf_error());
        assert!(!error.is_session_error());
        assert_eq!(error.error_message(), "CSRF token invalid");
    }

    #[test]
    fn test_check_for_api_error() {
        let error_xml = r#"<error><code>125003</code><message/></error>"#;
        let error = check_for_api_error(error_xml).unwrap();
        
        assert_eq!(error.code, ApiErrorCode::WrongSessionToken);
        assert!(error.is_session_error());
        assert_eq!(error.error_message(), "Wrong session token");

        let success_xml = r#"<response>OK</response>"#;
        assert!(check_for_api_error(success_xml).is_none());
    }

    #[test]
    fn test_error_code_classification() {
        let mut error = ApiError { 
            code: ApiErrorCode::UsernameOrPasswordWrong, 
            message: None 
        };
        assert!(error.is_auth_error());
        assert!(!error.is_csrf_error());
        assert!(!error.is_session_error());

        error.code = ApiErrorCode::WrongToken;
        assert!(error.is_csrf_error());
        assert!(!error.is_auth_error());
    }
}
