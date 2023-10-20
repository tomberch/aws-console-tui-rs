use crate::state::appstate::ComponentType;

#[derive(Debug, Clone)]
pub enum ProfileAction {
    SelectProfile { profile_name: String },
}

#[derive(Debug, Clone)]
pub enum Action {
    SetFocus { component_type: ComponentType },
    ProfileAction { action: ProfileAction },
}
