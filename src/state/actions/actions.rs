#[derive(Debug, Clone)]
pub enum ProfileAction {
    SelectProfile { name: String },
}

#[derive(Debug, Clone)]
pub enum Action {
    ProfileAction { action: ProfileAction },
}
