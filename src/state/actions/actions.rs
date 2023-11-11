use std::time::Duration;

use crate::state::appstate::{AWSService, ComponentType, MenuItem, ProfileSource};

#[derive(Debug, Clone)]
pub enum ProfileAction {
    SelectProfile { profile: (String, ProfileSource) },
}

#[derive(Debug, Clone)]
pub enum RegionAction {
    SelectRegion { region_name: String },
}

#[derive(Debug, Clone)]
pub enum ServiceAction {
    SelectService { service: AWSService },
}

#[derive(Debug, Clone)]
pub enum CloudWatchLogsAction {
    GetLogGroups { token: Option<String> },
}

#[derive(Debug, Clone)]
pub enum Action {
    SetFocus {
        component_type: ComponentType,
        breadcrumbs: Vec<String>,
        menu: Vec<MenuItem>,
    },
    RenderDuration {
        duration: Duration,
    },
    Profile {
        action: ProfileAction,
    },
    Region {
        action: RegionAction,
    },
    Service {
        action: ServiceAction,
    },
    CloudWatchLogs {
        action: CloudWatchLogsAction,
    },
}
