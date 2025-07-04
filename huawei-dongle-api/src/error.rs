//! Error types for the Huawei Dongle API

use thiserror::Error;

/// Common Huawei API Error Codes
/// 
/// These error codes are returned by the device in XML error responses
/// 
/// ## Authentication Errors (100xxx)
/// - `100003` - No rights (login required)
/// - `108001` - Username wrong
/// - `108002` - Password wrong  
/// - `108003` - Already logged in
/// - `108006` - Username or password wrong
/// - `108007` - Too many login attempts
/// 
/// ## System Errors (100xxx)
/// - `100001` - Unknown system error
/// - `100002` - Not supported
/// - `100004` - System busy
/// - `100005` - Format error
/// 
/// ## Session/CSRF Errors (125xxx)
/// - `125001` - Wrong token
/// - `125002` - CSRF token invalid
/// - `125003` - Wrong session token
/// 
/// ## SMS Errors (111xxx)
/// - `111001` - Phone number invalid
/// - `111019` - SMS center number invalid
/// - `111020` - SMS processing
/// - `111022` - SMS not enough space
/// 
/// ## Network/Connection Errors (112xxx, 113xxx)
/// - `112001` - Voice busy
/// - `113017` - SIM not inserted
/// - `114001` - File not found
/// - `114002` - File too large
/// 
/// ## PIN/PUK Errors (106xxx, 107xxx)
/// - `106001` - Incorrect PIN
/// - `107002` - Incorrect PUK
/// - `107003` - PUK times exceeded (SIM locked)
pub mod error_codes {
    pub const NO_RIGHTS: i32 = 100003;
    pub const CSRF_TOKEN_ERROR: i32 = 125002;
    pub const SESSION_TOKEN_ERROR: i32 = 125003;
    pub const USERNAME_WRONG: i32 = 108001;
    pub const PASSWORD_WRONG: i32 = 108002;
    pub const ALREADY_LOGIN: i32 = 108003;
    pub const USERNAME_PWD_WRONG: i32 = 108006;
    pub const USERNAME_PWD_OVERRUN: i32 = 108007;
}

/// Result type alias for this crate
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for the Huawei Dongle API
#[derive(Error, Debug)]
pub enum Error {
    /// HTTP request errors
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// XML parsing errors
    #[error("XML parsing failed: {0}")]
    Xml(#[from] serde_xml_rs::Error),

    /// Quick XML parsing errors
    #[error("XML parsing failed: {0}")]
    QuickXml(#[from] quick_xml::Error),

    /// URL parsing errors
    #[error("Invalid URL: {0}")]
    Url(#[from] url::ParseError),

    /// Authentication errors
    #[error("Authentication failed: {message}")]
    Authentication { message: String },

    /// Login required error
    #[error("Login required")]
    LoginRequired,

    /// Invalid username error
    #[error("Invalid username")]
    InvalidUsername,

    /// Invalid password error
    #[error("Invalid password")]
    InvalidPassword,

    /// Invalid credentials error
    #[error("Invalid username or password")]
    InvalidCredentials,

    /// Too many login attempts
    #[error("Too many login attempts")]
    TooManyLoginAttempts,

    /// Already logged in
    #[error("Already logged in")]
    AlreadyLoggedIn,

    /// CSRF token error
    #[error("CSRF token invalid")]
    CsrfTokenInvalid,

    /// Session token error
    #[error("Session token invalid")]
    SessionTokenInvalid,

    /// API errors with error code
    #[error("API error {code}: {message}")]
    Api { code: i32, message: String },

    /// Session management errors
    #[error("Session error: {message}")]
    Session { message: String },

    /// Configuration errors
    #[error("Configuration error: {message}")]
    Config { message: String },

    /// Generic errors
    #[error("Error: {message}")]
    Generic { message: String },
}

impl Error {
    /// Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            Error::Http(e) => {
                if e.is_timeout() || e.is_connect() {
                    return true;
                }
                if let Some(status) = e.status() {
                    return !status.is_client_error();
                }
                true
            }
            Error::Api { code, .. } => {
                *code >= 500 && *code < 600
            }
            Error::Session { .. } => true,
            Error::LoginRequired => false,
            Error::InvalidUsername => false,
            Error::InvalidPassword => false,
            Error::InvalidCredentials => false,
            Error::TooManyLoginAttempts => false,
            Error::AlreadyLoggedIn => false,
            Error::CsrfTokenInvalid => true,
            Error::SessionTokenInvalid => true,
            _ => false,
        }
    }

    /// Create an authentication error
    pub fn authentication<S: Into<String>>(message: S) -> Self {
        Self::Authentication {
            message: message.into(),
        }
    }

    /// Create an API error with specific error variant for known codes
    pub fn api(code: i32, message: String) -> Self {
        use error_codes::*;
        
        match code {
            NO_RIGHTS => Self::LoginRequired,
            CSRF_TOKEN_ERROR => Self::CsrfTokenInvalid,
            SESSION_TOKEN_ERROR => Self::SessionTokenInvalid,
            USERNAME_WRONG => Self::InvalidUsername,
            PASSWORD_WRONG => Self::InvalidPassword,
            ALREADY_LOGIN => Self::AlreadyLoggedIn,
            USERNAME_PWD_WRONG => Self::InvalidCredentials,
            USERNAME_PWD_OVERRUN => Self::TooManyLoginAttempts,
            _ => Self::Api { code, message }
        }
    }

    /// Create a session error
    pub fn session<S: Into<String>>(message: S) -> Self {
        Self::Session {
            message: message.into(),
        }
    }

    /// Create a config error
    pub fn config<S: Into<String>>(message: S) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    /// Create a generic error
    pub fn generic<S: Into<String>>(message: S) -> Self {
        Self::Generic {
            message: message.into(),
        }
    }
}
