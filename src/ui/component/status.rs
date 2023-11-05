use crossterm::event::KeyEvent;
use ratatui::{
    prelude::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    text::Line,
    widgets::Paragraph,
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    state::{
        actions::actions::Action,
        appstate::{AppState, ComponentType},
    },
    ui::config::TUI_CONFIG,
};

use super::Component;

pub struct StatusComponent {}

impl Component for StatusComponent {
    fn new(_action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        StatusComponent {}
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::Status
    }

    fn handle_key_event(&mut self, _key: KeyEvent, _app_state: &AppState) -> anyhow::Result<()> {
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, app_state: &AppState) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1), Constraint::Length(1)])
            .split(area);

        let status_text = if app_state.status_state.err_message.is_empty() {
            Line::styled(
                &app_state.status_state.message,
                Style::default().fg(TUI_CONFIG.theme.status_message_text),
            )
        } else {
            Line::styled(
                &app_state.status_state.err_message,
                Style::default().fg(TUI_CONFIG.theme.error_message_text),
            )
        };

        frame.render_widget(
            Paragraph::new(status_text).alignment(Alignment::Center),
            layout[1],
        );
    }
}
