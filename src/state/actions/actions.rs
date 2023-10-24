use crate::state::appstate::{AWSService, ComponentType};

#[derive(Debug, Clone)]
pub enum ProfileAction {
    SelectProfile { profile_name: String },
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
pub enum Action {
    SetFocus { component_type: ComponentType },
    Profile { action: ProfileAction },
    Region { action: RegionAction },
    Service { action: ServiceAction },
}
