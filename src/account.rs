use lighty_launcher::{
    Authenticator, UserProfile,
    auth::{self, AuthProvider, UserRole},
};
use secrecy::{ExposeSecret, SecretBox};

#[derive(Debug, Clone, PartialEq)]
pub enum MyAuthProvider {
    Offline,
    Microsoft {
        client_id: String,
        refresh_token: String,
    },
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MyUserRole {
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
    pub role: Option<MyUserRole>,
    pub banned: bool,
    pub provider: MyAuthProvider,
}

impl Default for MyUserProfile {
    fn default() -> Self {
        Self {
            id: None,
            username: "".into(),
            uuid: "".into(),
            access_token: None,
            xuid: None,
            email: None,
            email_verified: false,
            money: None,
            role: None,
            banned: false,
            provider: MyAuthProvider::Offline,
        }
    }
}

impl ToString for MyUserProfile {
    fn to_string(&self) -> String {
        String::from(format!("{:?} {:?}", self.username, self.provider))
    }
}

pub fn to_my_user_profile(user: UserProfile) -> MyUserProfile {
    let provider = match user.provider {
        auth::AuthProvider::Microsoft {
            client_id,
            refresh_token,
        } => MyAuthProvider::Microsoft {
            client_id,
            refresh_token: refresh_token
                .unwrap_or_default()
                .expose_secret()
                .to_string(),
        },
        auth::AuthProvider::Offline => MyAuthProvider::Offline,
        _ => MyAuthProvider::Offline,
    };
    let mut role: MyUserRole = MyUserRole::default();

    if let Some(r) = user.role {
        role = MyUserRole {
            name: r.name,
            color: r.color,
        };
    }

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
        role: Some(role),
        banned: user.banned,
        provider: provider,
    }
}

pub fn to_user_profile(user: MyUserProfile) -> UserProfile {
    let provider = match &user.provider {
        crate::account::MyAuthProvider::Microsoft {
            client_id,
            refresh_token,
        } => AuthProvider::Microsoft {
            client_id: client_id.to_string(),
            refresh_token: Some(SecretBox::new(String::from(refresh_token).into_boxed_str())),
        },
        crate::account::MyAuthProvider::Offline => AuthProvider::Offline,
    };
    let mut role = None;
    if let Some(r) = user.role {
        role = Some(UserRole {
            name: r.name,
            color: r.color,
        });
    }

    UserProfile {
        id: user.id,
        username: user.username,
        uuid: user.uuid,
        access_token: Some(SecretBox::new(
            user.access_token
                .unwrap_or_else(|| String::new())
                .into_boxed_str(),
        )),
        xuid: user.xuid,
        email: user.email,
        email_verified: user.email_verified,
        money: user.money,
        role: role,
        provider,
        banned: user.banned,
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
        Ok(user) => Ok(to_my_user_profile(user)),
        Err(err) => Err(format!("{}", err)),
    }
}

pub async fn create_offline_account(username: String) -> Result<MyUserProfile, String> {
    let mut auth = auth::OfflineAuth::new(username);
    match auth.authenticate(None).await {
        Ok(user) => Ok(to_my_user_profile(user)),
        Err(err) => Err(format!("{}", err)),
    }
}
