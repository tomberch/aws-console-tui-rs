use std::time::Duration;

use anyhow::Context;
use crossterm::event::{Event, EventStream, KeyEventKind};
use futures::{FutureExt, StreamExt};
use ratatui::prelude::Rect;
use tokio::sync::mpsc::{self, UnboundedReceiver};
use tokio_util::sync::CancellationToken;

use crate::state::{actions::actions::Action, state::AppState};

use super::{config::TUI_CONFIG, term::Term};

pub struct UIManager {
    action_tx: mpsc::UnboundedSender<Action>,
}

impl UIManager {
    pub fn new() -> (Self, UnboundedReceiver<Action>) {
        let (action_tx, action_rx) = mpsc::unbounded_channel();
        (Self { action_tx }, action_rx)
    }

    pub async fn run(
        self,
        mut state_rx: UnboundedReceiver<AppState>,
        cancellation_token: CancellationToken,
    ) -> anyhow::Result<()> {
        let mut terminal = Term::start().context("Cannot start the terminal")?;
        let mut ticker = tokio::time::interval(Duration::from_millis(TUI_CONFIG.tick_rate_in_ms));
        let mut event_reader = EventStream::new();

        loop {
            let crossterm_event = event_reader.next().fuse();

            tokio::select! {
                            _ = cancellation_token.cancelled() => {
                                break;
                              }

                            _ = ticker.tick() => (),

                            maybe_event = crossterm_event => match maybe_event {
                Some(Ok(event)) => {
                    match event {


                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {

                    }
                  },
                  Event::Mouse(_mouse) => {},
                  Event::Resize(width, height) => {
                    terminal
                    .resize(Rect::new(0, 0, width, height))
                    .context("Tried to resize terminal window")?;
                  },
                  Event::FocusLost => {},
                  Event::FocusGained => {},
                  Event::Paste(_s) => {

                  },}},
                Some(Err(_)) => {

                  }
                None => todo!(),
            },
            Some(state) = state_rx.recv() => {
                app_router = app_router.move_with_state(&state);
            },
                        }
        }

        Term::stop();

        Ok(())
    }
}
