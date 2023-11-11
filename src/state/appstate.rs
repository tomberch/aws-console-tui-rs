use std::{collections::HashMap, time::Duration};

use aws_config::SdkConfig;
use ratatui::style::Color;

use crate::{
    config::config::{AWSConfig, AppConfig},
    repository::profile::get_available_profiles,
    ui::config::{MenuItemText, TUI_CONFIG},
};

use super::cloud_watch_logs_state::CloudWatchState;

#[derive(Clone, Debug, PartialEq)]
pub enum ComponentType {
    Home,
    Profiles,
    Regions,
    Services,
    Status,
    AWSService,
    //  NoAWSService,
    //  CloudWatch,
}

#[derive(Clone, Debug)]
pub enum ProfileSource {
    Environment,
    CredentialsFile,
    ConfigFile,
}

#[derive(Debug, Clone)]
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
pub struct ToolbarState {
    pub profile_name: String,
    pub account: String,
    pub user: String,
    pub cpu_usage: String,
    pub memory_usage: String,
    pub menu: Vec<MenuItem>,
}

#[derive(Clone, Debug, Default)]
pub struct MenuItem {
    pub command: String,
    pub title: String,
    pub common_command: bool,
}

impl From<MenuItemText<'_>> for MenuItem {
    fn from(val: MenuItemText<'_>) -> Self {
        MenuItem {
            command: val.command.into(),
            title: val.title.into(),
            common_command: val.common_command,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct StatusState {
    pub action_pending: bool,
    pub message: String,
    pub err_message: String,
    pub err_message_backtrace: String,
    pub breadcrumbs: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct MeasureState {
    pub render_duration: String,
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub aws_config: AWSConfig,
    pub focus_component: ComponentType,
    pub active_profile: Option<Profile>,
    pub profile_state: ProfilesState,
    pub region_state: RegionsState,
    pub toolbar_state: ToolbarState,
    pub status_state: StatusState,
    pub measure_state: MeasureState,
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
            toolbar_state: ToolbarState {
                profile_name: "none".into(),
                account: "none".into(),
                user: "none".into(),
                cpu_usage: String::default(),
                memory_usage: String::default(),
                menu: vec![
                    TUI_CONFIG.menu.back_tab.into(),
                    TUI_CONFIG.menu.tab.into(),
                    TUI_CONFIG.menu.quit.into(),
                ],
            },
            status_state: StatusState {
                action_pending: false,
                message: "No profile active. Please select profile and press <Enter>".into(),
                err_message: "".into(),
                err_message_backtrace: "".into(),
                breadcrumbs: vec![TUI_CONFIG.breadcrumbs.profiles.into()],
            },
            measure_state: MeasureState::default(),
            cloud_watch_state: CloudWatchState::default(),
        }
    }
}
