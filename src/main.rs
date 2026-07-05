mod account;
mod ui;

use iced::{Element, Task, widget::container};
use lighty_launcher::core::AppState;

use crate::{
    account::{MyUserProfile, create_offline_account, create_online_account},
    ui::account_pages::{AccountEvent, LoginMode},
};

fn main() -> iced::Result {
    AppState::init("ZzzClient").expect("Launcher global state not initialized");
    iced::application(ClientState::new, ClientState::update, ClientState::view).run()
}

#[derive(Debug)]
struct ClientState {
    current_page: Pages,
    name_input: String,
    current_login_mode: LoginMode,
    accounts: Vec<MyUserProfile>,
    current_user: Option<MyUserProfile>,
}

impl ClientState {
    fn new() -> Self {
        Self {
            current_page: Pages::AccountPage,
            name_input: String::new(),
            current_login_mode: LoginMode::SelectMode,
            accounts: vec![],
            current_user: None,
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::None => Task::none(),
            Message::Page(page) => {
                self.current_page = page;
                Task::none()
            }
            Message::Account(event) => match event {
                AccountEvent::CreateOnlineAccount => {
                    Task::perform(create_online_account(), Message::Logged)
                }
                AccountEvent::CreateOfflineAccount => {
                    let username = self.name_input.clone();
                    self.name_input.clear();
                    Task::perform(create_offline_account(username), Message::Logged)
                }
                AccountEvent::LoginAccount(user) => {
                    self.current_user = Some(user);
                    self.current_page = Pages::InstancePage;
                    Task::none()
                }
            },
            Message::Logged(result) => {
                println!("logged");
                if let Ok(new_user) = result {
                    if !self
                        .accounts
                        .iter()
                        .any(|user| user.username == new_user.username)
                    {
                        self.current_user = Some(new_user.clone());
                        self.accounts.push(new_user);
                        self.current_login_mode = LoginMode::SelectMode;
                    } else {
                        self.current_user = Some(new_user);
                        self.current_login_mode = LoginMode::SelectMode;
                        self.current_page = Pages::InstancePage;
                    }
                }
                Task::none()
            }
            Message::AuthMode(mode) => {
                self.current_login_mode = mode;
                Task::none()
            }
            Message::NameChanged(name) => {
                self.name_input = name;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        container(match self.current_page {
            Pages::AccountPage => self.account_page(),
            Pages::InstancePage => self.instance_page(),
        })
        .into()
    }
}

#[derive(Debug, Clone)]
enum Message {
    None,
    Page(Pages),
    Account(AccountEvent),
    AuthMode(LoginMode),
    NameChanged(String),
    Logged(Result<MyUserProfile, String>),
}

#[derive(Debug, Clone)]
enum Pages {
    AccountPage,
    InstancePage,
}
