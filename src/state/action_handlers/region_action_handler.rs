use tracing::{event, Level};

use crate::state::appstate::AppState;

use super::actions::RegionAction;

pub struct RegionActionHandler;

impl RegionActionHandler {
    pub fn handle(action: RegionAction, app_state: &mut AppState) {
        match action {
            RegionAction::SelectRegion { region_name } => {
                RegionActionHandler::handle_select_region(region_name, app_state);
            }
        }
    }
    fn handle_select_region(region_name: String, app_state: &mut AppState) {
        app_state
            .active_profile
            .as_mut()
            .map(|active_profile| active_profile.selected_region.insert(region_name.clone()));

        event!(Level::DEBUG, "{:?}", app_state);
    }
}
