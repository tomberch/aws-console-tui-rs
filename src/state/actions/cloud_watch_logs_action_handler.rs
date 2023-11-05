use tracing::{event, Level};

use crate::{
    repository::cloud_watch_logs::CloudWatchLogsRepository,
    state::{appstate::AppState, cloud_watch_logs_state::CloudWatchState},
    ui::config::TUI_CONFIG,
};

use super::actions::CloudWatchLogsAction;

pub struct CloudWatchLogsActionHandler;

impl CloudWatchLogsActionHandler {
    pub async fn handle(action: CloudWatchLogsAction, app_state: &mut AppState) {
        match action {
            CloudWatchLogsAction::GetLogGroups { token } => {
                CloudWatchLogsActionHandler::handle_get_log_groups(token, app_state).await;
            }
        }
    }

    async fn handle_get_log_groups(next_token: Option<String>, app_state: &mut AppState) {
        if let Some(profile) = &app_state.active_profile {
            match CloudWatchLogsRepository::describe_log_groups(
                &app_state.aws_config,
                &profile.sdk_config,
                next_token,
            )
            .await
            {
                Ok(log_groups) => {
                    app_state.cloud_watch_state = CloudWatchState {
                        log_groups,
                        selected_log_group: None,
                    }
                }
                Err(_) => {
                    app_state.status_state.err_message =
                        TUI_CONFIG.messages.error_caller_identity.into();
                    app_state.cloud_watch_state = CloudWatchState::default()
                }
            };

            event!(Level::DEBUG, "{:?}", app_state);
        }
    }
}
