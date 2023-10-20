use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio_util::sync::CancellationToken;

use crate::{config::config::AppConfig, state::actions::profile_actions::ProfileActionHandler};

use super::{actions::actions::Action, appstate::AppState};

pub struct StateManager {
    app_config: AppConfig,
    state_tx: UnboundedSender<AppState>,
}

impl StateManager {
    pub fn new(app_config: AppConfig) -> (Self, UnboundedReceiver<AppState>) {
        let (state_tx, state_rx) = mpsc::unbounded_channel::<AppState>();
        (
            StateManager {
                app_config,
                state_tx,
            },
            state_rx,
        )
    }

    pub async fn run(
        self,
        mut action_rx: UnboundedReceiver<Action>,
        cancellation_token: CancellationToken,
    ) -> anyhow::Result<()> {
        let mut app_state = AppState::new(&self.app_config);

        // set the initial state once
        self.state_tx.send(app_state.clone())?;

        loop {
            tokio::select! {
                        _ = cancellation_token.cancelled() => {
                            break;
                          }

                    Some(action) = action_rx.recv() => match action {
                        Action::SetFocus { component_type } => app_state.focus_component = component_type,
                        Action::ProfileAction{ action }=>{ProfileActionHandler::handle(self.state_tx.clone(), action, &mut app_state).await},

                }
            }

            self.state_tx.send(app_state.clone())?;
        }

        Ok(())
    }
}