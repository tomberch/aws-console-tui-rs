use std::io::Stdout;

use crossterm::event::KeyEvent;
use ratatui::{
    prelude::{CrosstermBackend, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::state::{
    actions::actions::Action,
    appstate::{AppState, ComponentType},
};

use super::Component;

struct Props {
    message: String,
    err_message: String,
}

impl From<&AppState> for Props {
    fn from(app_state: &AppState) -> Self {
        Props {
            message: app_state.status_state.message.clone(),
            err_message: app_state.status_state.err_message.clone(),
        }
    }
}
pub struct StatusComponent {
    action_tx: UnboundedSender<Action>,
    props: Props,
}

impl Component for StatusComponent {
    fn new(app_state: &AppState, action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        StatusComponent {
            action_tx: action_tx.clone(),
            props: Props::from(app_state),
        }
    }

    fn move_with_state(self, app_state: &AppState) -> Self
    where
        Self: Sized,
    {
        StatusComponent {
            props: Props::from(app_state),
            ..self
        }
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::Status
    }

    fn handle_key_event(&mut self, _key: KeyEvent) -> anyhow::Result<()> {
        Ok(())
    }
}

impl StatusComponent {
    pub fn render(&mut self, frame: &mut Frame<'_, CrosstermBackend<Stdout>>, area: Rect) {
        let status_text = vec![
            Line::styled(&self.props.message, Style::default().fg(Color::DarkGray)),
            Line::styled(&self.props.err_message, Style::default().fg(Color::Red)),
        ];

        frame.render_widget(
            Paragraph::new(status_text).block(
                Block::default()
                    .border_style(Style::new().fg(Color::White))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            ),
            area,
        );
    }
}
