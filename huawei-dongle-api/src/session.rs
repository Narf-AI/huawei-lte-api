//! Session management and CSRF token handling

use crate::error::{Error, Result};
use reqwest::Client as HttpClient;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, trace};
use url::Url;

/// Session state for managing authentication and CSRF tokens
#[derive(Debug, Clone, Default)]
pub struct SessionState {
    /// Current CSRF token
    pub csrf_token: Option<String>,
    /// Session cookies are managed by reqwest's cookie store
    pub is_authenticated: bool,
    /// Username of the authenticated user
    pub username: Option<String>,
    /// Last authentication time
    pub last_auth_time: Option<chrono::DateTime<chrono::Utc>>,
}

/// Session manager handles CSRF tokens and authentication state
#[derive(Debug)]
pub struct SessionManager {
    http_client: HttpClient,
    base_url: Url,
    state: Arc<RwLock<SessionState>>,
}

impl SessionManager {
    pub fn new(http_client: HttpClient, base_url: Url) -> Self {
        Self {
            http_client,
            base_url,
            state: Arc::new(RwLock::new(SessionState::default())),
        }
    }

    /// Get the current CSRF token, fetching one if needed
    pub async fn get_csrf_token(&self) -> Result<String> {
        {
            let state = self.state.read().await;
            if let Some(ref token) = state.csrf_token {
                trace!("Using cached CSRF token");
                return Ok(token.clone());
            }
        }

        self.refresh_csrf_token().await
    }

    /// Refresh the CSRF token by fetching from the token endpoint
    pub async fn refresh_csrf_token(&self) -> Result<String> {
        debug!("Fetching new CSRF token from /api/webserver/token");

        match self.try_api_token().await {
            Ok(token) => {
                debug!("Successfully fetched token from API endpoint");
                return Ok(token);
            }
            Err(e) => {
                debug!("API token fetch failed: {}, trying homepage fallback", e);
            }
        }

        self.try_homepage_token().await
    }

    /// Try to get CSRF token from the API endpoint
    async fn try_api_token(&self) -> Result<String> {
        let url = self.base_url.join("/api/webserver/token")?;
        let response = self.http_client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(Error::session(format!(
                "Failed to fetch token: HTTP {}",
                response.status()
            )));
        }

        let xml = response.text().await?;
        trace!("Token response XML: {}", xml);

        let token = self.extract_token_from_xml(&xml)?;

        {
            let mut state = self.state.write().await;
            state.csrf_token = Some(token.clone());
        }

        Ok(token)
    }

    /// Try to get CSRF token from homepage HTML
    async fn try_homepage_token(&self) -> Result<String> {
        debug!("Fetching CSRF token from homepage HTML");

        let response = self.http_client.get(self.base_url.clone()).send().await?;

        if !response.status().is_success() {
            return Err(Error::session(format!(
                "Failed to fetch homepage: HTTP {}",
                response.status()
            )));
        }

        let html = response.text().await?;
        trace!("Homepage HTML length: {} chars", html.len());

        let token = self.extract_token_from_html(&html)?;

        {
            let mut state = self.state.write().await;
            state.csrf_token = Some(token.clone());
        }

        debug!("Successfully extracted token from homepage HTML");
        Ok(token)
    }


    fn extract_token_from_xml(&self, xml: &str) -> Result<String> {
        use quick_xml::events::Event;
        use quick_xml::Reader;

        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);

        let mut buf = Vec::new();
        let mut in_token = false;

        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(ref e) if e.name().as_ref() == b"token" => {
                    in_token = true;
                }
                Event::Text(e) if in_token => {
                    let token = e.unescape()?.into_owned();
                    return Ok(token);
                }
                Event::End(ref e) if e.name().as_ref() == b"token" => {
                    in_token = false;
                }
                Event::Eof => break,
                _ => (),
            }
            buf.clear();
        }

        Err(Error::session("Could not find token in XML response"))
    }

    /// Extract CSRF token from HTML homepage
    fn extract_token_from_html(&self, html: &str) -> Result<String> {
        use scraper::{Html, Selector};

        let document = Html::parse_document(html);
        
        let meta_selector = Selector::parse(r#"meta[name="csrf_token"]"#)
            .map_err(|_| Error::session("Invalid CSS selector"))?;
        
        if let Some(meta_element) = document.select(&meta_selector).next() {
            if let Some(content) = meta_element.value().attr("content") {
                if !content.is_empty() {
                    return Ok(content.to_string());
                }
            }
        }

        let token_selector = Selector::parse(r#"meta[content*="csrf"]"#)
            .map_err(|_| Error::session("Invalid CSS selector"))?;
        
        if let Some(meta_element) = document.select(&token_selector).next() {
            if let Some(content) = meta_element.value().attr("content") {
                if !content.is_empty() {
                    return Ok(content.to_string());
                }
            }
        }

        let all_meta_selector = Selector::parse("meta[content]")
            .map_err(|_| Error::session("Invalid CSS selector"))?;
        
        for meta_element in document.select(&all_meta_selector) {
            if let Some(content) = meta_element.value().attr("content") {
                if content.len() > 20 && content.chars().all(|c| c.is_alphanumeric()) {
                    debug!("Found potential token in meta tag: {}...", &content[..10]);
                    return Ok(content.to_string());
                }
            }
        }

        Err(Error::session("Could not find CSRF token in HTML"))
    }

    pub async fn clear_session(&self) {
        let mut state = self.state.write().await;
        state.csrf_token = None;
        state.is_authenticated = false;
        state.username = None;
        state.last_auth_time = None;
        debug!("Session state cleared");
    }

    pub async fn is_authenticated(&self) -> bool {
        let state = self.state.read().await;
        state.is_authenticated
    }

    /// Mark session as invalidated (e.g., after getting 401)
    pub async fn invalidate_session(&self) {
        debug!("Session invalidated, will need to re-authenticate");
        self.clear_session().await;
    }

    /// Mark user as authenticated
    pub async fn mark_authenticated(&self, username: &str) {
        let mut state = self.state.write().await;
        state.is_authenticated = true;
        state.username = Some(username.to_string());
        state.last_auth_time = Some(chrono::Utc::now());
        debug!("User '{}' marked as authenticated", username);
    }

    pub async fn current_username(&self) -> Option<String> {
        let state = self.state.read().await;
        state.username.clone()
    }

    pub async fn last_auth_time(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        let state = self.state.read().await;
        state.last_auth_time
    }

    /// Check if the session is expired (based on time)
    pub async fn is_session_expired(&self, max_age_minutes: u64) -> bool {
        let state = self.state.read().await;
        if let Some(last_auth) = state.last_auth_time {
            let now = chrono::Utc::now();
            let age = now.signed_duration_since(last_auth);
            age.num_minutes() > max_age_minutes as i64
        } else {
            true
        }
    }

    /// Update CSRF token from response headers if available
    pub async fn update_token_from_headers(&self, headers: &reqwest::header::HeaderMap) {
        let token_headers = [
            "__RequestVerificationToken",
            "__RequestVerificationTokenone", 
            "__RequestVerificationTokentwo",
        ];

        for header_name in &token_headers {
            if let Some(token_value) = headers.get(*header_name) {
                if let Ok(token_str) = token_value.to_str() {
                    if !token_str.is_empty() {
                        let mut state = self.state.write().await;
                        state.csrf_token = Some(token_str.to_string());
                        debug!("Updated CSRF token from response header {}: {}", header_name, token_str);
                        return;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::HeaderMap;

    #[tokio::test]
    async fn test_update_token_from_headers() {
        let http_client = reqwest::Client::new();
        let base_url = Url::parse("http://192.168.8.1").unwrap();
        let session = SessionManager::new(http_client, base_url);
        
        let mut state = session.state.write().await;
        state.csrf_token = Some("old_token".to_string());
        drop(state);

        let mut headers = HeaderMap::new();
        headers.insert("__RequestVerificationToken", "new_token".parse().unwrap());

        session.update_token_from_headers(&headers).await;

        let state = session.state.read().await;
        assert_eq!(state.csrf_token, Some("new_token".to_string()));
    }

    #[tokio::test]
    async fn test_update_token_from_headers_alternate_names() {
        let http_client = reqwest::Client::new();
        let base_url = Url::parse("http://192.168.8.1").unwrap();
        let session = SessionManager::new(http_client, base_url);

        let mut headers = HeaderMap::new();
        headers.insert("__RequestVerificationTokenone", "token_one".parse().unwrap());
        session.update_token_from_headers(&headers).await;
        
        let state = session.state.read().await;
        assert_eq!(state.csrf_token, Some("token_one".to_string()));
        drop(state);

        let mut headers = HeaderMap::new();
        headers.insert("__RequestVerificationTokentwo", "token_two".parse().unwrap());
        session.update_token_from_headers(&headers).await;
        
        let state = session.state.read().await;
        assert_eq!(state.csrf_token, Some("token_two".to_string()));
    }

    #[tokio::test]
    async fn test_no_token_update_when_missing() {
        let http_client = reqwest::Client::new();
        let base_url = Url::parse("http://192.168.8.1").unwrap();
        let session = SessionManager::new(http_client, base_url);
        
        let mut state = session.state.write().await;
        state.csrf_token = Some("existing_token".to_string());
        drop(state);

        let headers = HeaderMap::new();
        session.update_token_from_headers(&headers).await;

        let state = session.state.read().await;
        assert_eq!(state.csrf_token, Some("existing_token".to_string()));
    }
}
