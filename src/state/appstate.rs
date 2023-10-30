use std::collections::HashMap;

use aws_config::SdkConfig;

use crate::{
    config::config::{AWSConfig, AppConfig},
    repository::profile::get_available_profiles,
};

use super::cloud_watch_logs_state::CloudWatchState;

#[derive(Clone, Debug)]
pub enum ComponentType {
    Home,
    Profiles,
    Regions,
    Services,
    Status,
    AWSService,
    NoAWSService,
    CloudWatch,
}

#[derive(Clone, Debug)]
pub enum ProfileSource {
    Environment,
    CredentialsFile,
    ConfigFile,
}

#[derive(Clone, Debug)]
pub struct Profile {
    pub name: String,
    pub source: ProfileSource,
    pub sdk_config: SdkConfig,
    pub account: String,
    pub user: String,
    pub err_message: String,
    pub err_message_backtrace: String,
    pub regions: Vec<String>,
    pub selected_region: Option<String>,
    pub selected_service: AWSService,
}

#[derive(Clone, Debug)]
pub struct ProfilesState {
    pub profile_names: HashMap<String, ProfileSource>,
    pub profiles: HashMap<String, Profile>,
}

#[derive(Clone, Debug)]
pub struct RegionsState {
    pub region_names: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AWSService {
    None,
    CloudWatchLogs,
    Eks,
    DynamoDB,
    S3,
    ServiceCatalog,
}

#[derive(Clone, Debug, Default)]
pub struct StatusState {
    pub action_pending: bool,
    pub message: String,
    pub err_message: String,
    pub err_message_backtrace: String,
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub aws_config: AWSConfig,
    pub focus_component: ComponentType,
    pub active_profile: Option<Profile>,
    pub profile_state: ProfilesState,
    pub region_state: RegionsState,
    pub status_state: StatusState,
    pub cloud_watch_state: CloudWatchState,
}

impl AppState {
    pub fn new(app_config: &AppConfig) -> Self {
        AppState {
            aws_config: app_config.aws.clone(),
            focus_component: ComponentType::Profiles,
            active_profile: None,
            profile_state: ProfilesState {
                profile_names: get_available_profiles(&app_config.aws).unwrap(),
                profiles: HashMap::new(),
            },
            region_state: RegionsState {
                region_names: vec![],
            },
            status_state: StatusState {
                action_pending: false,
                message: "No profile active. Please select profile and press <Enter>".into(),
                err_message: "".into(),
                err_message_backtrace: "".into(),
            },
            cloud_watch_state: CloudWatchState::default(),
        }
    }
}
