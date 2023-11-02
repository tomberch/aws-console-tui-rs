use std::{cmp::min, io::Stdout};

use anyhow::Context;
use chrono::{DateTime, SecondsFormat, Utc};
use crossterm::event::{KeyCode, KeyEvent};

use ratatui::{
    prelude::{Alignment, CrosstermBackend, Rect},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;
use tui_tree_widget::TreeState;

use crate::{
    state::{
        actions::actions::{Action, CloudWatchLogsAction},
        appstate::{AppState, ComponentType},
        cloud_watch_logs_state::CloudWatchLogGroup,
    },
    ui::config::TUI_CONFIG,
};

use super::Component;

struct Props {
    has_focus: bool,
    log_groups: Vec<CloudWatchLogGroup>,
    selected_log_group: Option<CloudWatchLogGroup>,
}

impl From<&AppState> for Props {
    fn from(app_state: &AppState) -> Self {
        Props {
            has_focus: matches!(app_state.focus_component, ComponentType::AWSService),
            log_groups: app_state.cloud_watch_state.log_groups.clone(),
            selected_log_group: app_state.cloud_watch_state.selected_log_group.clone(),
        }
    }
}
pub struct CloudWatchLogsComponent {
    action_tx: UnboundedSender<Action>,
    first_time_render: bool,
    tree_state: TreeState<String>,
    props: Props,
}

impl Component for CloudWatchLogsComponent {
    fn new(app_state: &AppState, action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        CloudWatchLogsComponent {
            action_tx: action_tx.clone(),
            first_time_render: true,
            tree_state: TreeState::default(),
            props: Props::from(app_state),
        }
    }

    fn move_with_state(self, app_state: &AppState) -> Self
    where
        Self: Sized,
    {
        CloudWatchLogsComponent {
            props: Props::from(app_state),
            ..self
        }
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::AWSService
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> anyhow::Result<()> {
        match key.code {
            KeyCode::Char('u') => self.update(),
            _ => {}
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        if self.first_time_render {
            self.update();
            self.first_time_render = false;
        }

        if self.props.log_groups.is_empty() {
            frame.render_widget(
                Paragraph::new("\nNo Log Groups available").block(self.create_block()),
                area,
            );
        } else {
            let list_items = self
                .props
                .log_groups
                .iter()
                .map(|log_group| ListItem::new(self.create_list_item_text(log_group)))
                .collect::<Vec<ListItem>>();

            let list = List::new(list_items).block(self.create_block());
            frame.render_widget(list, area);
        }
    }
}

impl CloudWatchLogsComponent {
    fn create_list_item_text(&self, log_group: &CloudWatchLogGroup) -> Text {
        let name = match log_group.name.clone() {
            Some(name) => name,
            None => "unknown name".into(),
        };

        let retention_days = match log_group.retention_days {
            Some(retention_days) => {
                if retention_days == 1 {
                    "1 day".into()
                } else {
                    format!("{} days", retention_days)
                }
            }
            None => "indefinite".into(),
        };

        let date_created = match log_group.date_created {
            Some(date_created) => DateTime::from_timestamp(date_created / 1000, 0)
                .unwrap()
                .to_rfc3339_opts(SecondsFormat::Secs, true),
            None => "unknown creation date".into(),
        };

        let stored_bytes = match log_group.stored_bytes {
            Some(stored_bytes) => human_bytes::human_bytes(stored_bytes as f64),
            None => "unknown byte size".into(),
        };

        Text::from(format!(
            "{}  {: <8}  {: >10}  {}",
            date_created, retention_days, stored_bytes, name
        ))
    }

    fn update(&self) {
        let _ = self.action_tx.send(Action::CloudWatchLogs {
            action: CloudWatchLogsAction::GetLogGroups { token: None },
        });
    }

    fn create_block(&self) -> Block {
        Block::default()
            .title(format!(
                " CloudWatch Logs [{}] ",
                TUI_CONFIG.key_config.focus_aws_service.key_string
            ))
            .title_alignment(Alignment::Center)
            .border_style(if self.props.has_focus {
                TUI_CONFIG.focus_border
            } else {
                TUI_CONFIG.non_focus_border
            })
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
    }

    // fn first(&mut self) {
    //     self.tree_state.select_first();
    // }

    // fn last(&mut self) {
    //     self.tree_state.select_last(&self.items);
    // }

    // fn down(&mut self) {
    //     self.tree_state.key_down(&self.items);
    // }

    // fn up(&mut self) {
    //     self.tree_state.key_up(&self.items);
    // }

    // fn left(&mut self) {
    //     self.tree_state.key_left();
    // }

    // fn right(&mut self) {
    //     self.tree_state.key_right();
    // }

    // fn toggle(&mut self) {
    //     self.tree_state.toggle_selected();
    // }
}
