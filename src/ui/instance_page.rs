use iced::{
    Element,
    Length::Fill,
    widget::{Space, button, column, container, row},
};

use crate::{ClientState, Message};

impl ClientState {
    pub fn instance_page(&self) -> Element<'_, Message> {
        let username = self
            .current_user
            .as_ref()
            .map(|user| user.username.as_str())
            .unwrap_or("");

        container(column![
            row![
                Space::new().width(Fill),
                button(username).on_press(Message::None)
            ],
            self.instance_list()
        ])
        .into()
    }

    fn instance_list(&self) -> Element<'_, Message> {
        let list = column![];

        container(list).width(300).height(Fill).into()
    }
}
