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
        appstate::{AppState, ComponentType, MenuItem},
    },
    ui::config::TUI_CONFIG,
};

use super::Component;

pub struct ToolbarComponent {
    menu_table_state: TableState,
    info_table_state: TableState,
}

impl Component for ToolbarComponent {
    fn new(_action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        ToolbarComponent {
            menu_table_state: TableState::default(),
            info_table_state: TableState::default(),
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
            .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(area);

        self.render_info_table(frame, layout[0], app_state);
        self.render_menu_table(frame, layout[1], app_state);
    }
}

impl ToolbarComponent {
    fn render_info_table(&mut self, frame: &mut Frame, area: Rect, app_state: &AppState) {
        let topic_color = TUI_CONFIG.theme.toolbar_info_topic;
        let mut table_rows = vec![
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
            Row::new(vec![
                Cell::from("CPU:").style(Style::default().fg(topic_color)),
                Cell::from(app_state.toolbar_state.cpu_usage.as_str())
                    .style(Style::default().fg(Color::White)),
            ]),
            Row::new(vec![
                Cell::from("Memory:").style(Style::default().fg(topic_color)),
                Cell::from(app_state.toolbar_state.memory_usage.as_str())
                    .style(Style::default().fg(Color::White)),
            ]),
        ];

        if app_state.measure_state.is_active {
            table_rows.push(Row::new(vec![
                Cell::from("Perf:").style(Style::default().fg(topic_color)),
                Cell::from(format!(
                    "{}/{}",
                    app_state.measure_state.render_duration.as_str(),
                    app_state.measure_state.action_duration.as_str()
                ))
                .style(Style::default().fg(Color::White)),
            ]));
        }

        let info_table = Table::new(table_rows)
            .column_spacing(2)
            .widths(&[Constraint::Length(8), Constraint::Length(24)]);

        frame.render_stateful_widget(info_table, area, &mut self.info_table_state);
    }

    fn render_menu_table(&mut self, frame: &mut Frame, area: Rect, app_state: &AppState) {
        let mut table_rows: Vec<Row> = vec![];
        let mut max_len_command: [usize; 3] = [0, 0, 0];
        let mut max_len_title: [usize; 3] = [0, 0, 0];
        let empty_menu_item = MenuItem {
            command: "".into(),
            title: "".into(),
            color_index: 0,
        };

        for index in 0..6 {
            let menu_item1 = app_state.toolbar_state.menu_items[0]
                .get(index)
                .unwrap_or(&empty_menu_item);
            let menu_item2 = app_state.toolbar_state.menu_items[1]
                .get(index)
                .unwrap_or(&empty_menu_item);
            let menu_item3 = app_state.toolbar_state.menu_items[2]
                .get(index)
                .unwrap_or(&empty_menu_item);

            max_len_command[0] = std::cmp::max(menu_item1.command.len(), max_len_command[0]);
            max_len_title[0] = std::cmp::max(menu_item1.title.len(), max_len_title[0]);
            max_len_command[1] = std::cmp::max(menu_item2.command.len(), max_len_command[1]);
            max_len_title[1] = std::cmp::max(menu_item2.title.len(), max_len_title[1]);
            max_len_command[2] = std::cmp::max(menu_item3.command.len(), max_len_command[2]);
            max_len_title[2] = std::cmp::max(menu_item3.title.len(), max_len_title[2]);

            table_rows.push(Row::new(vec![
                Cell::from(menu_item1.command.as_str()).style(
                    Style::default().fg(TUI_CONFIG.theme.command_colors[menu_item1.color_index]),
                ),
                Cell::from(menu_item1.title.as_str()).style(Style::default().fg(Color::White)),
                Cell::from(menu_item2.command.as_str()).style(
                    Style::default().fg(TUI_CONFIG.theme.command_colors[menu_item2.color_index]),
                ),
                Cell::from(menu_item2.title.as_str()).style(Style::default().fg(Color::White)),
                Cell::from(menu_item3.command.as_str()).style(
                    Style::default().fg(TUI_CONFIG.theme.command_colors[menu_item3.color_index]),
                ),
                Cell::from(menu_item3.title.as_str()).style(Style::default().fg(Color::White)),
            ]));
        }

        let constraints = [
            Constraint::Length(max_len_command[0] as u16),
            Constraint::Length(max_len_title[0] as u16 + 3),
            Constraint::Length(max_len_command[1] as u16),
            Constraint::Length(max_len_title[1] as u16 + 3),
            Constraint::Length(max_len_command[2] as u16),
            Constraint::Length(max_len_title[2] as u16),
        ];

        let menu_table = Table::new(table_rows)
            .column_spacing(2)
            .widths(&constraints);

        frame.render_stateful_widget(menu_table, area, &mut self.menu_table_state);
    }
}
