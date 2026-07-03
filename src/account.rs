use lighty_launcher::{
    Authenticator, UserProfile,
    auth::{self},
};
use secrecy::SecretBox;


#[derive(Debug, PartialEq)]
enum AuthProvider {
    Offline,
    Microsoft {
        client_id: String,
        refresh_token: String
    }
}

#[derive(Debug, PartialEq)]
struct UserRole {
    pub name: String,
    pub color: Option<String>
}

#[derive(Debug, PartialEq)]
pub struct MyUserProfile {
    pub id: Option<u64>,
    pub username: String,
    pub uuid: String,
    pub access_token: Option<String>,
    xuid: Option<String>,
    pub email: Option<String>,
    pub email_verified: bool,
    pub money: Option<f64>,
    pub role: Option<UserRole>,
    pub banned: bool,
    pub provider: AuthProvider
}

pub async fn create_online_account() -> Result<UserProfile, String> {
    // generate client id in microsoft site
    let mut auth = auth::MicrosoftAuth::new("");
    match auth.authenticate(None).await {
        Ok(user) => Ok(user),
        Err(err) => Err(format!("{}", err)),
    }
}

pub async fn create_offline_account(username: String) -> Result<UserProfile, String> {
    let mut auth = auth::OfflineAuth::new(username);
    match auth.authenticate(None).await {
        Ok(user) => Ok(user),
        Err(err) => Err(format!("{}", err)),
    }
}
