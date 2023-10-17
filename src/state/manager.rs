use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio_util::sync::CancellationToken;

use crate::state::actions::profile_actions::ProfileActionHandler;

use super::{actions::actions::Action, state::AppState};

pub struct StateManager {
    state_tx: UnboundedSender<AppState>,
}

impl StateManager {
    pub fn new() -> (Self, UnboundedReceiver<AppState>) {
        let (state_tx, state_rx) = mpsc::unbounded_channel::<AppState>();
        (StateManager { state_tx }, state_rx)
    }

    pub async fn run(
        self,
        mut action_rx: UnboundedReceiver<Action>,
        cancellation_token: CancellationToken,
    ) -> anyhow::Result<()> {
        let mut app_state = AppState::default();

        // set the initial state once
        self.state_tx.send(app_state.clone())?;

        loop {
            tokio::select! {
                    _ = cancellation_token.cancelled() => {
                        break;
                      }

                Some(action) = action_rx.recv() => match action {
                    Action::ProfileAction { action } => {
                        ProfileActionHandler::handle(action, &mut app_state)
                    }
                },
            }
        }

        Ok(())
    }
}
