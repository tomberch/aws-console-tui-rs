use tokio::sync::mpsc::UnboundedSender;

use crate::ui::config::TUI_CONFIG;

use super::appstate::AppState;

pub mod actions;
pub mod cloud_watch_logs_action_handler;
pub mod profile_action_handler;
pub mod region_action_handler;
pub mod service_action_handler;

pub struct ActionHandler;
impl ActionHandler {
    fn initiate_action_pending(state_tx: UnboundedSender<AppState>, app_state: &mut AppState) {
        app_state.status_state.action_pending = true;
        app_state.status_state.message = TUI_CONFIG.messages.pending_action.into();
        app_state.status_state.err_message = "".into();
        let _ = state_tx.send(app_state.clone());
    }

    fn release_action_pending(state_tx: UnboundedSender<AppState>, app_state: &mut AppState) {
        if app_state.status_state.err_message.is_empty() {
            app_state.status_state.message = "".into();
        }
        app_state.status_state.action_pending = false;
        let _ = state_tx.send(app_state.clone());
    }
}
