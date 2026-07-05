use lighty_launcher::{
    Authenticator, UserProfile,
    auth::{self},
};
use secrecy::ExposeSecret;

#[derive(Debug, Clone, PartialEq)]
pub enum AuthProvider {
    Offline,
    Microsoft {
        client_id: String,
        refresh_token: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserRole {
    pub name: String,
    pub color: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MyUserProfile {
    pub id: Option<u64>,
    pub username: String,
    pub uuid: String,
    pub access_token: Option<String>,
    pub xuid: Option<String>,
    pub email: Option<String>,
    pub email_verified: bool,
    pub money: Option<f64>,
    pub banned: bool,
    pub provider: AuthProvider,
}

impl ToString for MyUserProfile {
    fn to_string(&self) -> String {
        String::from(format!("{:?} {:?}", self.username, self.provider))
    }
}

pub fn to_user_profile(user: UserProfile) -> MyUserProfile {
    let provider = match user.provider {
        auth::AuthProvider::Microsoft {
            client_id,
            refresh_token,
        } => AuthProvider::Microsoft {
            client_id,
            refresh_token: refresh_token
                .unwrap_or_default()
                .expose_secret()
                .to_string(),
        },
        auth::AuthProvider::Offline => AuthProvider::Offline,
        _ => AuthProvider::Offline,
    };

    MyUserProfile {
        id: user.id,
        username: user.username,
        uuid: user.uuid,
        access_token: Some(
            user.access_token
                .unwrap_or_default()
                .expose_secret()
                .to_string(),
        ),
        xuid: user.xuid,
        email: user.email,
        email_verified: user.email_verified,
        money: user.money,
        banned: user.banned,
        provider: provider,
    }
}

pub async fn create_online_account() -> Result<MyUserProfile, String> {
    // generate client id in microsoft site
    let mut auth = auth::MicrosoftAuth::new("00000000402b5328");
    auth.set_device_code_callback(|code, url| {
        println!("visit: {}", url);
        println!("code: {code}")
    });
    match auth.authenticate(None).await {
        Ok(user) => Ok(to_user_profile(user)),
        Err(err) => Err(format!("{}", err)),
    }
}

pub async fn create_offline_account(username: String) -> Result<MyUserProfile, String> {
    let mut auth = auth::OfflineAuth::new(username);
    match auth.authenticate(None).await {
        Ok(user) => Ok(to_user_profile(user)),
        Err(err) => Err(format!("{}", err)),
    }
}
