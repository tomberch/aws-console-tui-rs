use std::io::Stdout;

use crossterm::event::KeyEvent;
use ratatui::{
    prelude::{CrosstermBackend, Rect},
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::state::{
    actions::actions::Action,
    appstate::{AppState, ComponentType},
};

pub mod cloud_watch_logs;
pub mod profiles;
pub mod regions;
pub mod services;
pub mod status;

pub trait Component {
    fn new(app_state: &AppState, action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized;

    fn move_with_state(self, state: &AppState) -> Self
    where
        Self: Sized;

    fn component_type(&self) -> ComponentType;

    fn handle_key_event(&mut self, key: KeyEvent) -> anyhow::Result<()>;

    fn render(&mut self, frame: &mut Frame<'_, CrosstermBackend<Stdout>>, area: Rect);
}
