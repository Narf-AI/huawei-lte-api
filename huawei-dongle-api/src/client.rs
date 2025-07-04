//! HTTP client for Huawei Dongle API
//! 
//! The [`Client`] is the main entry point for interacting with Huawei LTE devices.
//! It manages HTTP connections, session state, and provides access to all API endpoints.
//! 
//! # Examples
//! 
//! ```no_run
//! use huawei_dongle_api::{Client, Config};
//! 
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create client with default settings (http://192.168.8.1)
//! let client = Client::new(Config::default())?;
//! 
//! // Or create with custom URL
//! let client = Client::for_url("http://192.168.1.1")?;
//! 
//! // Access API endpoints through the client
//! let device_info = client.device().information().await?;
//! let sms_count = client.sms().count().await?;
//! # Ok(())
//! # }
//! ```

use crate::{
    api,
    config::Config,
    error::{Error, Result},
    models::common::check_for_api_error,
    retry::RetryStrategy,
    session::SessionManager,
};
use reqwest::{Client as HttpClient, ClientBuilder, Response};
use tracing::{debug, trace};
use url::Url;

/// Main client for interacting with Huawei LTE dongles.
/// 
/// The client handles:
/// - HTTP connection pooling
/// - Session management and CSRF tokens
/// - Automatic retry on transient failures
/// - Error recovery and session refresh
/// 
/// # Thread Safety
/// 
/// The client is thread-safe and can be shared across multiple tasks using `Arc`:
/// 
/// ```no_run
/// use std::sync::Arc;
/// use huawei_dongle_api::{Client, Config};
/// 
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Arc::new(Client::new(Config::default())?);
/// 
/// // Clone the Arc for use in multiple tasks
/// let client2 = client.clone();
/// tokio::spawn(async move {
///     let status = client2.monitoring().status().await;
/// });
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct Client {
    http_client: HttpClient,
    config: Config,
    session: SessionManager,
    retry_strategy: RetryStrategy,
}

impl Client {
    /// Create a new client with the given configuration.
    /// 
    /// # Arguments
    /// 
    /// * `config` - Configuration for the client
    /// 
    /// # Errors
    /// 
    /// Returns an error if the HTTP client cannot be created.
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// use huawei_dongle_api::{Client, Config};
    /// 
    /// let config = Config::default();
    /// let client = Client::new(config)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(config: Config) -> Result<Self> {
        let http_client = ClientBuilder::new()
            .cookie_store(true)
            .timeout(config.timeout)
            .user_agent(&config.user_agent)
            .build()?;

        let session = SessionManager::new(http_client.clone(), config.base_url.clone());

        let retry_strategy = RetryStrategy {
            max_attempts: config.max_retries,
            initial_delay: config.retry_delay,
            max_delay: config.max_retry_delay,
            ..Default::default()
        };

        Ok(Self {
            http_client,
            config,
            session,
            retry_strategy,
        })
    }

    /// Create a client with default configuration
    pub fn with_default_config() -> Result<Self> {
        Self::new(Config::default())
    }

    /// Create a client for a specific URL
    pub fn for_url<S: AsRef<str>>(url: S) -> Result<Self> {
        let config = Config::for_url(url)?;
        Self::new(config)
    }

    pub fn device(&self) -> api::device::DeviceApi {
        api::device::DeviceApi::new(self)
    }

    pub fn monitoring(&self) -> api::monitoring::MonitoringApi {
        api::monitoring::MonitoringApi::new(self)
    }

    pub fn network(&self) -> api::network::NetworkApi {
        api::network::NetworkApi::new(self)
    }

    pub fn sms(&self) -> api::sms::SmsApi {
        api::sms::SmsApi::new(self)
    }

    pub fn dhcp(&self) -> api::dhcp::DhcpApi {
        api::dhcp::DhcpApi::new(self)
    }

    pub fn auth(&self) -> api::auth::AuthApi {
        api::auth::AuthApi::new(self)
    }

    pub(crate) fn session(&self) -> &SessionManager {
        &self.session
    }


    pub(crate) async fn get(&self, path: &str) -> Result<Response> {
        let url = self.build_url(path)?;
        trace!("GET {}", url);

        self.retry_strategy
            .execute(|| async {
                let response = self.http_client.get(url.clone()).send().await?;
                self.check_response_status(&response).await?;
                self.session.update_token_from_headers(response.headers()).await;
                Ok(response)
            })
            .await
    }

    pub(crate) async fn get_authenticated(&self, path: &str) -> Result<Response> {
        let url = self.build_url(path)?;
        trace!("GET {} (authenticated)", url);

        let result = self.get_authenticated_internal(&url).await;
        match &result {
            Err(Error::CsrfTokenInvalid) | Err(Error::SessionTokenInvalid) => {
                debug!("CSRF/Session error detected, refreshing token and retrying");
                self.session.refresh_csrf_token().await?;
                self.get_authenticated_internal(&url).await
            }
            _ => result
        }
    }

    /// Internal GET implementation
    async fn get_authenticated_internal(&self, url: &Url) -> Result<Response> {
        self.retry_strategy
            .execute(|| async {
                let csrf_token = self.session.get_csrf_token().await?;

                let response = self
                    .http_client
                    .get(url.clone())
                    .header("X-Requested-With", "XMLHttpRequest")
                    .header("__RequestVerificationToken", &csrf_token)
                    .send()
                    .await?;

                self.check_response_status(&response).await?;
                self.session.update_token_from_headers(response.headers()).await;
                Ok(response)
            })
            .await
    }

    pub(crate) async fn post_xml(&self, path: &str, xml_body: &str) -> Result<Response> {
        let url = self.build_url(path)?;
        trace!("POST {} with XML body", url);

        let result = self.post_xml_internal(&url, xml_body).await;
        match &result {
            Err(Error::CsrfTokenInvalid) | Err(Error::SessionTokenInvalid) => {
                debug!("CSRF/Session error detected, refreshing token and retrying");
                self.session.refresh_csrf_token().await?;
                self.post_xml_internal(&url, xml_body).await
            }
            _ => result
        }
    }

    /// Internal POST implementation
    async fn post_xml_internal(&self, url: &Url, xml_body: &str) -> Result<Response> {
        self.retry_strategy
            .execute(|| async {
                let csrf_token = self.session.get_csrf_token().await?;

                let response = self
                    .http_client
                    .post(url.clone())
                    .header(
                        "Content-Type",
                        "application/x-www-form-urlencoded; charset=UTF-8",
                    )
                    .header("X-Requested-With", "XMLHttpRequest")
                    .header("__RequestVerificationToken", &csrf_token)
                    .body(xml_body.to_string())
                    .send()
                    .await?;

                self.check_response_status(&response).await?;
                self.session.update_token_from_headers(response.headers()).await;
                Ok(response)
            })
            .await
    }


    fn build_url(&self, path: &str) -> Result<Url> {
        let path = if path.starts_with('/') {
            path.to_string()
        } else {
            format!("/{}", path)
        };

        Ok(self.config.base_url.join(&path)?)
    }

    /// Check response status and handle common error cases
    async fn check_response_status(&self, response: &Response) -> Result<()> {
        let status = response.status();

        if status.is_success() {
            return Ok(());
        }

        if status == 401 || status == 403 {
            debug!("Authentication error, invalidating session");
            self.session.invalidate_session().await;
            return Err(Error::authentication(format!(
                "Authentication failed: HTTP {}",
                status
            )));
        }

        if status.is_client_error() {
            return Err(Error::api(
                status.as_u16() as i32,
                format!("Client error: HTTP {}", status),
            ));
        }

        if status.is_server_error() {
            return Err(Error::api(
                status.as_u16() as i32,
                format!("Server error: HTTP {}", status),
            ));
        }

        Err(Error::api(
            status.as_u16() as i32,
            format!("Unexpected status: HTTP {}", status),
        ))
    }

    /// Check XML response for API errors and handle them appropriately
    pub(crate) async fn check_xml_for_errors(&self, xml_text: &str) -> Result<()> {
        if let Some(api_error) = check_for_api_error(xml_text) {
            debug!("API error detected: {} - {}", api_error.code, api_error.error_message());

            if api_error.is_csrf_error() || api_error.is_session_error() {
                debug!("Session/CSRF error, invalidating session");
                self.session.invalidate_session().await;
            }

            return Err(Error::api(
                api_error.code.as_int(),
                api_error.error_message(),
            ));
        }
        Ok(())
    }

    /// Execute a POST request with automatic CSRF token refresh on failure
    pub(crate) async fn post_xml_with_retry<F, T>(&self, path: &str, xml_body: &str, parse_fn: F) -> Result<T>
    where
        F: Fn(&str) -> Result<T>,
    {
        let response = self.post_xml(path, xml_body).await?;
        let text = response.text().await?;
        
        match self.check_xml_for_errors(&text).await {
            Ok(()) => parse_fn(&text),
            Err(Error::CsrfTokenInvalid) | Err(Error::SessionTokenInvalid) => {
                debug!("CSRF/Session error in response, refreshing token and retrying");
                self.session.refresh_csrf_token().await?;
                
                let response = self.post_xml(path, xml_body).await?;
                let text = response.text().await?;
                self.check_xml_for_errors(&text).await?;
                parse_fn(&text)
            }
            Err(e) => Err(e),
        }
    }

    /// Execute a GET request with automatic CSRF token refresh on failure
    pub(crate) async fn get_authenticated_with_retry<F, T>(&self, path: &str, parse_fn: F) -> Result<T>
    where
        F: Fn(&str) -> Result<T>,
    {
        let response = self.get_authenticated(path).await?;
        let text = response.text().await?;
        
        match self.check_xml_for_errors(&text).await {
            Ok(()) => parse_fn(&text),
            Err(Error::CsrfTokenInvalid) | Err(Error::SessionTokenInvalid) => {
                debug!("CSRF/Session error in response, refreshing token and retrying");
                self.session.refresh_csrf_token().await?;
                
                let response = self.get_authenticated(path).await?;
                let text = response.text().await?;
                self.check_xml_for_errors(&text).await?;
                parse_fn(&text)
            }
            Err(e) => Err(e),
        }
    }

    pub fn base_url(&self) -> &Url {
        &self.config.base_url
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let config = Config::default();
        let client = Client::new(config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_client_for_url() {
        let client = Client::for_url("http://192.168.62.1");
        assert!(client.is_ok());

        let client = client.unwrap();
        assert_eq!(client.base_url().as_str(), "http://192.168.62.1/");
    }

    #[test]
    fn test_build_url() {
        let client = Client::for_url("http://192.168.8.1").unwrap();

        let url = client.build_url("/api/device/information").unwrap();
        assert_eq!(url.as_str(), "http://192.168.8.1/api/device/information");

        let url = client.build_url("api/device/information").unwrap();
        assert_eq!(url.as_str(), "http://192.168.8.1/api/device/information");
    }
}
