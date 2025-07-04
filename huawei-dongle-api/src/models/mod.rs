//! Data models for API requests and responses
//! 
//! This module contains all the data structures used for communication with Huawei devices.
//! All models support XML serialization/deserialization using serde.
//! 
//! # Organization
//! 
//! - [`auth`] - Authentication and login structures
//! - [`common`] - Common types like errors and generic responses
//! - [`device`] - Device information and control structures
//! - [`dhcp`] - DHCP configuration models
//! - [`monitoring`] - Connection status and monitoring data
//! - [`network`] - Network configuration and status
//! - [`sms`] - SMS message structures
//! 
//! # XML Format
//! 
//! Huawei devices use a specific XML format for API communication:
//! 
//! ```xml
//! <request>
//!     <Username>admin</Username>
//!     <Password>YWRtaW4=</Password>
//! </request>
//! ```
//! 
//! ```xml
//! <response>
//!     <ConnectionStatus>901</ConnectionStatus>
//!     <CurrentNetworkType>19</CurrentNetworkType>
//! </response>
//! ```
//! 
//! All models handle this format automatically through serde attributes.

pub mod auth;
pub mod common;
pub mod device;
pub mod dhcp;
pub mod enums;
pub mod monitoring;
pub mod network;
pub mod sms;

// Re-export common types
pub use common::*;
pub use enums::*;
pub use monitoring::*;
pub use network::*;
pub use sms::*;
