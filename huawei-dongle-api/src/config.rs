//! Configuration for the Huawei Dongle API client
//! 
//! This module provides configuration options for customizing the behavior of the API client.
//! 
//! # Examples
//! 
//! ## Using Default Configuration
//! 
//! ```
//! use huawei_dongle_api::Config;
//! 
//! let config = Config::default();
//! // Uses http://192.168.8.1 with 30s timeout and 3 retries
//! ```
//! 
//! ## Custom Configuration
//! 
//! ```
//! use huawei_dongle_api::Config;
//! use std::time::Duration;
//! 
//! let config = Config::builder()
//!     .base_url("http://192.168.1.1")
//!     .timeout(Duration::from_secs(60))
//!     .max_retries(5)
//!     .retry_delay(Duration::from_millis(100))
//!     .max_retry_delay(Duration::from_secs(10))
//!     .user_agent("MyApp/1.0")
//!     .build();
//! ```
//! 
//! ## Quick Configuration for URL
//! 
//! ```
//! use huawei_dongle_api::Config;
//! 
//! let config = Config::for_url("http://192.168.62.1").unwrap();
//! ```

use crate::error::{Error, Result};
use std::time::Duration;
use url::Url;

/// Configuration for the Huawei Dongle API client.
/// 
/// Controls connection parameters, retry behavior, and HTTP settings.
#[derive(Debug, Clone)]
pub struct Config {
    /// Base URL of the device (e.g., "http://192.168.8.1")
    pub base_url: Url,
    /// Request timeout for HTTP operations
    pub timeout: Duration,
    /// Maximum number of retry attempts for failed requests
    pub max_retries: usize,
    /// Initial delay before first retry
    pub retry_delay: Duration,
    /// Maximum delay between retries (for exponential backoff)
    pub max_retry_delay: Duration,
    /// User agent string sent with requests
    pub user_agent: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_url: Url::parse("http://192.168.8.1").unwrap(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
            retry_delay: Duration::from_millis(500),
            max_retry_delay: Duration::from_secs(30),
            user_agent: format!("huawei-dongle-api/{}", env!("CARGO_PKG_VERSION")),
        }
    }
}

impl Config {
    /// Create a new config builder
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    /// Create a config with default settings for the given URL
    pub fn for_url<S: AsRef<str>>(url: S) -> Result<Self> {
        Ok(Self {
            base_url: Url::parse(url.as_ref())?,
            ..Default::default()
        })
    }
}

/// Builder for Config
#[derive(Debug, Default)]
pub struct ConfigBuilder {
    base_url: Option<String>,
    timeout: Option<Duration>,
    max_retries: Option<usize>,
    retry_delay: Option<Duration>,
    max_retry_delay: Option<Duration>,
    user_agent: Option<String>,
}

impl ConfigBuilder {
    pub fn base_url<S: Into<String>>(mut self, url: S) -> Self {
        self.base_url = Some(url.into());
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn max_retries(mut self, max_retries: usize) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    pub fn retry_delay(mut self, delay: Duration) -> Self {
        self.retry_delay = Some(delay);
        self
    }

    pub fn max_retry_delay(mut self, delay: Duration) -> Self {
        self.max_retry_delay = Some(delay);
        self
    }

    pub fn user_agent<S: Into<String>>(mut self, user_agent: S) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    pub fn build(self) -> Result<Config> {
        let default = Config::default();

        let base_url = if let Some(url) = self.base_url {
            Url::parse(&url).map_err(|e| Error::config(format!("Invalid base URL: {}", e)))?
        } else {
            default.base_url
        };

        Ok(Config {
            base_url,
            timeout: self.timeout.unwrap_or(default.timeout),
            max_retries: self.max_retries.unwrap_or(default.max_retries),
            retry_delay: self.retry_delay.unwrap_or(default.retry_delay),
            max_retry_delay: self.max_retry_delay.unwrap_or(default.max_retry_delay),
            user_agent: self.user_agent.unwrap_or(default.user_agent),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.base_url.as_str(), "http://192.168.8.1/");
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert_eq!(config.max_retries, 3);
    }

    #[test]
    fn test_config_builder() {
        let config = Config::builder()
            .base_url("http://192.168.62.1")
            .timeout(Duration::from_secs(60))
            .max_retries(5)
            .build()
            .unwrap();

        assert_eq!(config.base_url.as_str(), "http://192.168.62.1/");
        assert_eq!(config.timeout, Duration::from_secs(60));
        assert_eq!(config.max_retries, 5);
    }

    #[test]
    fn test_for_url() {
        let config = Config::for_url("http://192.168.62.1").unwrap();
        assert_eq!(config.base_url.as_str(), "http://192.168.62.1/");
    }
}
