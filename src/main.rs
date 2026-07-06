mod account;
mod instances;
mod ui;

use iced::{Element, Task, widget::container};
use lighty_launcher::{JavaDistribution, Loader, VersionBuilder, core::AppState};

use crate::{
    account::{MyUserProfile, create_offline_account, create_online_account},
    instances::run_instance,
    ui::{
        account_pages::{AccountEvent, LoginMode},
        instance_page::{Instance, MyLoader, to_loader},
    },
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
    instances: Vec<VersionBuilder<Loader>>,
    new_instance: Instance,
    java_distribution: JavaDistribution,
}

impl ClientState {
    fn new() -> Self {
        Self {
            current_page: Pages::AccountPage,
            name_input: String::new(),
            current_login_mode: LoginMode::SelectMode,
            accounts: vec![],
            current_user: None,
            instances: vec![],
            new_instance: Instance::default(),
            java_distribution: JavaDistribution::Temurin,
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
                    let username = self.name_input.to_string();
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
                if let Ok(new_user) = result {
                    if !self
                        .accounts
                        .iter()
                        .any(|user| user.username == new_user.username)
                    {
                        self.accounts.push(new_user);
                        self.current_login_mode = LoginMode::SelectMode;
                    } else {
                        self.current_login_mode = LoginMode::SelectMode;
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
            Message::ConfigNewInstance(config) => {
                match config {
                    ConfigInstance::InstanceName(name) => self.new_instance.instance_name = name,
                    ConfigInstance::Loader(loader) => {
                        self.new_instance.loader = Some(loader);
                    }
                    ConfigInstance::LoaderVersion(version) => {
                        self.new_instance.loader_version = version;
                    }
                    ConfigInstance::MinecraftVersion(version) => {
                        self.new_instance.minecraft_version = version;
                    }
                    ConfigInstance::CreateInstance => {
                        let ins = self.new_instance.clone();

                        let new_instance: VersionBuilder<Loader> = VersionBuilder::new(
                            &ins.instance_name,
                            to_loader(ins.loader.unwrap_or(MyLoader::Vanilla)),
                            &ins.loader_version,
                            &ins.minecraft_version,
                        );

                        self.instances.push(new_instance);
                    }
                }
                Task::none()
            }
            Message::RunInstance(ins) => {
                if let Some(user) = self.current_user.clone() {
                    let java = match self.java_distribution {
                        JavaDistribution::Temurin => JavaDistribution::Temurin,
                        JavaDistribution::GraalVM => JavaDistribution::GraalVM,
                        JavaDistribution::Liberica => JavaDistribution::Liberica,
                        JavaDistribution::Zulu => JavaDistribution::Zulu,
                    };

                    return Task::perform(run_instance(user, ins, java), |_| {
                        Message::InstanceRunning(())
                    });
                }
                Task::none()
            }
            Message::InstanceRunning(()) => Task::none(),
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
enum ConfigInstance {
    InstanceName(String),
    Loader(MyLoader),
    LoaderVersion(String),
    MinecraftVersion(String),
    CreateInstance,
}

#[derive(Debug, Clone)]
enum Message {
    None,
    Page(Pages),
    Account(AccountEvent),
    AuthMode(LoginMode),
    NameChanged(String),
    Logged(Result<MyUserProfile, String>),
    ConfigNewInstance(ConfigInstance),
    RunInstance(VersionBuilder<Loader>),
    InstanceRunning(()),
}

#[derive(Debug, Clone)]
enum Pages {
    AccountPage,
    InstancePage,
}
