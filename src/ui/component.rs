use anyhow::Context;
use crossterm::event::KeyEvent;
use ratatui::{prelude::Rect, Frame};
use tokio::sync::mpsc::UnboundedSender;

use crate::state::{
    action_handlers::actions::Action,
    appstate::{AppState, ComponentType},
};

pub mod base;
pub mod cloud_watch_logs;
pub mod profiles;
pub mod regions;
pub mod services;
pub mod status;
pub mod toolbar;

pub trait Component {
    fn new(action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized;

    fn component_type(&self) -> ComponentType;

    fn set_focus(&self) -> anyhow::Result<()> {
        Ok(())
    }

    fn send_focus_action(&self, action_tx: &UnboundedSender<Action>) -> anyhow::Result<()> {
        action_tx
            .send(Action::SetFocus {
                component_type: self.component_type(),
            })
            .context("Could not send action for focus update")
    }

    fn handle_key_event(&mut self, key: KeyEvent, app_state: &AppState) -> anyhow::Result<()>;

    fn render(&mut self, frame: &mut Frame, area: Rect, app_state: &AppState);
}
