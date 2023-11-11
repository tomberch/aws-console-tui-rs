use crossterm::event::KeyEvent;
use ratatui::{prelude::Rect, Frame};
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
pub mod toolbar;

pub trait Component {
    fn new(action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized;

    fn component_type(&self) -> ComponentType;

    fn send_focus_action(&mut self, _component_type: ComponentType) -> Result<(), anyhow::Error> {
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent, app_state: &AppState) -> anyhow::Result<()>;

    fn render(&mut self, frame: &mut Frame, area: Rect, app_state: &AppState);
}
