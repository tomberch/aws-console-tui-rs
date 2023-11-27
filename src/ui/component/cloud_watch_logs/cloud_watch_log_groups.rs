use chrono::{DateTime, SecondsFormat};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    state::{
        action_handlers::actions::{Action, CloudWatchLogsAction},
        appstate::{AppState, ComponentType},
        cloud_watch_logs_state::CloudWatchLogGroup,
    },
    ui::{
        component::{base::list_component::ListComponent, Component},
        tui_config::TUI_CONFIG,
    },
};
use ratatui::{
    prelude::{Alignment, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders, Clear, List, ListState, Paragraph},
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;
use tui_textarea::TextArea;

pub struct CloudWatchLogGroupComponent<'a> {
    action_tx: UnboundedSender<Action>,
    log_group_list: ListComponent<'a>,
    first_time_render: bool,
    filter_textarea: TextArea<'a>,
    is_editing_filter: bool,
}

impl<'a> Component for CloudWatchLogGroupComponent<'a> {
    fn new(action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        let mut filter_text = TextArea::default();
        filter_text.set_cursor_line_style(Style::default());
        filter_text.set_placeholder_text("Enter filter expression");
        filter_text.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title("Filter")
                .title_alignment(Alignment::Center),
        );

        CloudWatchLogGroupComponent {
            action_tx: action_tx.clone(),
            log_group_list: ListComponent::new(),
            filter_textarea: filter_text,
            first_time_render: true,
            is_editing_filter: false,
        }
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::AWSService
    }

    fn set_focus(&self) -> anyhow::Result<()> {
        self.set_breadcrumbs()?;

        self.action_tx.send(Action::SetMenu {
            menu_items: [
                vec![],
                vec![],
                vec![
                    TUI_CONFIG.menu.filter.into(),
                    TUI_CONFIG.menu.info.into(),
                    TUI_CONFIG.menu.up.into(),
                    TUI_CONFIG.menu.down.into(),
                    TUI_CONFIG.menu.select.into(),
                ],
            ],
        })?;

        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent, app_state: &AppState) -> anyhow::Result<()> {
        if self.is_editing_filter {
            match key.code {
                KeyCode::Enter => self.is_editing_filter = false,
                KeyCode::Char('m') if key.modifiers == KeyModifiers::CONTROL => {}
                _ => {
                    self.filter_textarea.input(key);
                }
            }
        } else {
            match key.code {
                KeyCode::Char('u') => self.update()?,
                KeyCode::Char('f') => self.is_editing_filter = true,
                val if TUI_CONFIG.list_config.selection_up == val => self.log_group_list.move_up(),
                val if TUI_CONFIG.list_config.selection_down == val => {
                    self.log_group_list.move_down();
                }
                val if TUI_CONFIG.list_config.do_selection == val => {
                    self.set_active_log_group(app_state)?;
                }

                _ => {}
            }
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, app_state: &AppState) {
        if self.first_time_render {
            let _ = self.update();
            self.first_time_render = false;
        }

        if app_state.cloud_watch_state.log_groups.is_empty() {
            frame.render_widget(
                Paragraph::new("\nNo Log Groups available").block(self.create_block(app_state)),
                area,
            );
        } else {
            if app_state.cloud_watch_state.log_groups.len() != self.log_group_list.get_list_len() {
                self.log_group_list.create_list_items(
                    app_state
                        .cloud_watch_state
                        .log_groups
                        .iter()
                        .map(|log_group| self.create_list_item(log_group))
                        .collect::<Vec<String>>(),
                );
            }

            let mut list_state =
                ListState::default().with_selected(Some(self.log_group_list.get_selected_index()));
            let list = List::new(self.log_group_list.create_tui_list())
                .highlight_style(TUI_CONFIG.list_config.selected_style)
                .highlight_symbol(TUI_CONFIG.list_config.selected_symbol)
                .block(self.create_block(app_state));
            frame.render_stateful_widget(list, area, &mut list_state);

            if self.is_editing_filter {
                let text_area = self.centered_rect(40, 10, app_state.area);
                frame.render_widget(Clear, text_area);
                frame.render_widget(self.filter_textarea.widget(), text_area);
            }
        }
    }
}

impl<'a> CloudWatchLogGroupComponent<'a> {
    fn has_focus(&self, app_state: &AppState) -> bool {
        app_state.focus_component == self.component_type()
    }

    fn create_list_item(&self, log_group: &CloudWatchLogGroup) -> String {
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

        format!("{}  {}", date_created, name)
    }

    fn set_active_log_group(&mut self, _app_state: &AppState) -> anyhow::Result<()> {
        Ok(())
    }

    fn update(&self) -> anyhow::Result<()> {
        self.action_tx.send(Action::CloudWatchLogs {
            action: CloudWatchLogsAction::GetLogGroups { token: None },
        })?;

        self.set_breadcrumbs()
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

    fn set_breadcrumbs(&self) -> Result<(), anyhow::Error> {
        self.action_tx.send(Action::SetBreadcrumbs {
            breadcrumbs: if self.filter_textarea.is_empty() {
                vec![TUI_CONFIG.breadcrumbs.cloud_watch_logs.into()]
            } else {
                vec![
                    TUI_CONFIG.breadcrumbs.cloud_watch_logs.into(),
                    TUI_CONFIG.breadcrumbs.cloud_watch_logs_filtered.into(),
                ]
            },
        })?;
        Ok(())
    }
}
