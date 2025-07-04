//! # Huawei LTE Dongle API
//! 
//! A robust async Rust client library for interacting with Huawei LTE dongles and routers.
//! 
//! This library provides a type-safe, async interface to the XML-based API used by Huawei HiLink
//! devices such as the E3372, E5577, B525 and many others. It handles authentication, session
//! management, and CSRF token rotation automatically.
//! 
//! ## Features
//! 
//! - **Async/await** - Built on tokio for efficient async I/O
//! - **Automatic retry** - Configurable retry logic with exponential backoff
//! - **Session management** - Automatic CSRF token handling and refresh
//! - **Type safety** - Strongly typed requests and responses
//! - **Error handling** - Comprehensive error types with automatic recovery
//! - **Device compatibility** - Handles quirks across different firmware versions
//! 
//! ## Quick Start
//! 
//! ```no_run
//! use huawei_dongle_api::{Client, Config};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a client with default config
//!     let client = Client::new(Config::default())?;
//!     
//!     // Get device information (no auth required)
//!     let device_info = client.device().information().await?;
//!     println!("Device: {}", device_info.device_name);
//!     
//!     // Check connection status (auth required - handled automatically)
//!     let status = client.monitoring().status().await?;
//!     println!("Connected: {}", status.is_connected());
//!     
//!     Ok(())
//! }
//! ```
//! 
//! ## Authentication
//! 
//! Most read operations don't require authentication, but monitoring status, SMS operations,
//! and configuration changes do. The library handles authentication automatically if no user/password is set:
//! 
//! ```no_run
//! # use huawei_dongle_api::{Client, Config};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::new(Config::default())?;
//! 
//! // Login (only needed for password-protected operations)
//! client.auth().login("admin", "password").await?;
//! 
//! // Now you can access protected endpoints
//! use huawei_dongle_api::models::{SmsListRequest, SmsBoxType, SmsSortType};
//! let request = SmsListRequest::new(1, 20, SmsBoxType::LocalInbox, SmsSortType::ByTime, false, true);
//! let sms_list = client.sms().list(&request).await?;
//! 
//! // Logout when done
//! client.auth().logout().await?;
//! # Ok(())
//! # }
//! ```
//! 
//! ## Configuration
//! 
//! The client can be configured with custom timeouts, retry policies, and base URLs:
//! 
//! ```no_run
//! use huawei_dongle_api::{Client, Config};
//! use std::time::Duration;
//! 
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = Config::builder()
//!     .base_url("http://192.168.8.1")
//!     .timeout(Duration::from_secs(30))
//!     .max_retries(5)
//!     .retry_delay(Duration::from_millis(500))
//!     .build();
//!     
//! let client = Client::new(config?)?;
//! # Ok(())
//! # }
//! ```
//! 
//! ## Error Handling
//! 
//! The library provides detailed error types and handles common issues automatically:
//! 
//! - CSRF token expiry - automatically refreshes and retries
//! - Session timeout - re-authenticates if needed
//! - Network errors - retries with exponential backoff
//! - Device quirks - handles different response formats
//! 
//! ## Supported APIs
//! 
//! - **Device** - Information, reboot, power control
//! - **Monitoring** - Connection status, signal strength, network info
//! - **SMS** - List, send, delete messages
//! - **Network** - Mode selection, operator info, signal details
//! - **DHCP** - IP configuration, DNS settings
//! - **Authentication** - Login/logout, password encoding

pub mod auth;
pub mod client;
pub mod config;
pub mod error;
pub mod retry;
pub mod session;

pub mod api;
pub mod models;

pub use client::Client;
pub use config::Config;
pub use error::{Error, Result};

