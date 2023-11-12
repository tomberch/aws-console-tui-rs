use std::cmp::min;

use chrono::{DateTime, SecondsFormat};
use crossterm::event::{KeyCode, KeyEvent};

use ratatui::{
    prelude::{Alignment, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    state::{
        actions::actions::{Action, CloudWatchLogsAction},
        appstate::{AppState, ComponentType},
        cloud_watch_logs_state::CloudWatchLogGroup,
    },
    ui::config::TUI_CONFIG,
};

use super::Component;

pub struct CloudWatchLogsComponent {
    action_tx: UnboundedSender<Action>,
    selected_index: u16,
    active_index: Option<u16>,
    first_time_render: bool,
}

impl Component for CloudWatchLogsComponent {
    fn new(action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        CloudWatchLogsComponent {
            action_tx: action_tx.clone(),
            selected_index: 0,
            active_index: None,
            first_time_render: true,
        }
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::AWSService
    }

    fn set_focus(&self) -> anyhow::Result<()> {
        self.action_tx.send(Action::SetBreadcrumbs {
            breadcrumbs: vec![TUI_CONFIG.breadcrumbs.cloud_watch_logs.into()],
        })?;

        self.action_tx.send(Action::SetMenu {
            menu_items: [
                vec![],
                vec![TUI_CONFIG.menu.details.into()],
                self.get_default_menu(),
            ],
        })?;

        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent, app_state: &AppState) -> anyhow::Result<()> {
        match key.code {
            KeyCode::Char('u') => self.update(),
            val if TUI_CONFIG.list_config.selection_up == val => {
                self.selected_index = if self.selected_index > 0 {
                    self.selected_index - 1
                } else {
                    0
                }
            }
            val if TUI_CONFIG.list_config.selection_down == val => {
                self.selected_index =
                    min(self.selected_index + 1, self.get_list_len(app_state) - 1);
            }
            val if TUI_CONFIG.list_config.do_selection == val => {
                self.set_active_log_group(self.selected_index, app_state)?;
            }

            _ => {}
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, app_state: &AppState) {
        if self.first_time_render {
            self.update();
            self.first_time_render = false;
        }

        if app_state.cloud_watch_state.log_groups.is_empty() {
            frame.render_widget(
                Paragraph::new("\nNo Log Groups available").block(self.create_block(app_state)),
                area,
            );
        } else {
            let list_items = app_state
                .cloud_watch_state
                .log_groups
                .iter()
                .enumerate()
                .map(|(index, log_group)| {
                    ListItem::new(self.create_list_item_text(index, log_group))
                })
                .collect::<Vec<ListItem>>();

            let mut list_state =
                ListState::default().with_selected(Some(self.selected_index.into()));
            let list = List::new(list_items)
                .block(self.create_block(app_state))
                .highlight_style(TUI_CONFIG.list_config.selected_style)
                .highlight_symbol(TUI_CONFIG.list_config.selected_symbol);
            frame.render_stateful_widget(list, area, &mut list_state);
        }
    }
}

impl CloudWatchLogsComponent {
    fn has_focus(&self, app_state: &AppState) -> bool {
        app_state.focus_component == self.component_type()
    }

    fn get_list_len(&self, app_state: &AppState) -> u16 {
        app_state
            .cloud_watch_state
            .log_groups
            .len()
            .try_into()
            .unwrap()
    }

    fn create_list_item_text(&self, index: usize, log_group: &CloudWatchLogGroup) -> Text {
        let is_active_profile_index = match self.active_index {
            None => false,
            Some(active_index) => usize::try_from(active_index)
                .map(|active_index| active_index == index)
                .unwrap_or(false),
        };

        let name = match log_group.name.clone() {
            Some(name) => name,
            None => "unknown name".into(),
        };

        let date_created = match log_group.date_created {
            Some(date_created) => DateTime::from_timestamp(date_created / 1000, 0)
                .unwrap()
                .to_rfc3339_opts(SecondsFormat::Secs, true),
            None => "unknown creation date".into(),
        };

        let list_item_string = format!("{}  {}", date_created, name);
        if is_active_profile_index {
            Text::styled(
                format!("**{}", list_item_string),
                Style::default().fg(Color::Yellow),
            )
        } else {
            Text::from(list_item_string)
        }
    }

    fn set_active_log_group(&mut self, index: u16, app_state: &AppState) -> anyhow::Result<()> {
        Ok(())
    }

    fn update(&self) {
        let _ = self.action_tx.send(Action::CloudWatchLogs {
            action: CloudWatchLogsAction::GetLogGroups { token: None },
        });
    }

    fn create_block(&self, app_state: &AppState) -> Block {
        Block::default()
            .title(format!(
                " CloudWatch Logs [{}] ",
                TUI_CONFIG.key_config.focus_aws_service.key_string
            ))
            .title_alignment(Alignment::Center)
            .border_style(Style::new().fg(if self.has_focus(app_state) {
                TUI_CONFIG.theme.border_highlight
            } else {
                TUI_CONFIG.theme.border
            }))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
    }
}
