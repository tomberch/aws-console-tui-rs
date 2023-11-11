use std::ops::Add;

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
        let table_rows = vec![
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
            Row::new(vec![
                Cell::from("Perf:").style(Style::default().fg(topic_color)),
                Cell::from(app_state.measure_state.render_duration.as_str())
                    .style(Style::default().fg(Color::White)),
            ]),
        ];

        let info_table = Table::new(table_rows)
            .column_spacing(2)
            .widths(&[Constraint::Length(8), Constraint::Length(15)]);

        frame.render_stateful_widget(info_table, area, &mut self.info_table_state);
    }

    fn render_menu_table(&mut self, frame: &mut Frame, area: Rect, app_state: &AppState) {
        let menu_color = TUI_CONFIG.theme.command;
        let mut table_rows: Vec<Row> = vec![];
        let mut max_len_command: [usize; 3] = [0, 0, 0];
        let mut max_len_title: [usize; 3] = [0, 0, 0];

        for index in 0..6 {
            let menu_item1 = self.get_menu_item_text(index, &app_state.toolbar_state.menu);
            let menu_item2 = self.get_menu_item_text(index + 6, &app_state.toolbar_state.menu);
            let menu_item3 = self.get_menu_item_text(index + 12, &app_state.toolbar_state.menu);
            // max_len_command[0] = std::cmp::max(command1.len(), max_len_command[0]);
            // max_len_title[0] = std::cmp::max(title1.len(), max_len_title[0]);
            // max_len_command[1] = std::cmp::max(command2.len(), max_len_command[1]);
            // max_len_title[1] = std::cmp::max(title2.len(), max_len_title[1]);
            // max_len_command[2] = std::cmp::max(command3.len(), max_len_command[2]);
            // max_len_title[2] = std::cmp::max(title3.len(), max_len_title[2]);
            table_rows.push(Row::new(vec![
                Cell::from(menu_item1.command).style(Style::default().fg(menu_color)),
                Cell::from(menu_item1.title).style(Style::default().fg(Color::White)),
                Cell::from(menu_item2.command).style(Style::default().fg(menu_color)),
                Cell::from(menu_item2.title).style(Style::default().fg(Color::White)),
                Cell::from(menu_item3.command).style(Style::default().fg(menu_color)),
                Cell::from(menu_item3.title).style(Style::default().fg(Color::White)),
            ]))
        }

        // let constraints = [
        //     Constraint::Length(max_len_command[0] as u16),
        //     Constraint::Length(max_len_title[0] as u16 + 2),
        //     Constraint::Length(max_len_command[1] as u16),
        //     Constraint::Length(max_len_title[1] as u16 + 2),
        //     Constraint::Length(max_len_command[2] as u16),
        //     Constraint::Length(max_len_title[2] as u16 + 2),
        // ];
        let menu_table = Table::new(table_rows).column_spacing(1).widths(&[
            Constraint::Min(0),
            Constraint::Min(0),
            Constraint::Min(0),
            Constraint::Min(0),
            Constraint::Min(0),
            Constraint::Min(0),
        ]);

        frame.render_stateful_widget(menu_table, area, &mut self.menu_table_state);
    }

    fn get_menu_item_text<'a>(
        &self,
        index: usize,
        menu_items: &'a Vec<MenuItem>,
    ) -> Option<&'a MenuItem> {
        if index < menu_items.len() {
            Some(&menu_items[index])
        } else {
            None
        }
    }
}
