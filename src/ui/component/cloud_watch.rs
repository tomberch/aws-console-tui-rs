use std::{cmp::min, io::Stdout};

use anyhow::Context;
use crossterm::event::KeyEvent;

use ratatui::{
    prelude::{Alignment, CrosstermBackend, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    state::{
        actions::actions::{Action, ProfileAction},
        appstate::{AppState, ComponentType},
    },
    ui::config::TUI_CONFIG,
};

use super::Component;

struct Props {
    items: Vec<String>,
    has_focus: bool,
}

impl From<&AppState> for Props {
    fn from(app_state: &AppState) -> Self {
        Props {
            items: app_state.profile_state.profile_names.clone(),
            has_focus: matches!(app_state.focus_component, ComponentType::Profiles),
        }
    }
}
pub struct CloudWatchComponent {
    action_tx: UnboundedSender<Action>,
    props: Props,
}

impl Component for CloudWatchComponent {
    fn new(app_state: &AppState, action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        CloudWatchComponent {
            action_tx: action_tx.clone(),
            props: Props::from(app_state),
        }
    }

    fn move_with_state(self, app_state: &AppState) -> Self
    where
        Self: Sized,
    {
        CloudWatchComponent {
            props: Props::from(app_state),
            ..self
        }
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::Profiles
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> anyhow::Result<()> {
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame<'_, CrosstermBackend<Stdout>>, area: Rect) {
        frame.render_widget(
            Paragraph::new("This is cloud watch").block(
                Block::default()
                    .border_style(Style::new().fg(Color::White))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            ),
            area,
        );
    }
}
