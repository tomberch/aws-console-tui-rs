use std::time::Duration;

use crate::ui::component::Component;
use anyhow::Context;
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures::{FutureExt, StreamExt};
use ratatui::prelude::Rect;
use tokio::sync::mpsc::{self, UnboundedReceiver};
use tokio_util::sync::CancellationToken;

use crate::state::{actions::actions::Action, appstate::AppState};

use super::{config::TUI_CONFIG, pages::home::HomePage, term::Term};

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

        let mut home_page = {
            let app_state = state_rx.recv().await.unwrap();

            HomePage::new(&app_state, self.action_tx.clone())
        };

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

                                    if self.should_quit(&key) {
                                        cancellation_token.cancel();
                                    } else {

                                    home_page.handle_key_event(key)?;}
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

                            }
                        }
                    },
                    Some(Err(_)) => {

                    },
                    None => todo!(),
                },
                Some(state) = state_rx.recv() => {
                    home_page = home_page.move_with_state(&state);
                },

            }

            terminal
                .draw(|frame| home_page.render(frame, frame.size()))
                .context("could not render to the terminal")?;
        }

        let _ = Term::stop();

        Ok(())
    }

    fn should_quit(&self, key: &KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') => true,
            KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => true,
            _ => false,
        }
    }
}
