use iced::{
    Alignment, Element,
    Length::Fill,
    widget::{Space, button, column, container, pick_list, row, scrollable, text, text_input},
};
use lighty_launcher::Loader;

use crate::{ClientState, Message};

const LOADERS: [MyLoader; 6] = [
    MyLoader::Vanilla,
    MyLoader::Fabric,
    MyLoader::Forge,
    MyLoader::Optifine,
    MyLoader::NeoForge,
    MyLoader::Quilt,
];

#[derive(Debug, PartialEq, Clone)]
pub enum MyLoader {
    Vanilla,
    Fabric,
    Forge,
    Optifine,
    NeoForge,
    Quilt,
}

pub fn to_loader(loader: MyLoader) -> Loader {
    match loader {
        MyLoader::Vanilla => Loader::Vanilla,
        MyLoader::Fabric => Loader::Fabric,
        MyLoader::Forge => Loader::Forge,
        MyLoader::NeoForge => Loader::NeoForge,
        MyLoader::Optifine => Loader::Optifine,
        MyLoader::Quilt => Loader::Quilt,
    }
}

impl std::fmt::Display for MyLoader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            MyLoader::Vanilla => "Vanilla",
            MyLoader::Fabric => "Fabric",
            MyLoader::Forge => "Forge",
            MyLoader::Optifine => "Optifine",
            MyLoader::NeoForge => "NeoForge",
            MyLoader::Quilt => "Quilt",
        };
        write!(f, "{}", str_val)
    }
}

#[derive(Debug, Clone)]
pub struct Instance {
    pub instance_name: String,
    pub loader: Option<MyLoader>,
    pub loader_version: String,
    pub minecraft_version: Option<String>,
}

impl Default for Instance {
    fn default() -> Self {
        Self {
            instance_name: "ZzzClient".to_string(),
            loader: Some(MyLoader::Vanilla),
            loader_version: "".to_string(),
            minecraft_version: None,
        }
    }
}

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
                button(username).on_press(Message::Page(crate::Pages::AccountPage))
            ],
            row![self.instance_list(), self.create_instance()]
        ])
        .into()
    }

    fn create_instance(&self) -> Element<'_, Message> {
        let instance_name_input = text_input("Instance Name", &self.new_instance.instance_name)
            .on_input(|name| Message::ConfigNewInstance(crate::ConfigInstance::InstanceName(name)));

        let minecraft_version = self.minecraft_versions_list();

        let loader_version_text_input =
            text_input("Loader Version", &self.new_instance.loader_version)
                .on_input(|i| Message::ConfigNewInstance(crate::ConfigInstance::LoaderVersion(i)));

        let column = column![
            instance_name_input,
            minecraft_version,
            self.loader_list(),
            loader_version_text_input,
            button("Create Instance").on_press(Message::ConfigNewInstance(
                crate::ConfigInstance::CreateInstance
            ))
        ];

        container(column).align_x(Alignment::Center).into()
    }

    fn loader_list(&self) -> Element<'_, Message> {
        let list = pick_list(LOADERS, self.new_instance.loader.clone(), |l| {
            Message::ConfigNewInstance(crate::ConfigInstance::Loader(l.clone()))
        });

        list.into()
    }

    fn minecraft_versions_list(&self) -> Element<'_, Message> {
        let list = pick_list(
            self.minecraft_versions.as_ref(),
            self.new_instance.minecraft_version.as_ref(),
            move |v| Message::ConfigNewInstance(crate::ConfigInstance::MinecraftVersion(v)),
        );

        list.into()
    }

    fn instance_list(&self) -> Element<'_, Message> {
        let list = scrollable(column![].extend(self.instances.iter().map(|ins| {
            button(text(format!(
                "{} \n {:?} {} \n {}",
                ins.name, ins.loader, ins.loader_version, ins.minecraft_version
            )))
            .on_press(Message::RunInstance(ins.clone()))
            .into()
        })));

        list.into()
    }
}
