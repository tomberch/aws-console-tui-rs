use tokio::sync::mpsc::UnboundedSender;

use crate::{
    state::appstate::{AWSService, AppState},
    ui::config::TUI_CONFIG,
};

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
                app_state.status_state.message = TUI_CONFIG.messages.pending_action.into();
                app_state.status_state.err_message = "".into();
                let _ = state_tx.send(app_state.clone());
                let _result =
                    ServiceActionHandler::handle_select_service(aws_service, app_state).await;
            }
        }
    }
    async fn handle_select_service(service: AWSService, app_state: &mut AppState) {
        app_state.service_state.selected_service = service;
    }
}
