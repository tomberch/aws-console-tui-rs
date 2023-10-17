use crate::{repository::login, state::state::AppState};

use super::actions::ProfileAction;

pub struct ProfileActionHandler;

impl ProfileActionHandler {
    pub fn handle(action: ProfileAction, app_state: &mut AppState) {
        match action {
            ProfileAction::SelectProfile { name } => {
                ProfileActionHandler::handle_select_profile(&name, app_state);
            }
            _ => {}
        }
    }
    fn handle_select_profile(name: &str, app_state: &mut AppState) {}
}
