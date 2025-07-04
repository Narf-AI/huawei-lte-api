//! API endpoint implementations.
//! 
//! This module contains the API endpoint implementations organized by functionality.
//! Each sub-module provides a specific API that can be accessed through the main [`Client`](crate::Client).
//! 
//! # Available APIs
//! 
//! - [`auth`] - Authentication operations (login/logout)
//! - [`device`] - Device information and control (reboot/power)
//! - [`dhcp`] - DHCP server configuration
//! - [`monitoring`] - Connection and signal monitoring
//! - [`network`] - Network mode and operator selection
//! - [`sms`] - SMS message management
//! 
//! # Usage Pattern
//! 
//! All APIs follow the same pattern:
//! 
//! ```no_run
//! # use huawei_dongle_api::{Client, Config};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::new(Config::default())?;
//! 
//! // Access APIs through the client
//! let device_api = client.device();
//! let sms_api = client.sms();
//! let monitoring_api = client.monitoring();
//! 
//! // Call API methods
//! let info = device_api.information().await?;
//! let count = sms_api.count().await?;
//! let status = monitoring_api.status().await?;
//! # Ok(())
//! # }
//! ```

pub mod auth;
pub mod device;
pub mod dhcp;
pub mod monitoring;
pub mod network;
pub mod sms;
