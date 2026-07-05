use lighty_launcher::{
    JavaDistribution, Launch, Loader, VersionBuilder, launch::InstallerError
};

use crate::{
    account::{MyUserProfile, to_user_profile},
};

#[derive(Debug, Clone)]
pub struct Instance {
    pub instance_name: String,
    pub loader: Loader,
    pub loader_version: String,
    pub minecraft_version: String
}

pub async fn run_instance(user: MyUserProfile, mut instance: VersionBuilder<Loader>, java_distribution: JavaDistribution) -> Result<(), InstallerError> {
    instance
        .launch(
            &to_user_profile(&user),
            java_distribution,
        )
        .run()
        .await?;

    Ok(())
}