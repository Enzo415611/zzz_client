use iced::{
    Alignment, Element,
    Length::{self, Fill},
    widget::{button, column, container, pick_list, row, text, text_input},
};

use crate::{ClientState, Message, account::MyUserProfile};

#[derive(Debug, Clone)]
pub enum AccountEvent {
    CreateOnlineAccount,
    CreateOfflineAccount,
    LoginAccount(MyUserProfile),
}

#[derive(Debug, Clone)]
pub enum LoginMode {
    SelectMode,
    Online,
    Offline,
}

impl ClientState {
    pub fn account_page(&self) -> Element<'_, Message> {
        let column = match self.current_login_mode {
            LoginMode::SelectMode => column![
                row![
                    button("Online Account").on_press(Message::AuthMode(LoginMode::Online)),
                    button("Offline Account").on_press(Message::AuthMode(LoginMode::Offline))
                ]
                .spacing(10)
                .align_y(Alignment::Center),
                pick_list(self.accounts.clone(), self.current_user.clone(), |user| {
                    Message::Account(AccountEvent::LoginAccount(user))
                }).width(200).menu_height(Fill)
            ]
            .spacing(10)
            .height(Fill)
            .align_x(Alignment::Center),
            LoginMode::Online => column![self.login_online_account()].align_x(Alignment::Center),
            LoginMode::Offline => column![self.create_offline_account()].align_x(Alignment::Center),
        };

        container(column).center(Length::Fill).into()
    }

    fn login_online_account(&self) -> Element<'_, Message> {
        // manda pra um link da microsoft
        container(
            row![
                button("<").on_press(Message::AuthMode(LoginMode::SelectMode)),
                text("login em wwww")
            ]
            .height(Fill)
            .align_y(Alignment::Center),
        )
        .into()
    }

    fn create_offline_account(&self) -> Element<'_, Message> {
        container(
            row![
                button("<").on_press(Message::AuthMode(LoginMode::SelectMode)),
                text_input("Account Name", &self.name_input).on_input(Message::NameChanged),
                button("Create").on_press(Message::Account(AccountEvent::CreateOfflineAccount))
            ]
            .align_y(Alignment::Center),
        )
        .into()
    }
}
