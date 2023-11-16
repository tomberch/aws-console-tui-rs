use std::collections::HashMap;

use aws_config::SdkConfig;

use crate::{
    config::app_config::{AWSConfig, AppConfig},
    repository::profile::get_available_profiles,
    ui::tui_config::MenuItemText,
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
    pub menu_items: [Vec<MenuItem>; 3],
}

#[derive(Clone, Debug, Default)]
pub struct MenuItem {
    pub command: String,
    pub title: String,
    pub color_index: usize,
}

impl From<MenuItemText<'_>> for MenuItem {
    fn from(val: MenuItemText<'_>) -> Self {
        MenuItem {
            command: val.command.into(),
            title: val.title.into(),
            color_index: val.color_index,
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
    pub is_active: bool,
    pub render_duration: String,
    pub action_duration: String,
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub aws_config: AWSConfig,
    pub focus_component: ComponentType,
    pub is_expanded: bool,
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
            is_expanded: true,
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
                menu_items: [vec![], vec![], vec![]],
            },
            status_state: StatusState {
                action_pending: false,
                message: "No profile active. Please select profile and press <Enter>".into(),
                err_message: "".into(),
                err_message_backtrace: "".into(),
                breadcrumbs: vec![],
            },
            measure_state: MeasureState {
                is_active: !app_config.performance.is_empty(),
                ..Default::default()
            },
            cloud_watch_state: CloudWatchState::default(),
        }
    }
}
