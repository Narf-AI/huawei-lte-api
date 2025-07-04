//! Authentication API endpoints

use crate::{
    auth::PasswordEncoder,
    client::Client,
    error::{Error, Result},
    models::{auth::*, common::Response},
};
use tracing::{debug, trace};

/// Authentication API for login/logout operations
pub struct AuthApi<'a> {
    client: &'a Client,
}

impl<'a> AuthApi<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// This endpoint does not require authentication.
    /// Returns information about password encoding requirements and current login status.
    pub async fn state_login(&self) -> Result<LoginState> {
        debug!("Fetching login state");

        let response = self.client.get("/api/user/state-login").await?;
        let text = response.text().await?;

        trace!("Login state response: {}", text);

        let state: LoginState = serde_xml_rs::from_str(&text)
            .map_err(|e| Error::generic(format!("Failed to parse login state: {}", e)))?;

        debug!(
            "Login state: {} (password_type: {})",
            if state.is_logged_in() { "logged in" } else { "not logged in" },
            state.password_type
        );

        Ok(state)
    }

    /// This endpoint requires a valid CSRF token but not authentication.
    /// Password will be automatically encoded based on the device requirements.
    pub async fn login(&self, username: &str, password: &str) -> Result<()> {
        debug!("Attempting login for user: {}", username);

        let login_state = self.state_login().await?;

        if login_state.is_logged_in() {
            debug!("User is already logged in");
            return Ok(());
        }

        if login_state.is_locked() {
            return Err(Error::session(format!(
                "Account is locked. Wait time: {} seconds",
                login_state.remain_wait_time
            )));
        }

        let encoded_password = PasswordEncoder::encode_password(password, &login_state);
        
        let request = LoginRequest::new(
            username.to_string(),
            encoded_password,
            login_state.password_type.clone(),
        );

        let xml = serde_xml_rs::to_string(&request)
            .map_err(|e| Error::generic(format!("Failed to serialize login request: {}", e)))?;

        trace!("Login request XML: {}", xml);

        let response = self.client.post_xml("/api/user/login", &xml).await?;
        let text = response.text().await?;

        trace!("Login response: {}", text);

        let result: Response = serde_xml_rs::from_str(&text)
            .map_err(|e| Error::generic(format!("Failed to parse login response: {}", e)))?;

        if !result.is_success() {
            let error_code = result.error_code().unwrap_or(-1);
            let error_message = match error_code {
                108001 => "Username wrong".to_string(),
                108002 => "Password wrong".to_string(),
                108006 => "Username or password wrong".to_string(),
                108007 => "Too many login attempts".to_string(),
                _ => result.error_message().unwrap_or("Login failed").to_string(),
            };

            return Err(Error::api(error_code, error_message));
        }

        self.client.session().mark_authenticated(username).await;
        
        debug!("Login successful for user: {}", username);
        Ok(())
    }

    /// This endpoint requires authentication and a valid CSRF token.
    pub async fn logout(&self) -> Result<()> {
        debug!("Attempting logout");

        let request = LogoutRequest::new();
        let xml = serde_xml_rs::to_string(&request)
            .map_err(|e| Error::generic(format!("Failed to serialize logout request: {}", e)))?;

        let response = self.client.post_xml("/api/user/logout", &xml).await?;
        let text = response.text().await?;

        trace!("Logout response: {}", text);

        let result: Response = serde_xml_rs::from_str(&text)
            .map_err(|e| Error::generic(format!("Failed to parse logout response: {}", e)))?;

        if !result.is_success() {
            return Err(Error::api(
                result.error_code().unwrap_or(-1),
                result
                    .error_message()
                    .unwrap_or("Logout failed")
                    .to_string(),
            ));
        }

        self.client.session().clear_session().await;
        
        debug!("Logout successful");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn test_auth_api_creation() {
        let config = Config::default();
        let client = crate::Client::new(config).unwrap();
        let auth_api = client.auth();

        assert_eq!(
            std::mem::size_of_val(&auth_api),
            std::mem::size_of::<&Client>()
        );
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
    fn test_logout_request_serialization() {
        let request = LogoutRequest::new();
        let xml = serde_xml_rs::to_string(&request).unwrap();
        assert!(xml.contains("<Logout>1</Logout>"));
    }
}