//! Authentication utilities and password encoding

use crate::models::auth::{LoginState, PasswordEncoding};
use base64::{engine::general_purpose, Engine as _};
use sha2::{Digest, Sha256};

/// Password encoder for different Huawei authentication types
pub struct PasswordEncoder;

impl PasswordEncoder {
    /// Encode password based on the login state requirements
    pub fn encode_password(password: &str, login_state: &LoginState) -> String {
        match login_state.password_encoding() {
            PasswordEncoding::Base64 | PasswordEncoding::Base64AfterChange => {
                Self::encode_base64(password)
            }
            PasswordEncoding::Sha256 => Self::encode_sha256(password),
            PasswordEncoding::Unknown => {
                Self::encode_sha256(password)
            }
        }
    }

    /// Encode password using BASE64
    fn encode_base64(password: &str) -> String {
        general_purpose::STANDARD.encode(password.as_bytes())
    }

    /// Encode password using SHA256
    fn encode_sha256(password: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::auth::LoginState;
    use crate::models::{LoginStatus, LockStatus};

    fn create_test_login_state(password_type: &str) -> LoginState {
        LoginState {
            password_type: password_type.to_string(),
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
        }
    }

    #[test]
    fn test_base64_encoding() {
        let login_state = create_test_login_state("0");
        let password = "admin";
        let encoded = PasswordEncoder::encode_password(password, &login_state);
        assert_eq!(encoded, "YWRtaW4=");
    }

    #[test]
    fn test_sha256_encoding() {
        let login_state = create_test_login_state("4");
        let password = "admin";
        let encoded = PasswordEncoder::encode_password(password, &login_state);
        
        assert_eq!(encoded.len(), 64);
        assert!(encoded.chars().all(|c| c.is_ascii_hexdigit()));
        let expected = "8c6976e5b5410415bde908bd4dee15dfb167a9c873fc4bb8a81f6f2ab448a918";
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_unknown_type_defaults_to_sha256() {
        let login_state = create_test_login_state("999"); // Unknown type
        let password = "admin";
        let encoded = PasswordEncoder::encode_password(password, &login_state);
        assert_eq!(encoded.len(), 64);
        let expected = "8c6976e5b5410415bde908bd4dee15dfb167a9c873fc4bb8a81f6f2ab448a918";
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_base64_after_change_encoding() {
        let login_state = create_test_login_state("3");
        let password = "newpassword";
        let encoded = PasswordEncoder::encode_password(password, &login_state);
        let expected = general_purpose::STANDARD.encode("newpassword".as_bytes());
        assert_eq!(encoded, expected);
    }
}