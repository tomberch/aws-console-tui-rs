use crossterm::event::KeyEvent;
use ratatui::{
    prelude::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Cell, Row, Table, TableState},
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

pub struct ToolbarComponent {
    table_state: TableState,
}

impl Component for ToolbarComponent {
    fn new(_action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        ToolbarComponent {
            table_state: TableState::default(),
        }
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::Status
    }

    fn handle_key_event(&mut self, _key: KeyEvent, _app_state: &AppState) -> anyhow::Result<()> {
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, app_state: &AppState) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(area);

        let topic_color = TUI_CONFIG.theme.toolbar_info_topic;
        let info_table = Table::new(vec![
            Row::new(vec![
                Cell::from("Profile:").style(Style::default().fg(topic_color)),
                Cell::from(app_state.toolbar_state.profile_name.as_str())
                    .style(Style::default().fg(Color::White)),
            ]),
            Row::new(vec![
                Cell::from("Account:").style(Style::default().fg(topic_color)),
                Cell::from(app_state.toolbar_state.account.as_str())
                    .style(Style::default().fg(Color::White)),
            ]),
            Row::new(vec![
                Cell::from("User:").style(Style::default().fg(topic_color)),
                Cell::from(app_state.toolbar_state.user.as_str())
                    .style(Style::default().fg(Color::White)),
            ]),
        ])
        .column_spacing(2)
        .widths(&[Constraint::Length(8), Constraint::Length(15)]);

        frame.render_stateful_widget(info_table, layout[0], &mut self.table_state);
    }
}
