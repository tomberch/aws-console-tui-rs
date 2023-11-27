use std::{sync::Arc, time::Duration};

use crate::ui::component::Component;
use anyhow::Context;
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures::{FutureExt, StreamExt};
use ratatui::prelude::Rect;

use tokio::{
    sync::{
        mpsc::{self, UnboundedReceiver},
        RwLock,
    },
    time::Instant,
};
use tokio_util::sync::CancellationToken;

use crate::state::{action_handlers::actions::Action, appstate::AppState};

use super::{pages::home::HomePage, term::Term, tui_config::TUI_CONFIG};

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
        mut state_rx: UnboundedReceiver<Arc<RwLock<AppState>>>,
        cancellation_token: CancellationToken,
    ) -> anyhow::Result<()> {
        let mut terminal = Term::start().context("Cannot start the terminal")?;
        self.update_frame_size(&terminal.get_frame().size())?;

        let mut ticker = tokio::time::interval(Duration::from_millis(TUI_CONFIG.tick_rate_in_ms));
        let mut performance_measure_ticker = tokio::time::interval(Duration::from_secs(
            TUI_CONFIG.performance_measure_rate_in_sec,
        ));
        let mut event_reader = EventStream::new();

        let mut app_state = state_rx.recv().await.unwrap().read().await.to_owned();
        let mut home_page = HomePage::new(self.action_tx.clone());
        home_page.set_initial_focus()?;

        let mut render_duration = Duration::default();

        loop {
            let crossterm_event = event_reader.next().fuse();

            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    break;
                }

                _ = ticker.tick() => (),

                _ = performance_measure_ticker.tick() => {
                     self.action_tx.send(Action::RenderDuration { duration: render_duration })?;
                }


                maybe_event = crossterm_event => match maybe_event {
                    Some(Ok(event)) => {
                        match event {

                            Event::Key(key) => {
                                if key.kind == KeyEventKind::Press {

                                    if self.should_quit(&key) {
                                        cancellation_token.cancel();
                                    } else {

                                    home_page.handle_key_event(key, &app_state)?;}
                                }
                            },
                            Event::Mouse(_mouse) => {},
                            Event::Resize(width, height) => {
                                terminal
                                .resize(Rect::new(0, 0, width, height))
                                .context("Tried to resize terminal window")?;
                             self.update_frame_size(&terminal.get_frame().size())?;
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
                Some(state) = state_rx.recv() =>
                    app_state = state.read().await.to_owned(),

            }

            let start = Instant::now();
            terminal
                .draw(|frame| home_page.render(frame, frame.size(), &app_state))
                .context("could not render to the terminal")?;
            render_duration = start.elapsed();
        }

        let _ = Term::stop();

        Ok(())
    }

    fn should_quit(&self, key: &KeyEvent) -> bool {
        matches!(key.code, KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL)
    }

    fn update_frame_size(&self, &area: &Rect) -> anyhow::Result<()> {
        self.action_tx.send(Action::SetTerminalArea { area })?;
        Ok(())
    }
}
