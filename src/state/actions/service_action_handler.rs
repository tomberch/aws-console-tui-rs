use tokio::sync::mpsc::UnboundedSender;
use tracing::{event, Level};

use crate::state::appstate::{AWSService, AppState, ComponentType};

use super::actions::ServiceAction;

pub struct ServiceActionHandler;

impl ServiceActionHandler {
    pub async fn handle(
        state_tx: UnboundedSender<AppState>,
        action: ServiceAction,
        app_state: &mut AppState,
    ) {
        match action {
            ServiceAction::SelectService {
                service: aws_service,
            } => {
                ServiceActionHandler::handle_select_service(aws_service, app_state);
                app_state.status_state.action_pending = false;
            }
        }
    }
    fn handle_select_service(service: AWSService, app_state: &mut AppState) {
        if let Some(active_profile) = app_state.active_profile.as_mut() {
            active_profile.selected_service = service;
            app_state.focus_component = ComponentType::AWSService;
        }
        event!(Level::DEBUG, "{:?}", app_state);
    }
}
