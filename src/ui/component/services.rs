use std::{cmp::min, io::Stdout};

use anyhow::Context;
use crossterm::event::KeyEvent;

use ratatui::{
    prelude::{Alignment, CrosstermBackend, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    state::{
        actions::actions::{Action, ServiceAction},
        appstate::{AWSService, AppState, ComponentType},
    },
    ui::config::TUI_CONFIG,
};

use super::Component;

struct Props {
    has_focus: bool,
    is_active_profile_set: bool,
}

impl From<&AppState> for Props {
    fn from(app_state: &AppState) -> Self {
        Props {
            has_focus: matches!(app_state.focus_component, ComponentType::Services),
            is_active_profile_set: app_state.active_profile.is_some(),
        }
    }
}
pub struct ServicesComponent<'a> {
    action_tx: UnboundedSender<Action>,
    props: Props,

    service_names: [&'a str; 5],
    selected_index: u16,
    active_service_index: Option<u16>,
}

impl<'a> Component for ServicesComponent<'a> {
    fn new(app_state: &AppState, action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        ServicesComponent {
            action_tx: action_tx.clone(),
            props: Props::from(app_state),
            selected_index: 0,
            service_names: [
                TUI_CONFIG.services.cloud_watch_logs,
                TUI_CONFIG.services.dynamodb,
                TUI_CONFIG.services.eks,
                TUI_CONFIG.services.s3_simple_storage_service,
                TUI_CONFIG.services.service_catalog,
            ],
            active_service_index: None,
        }
    }

    fn move_with_state(self, app_state: &AppState) -> Self
    where
        Self: Sized,
    {
        ServicesComponent {
            props: Props::from(app_state),
            ..self
        }
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::Services
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> anyhow::Result<()> {
        if !self.props.has_focus {
            if TUI_CONFIG.key_config.focus_services.key_code == key.code
                && TUI_CONFIG.key_config.focus_services.key_modifier == key.modifiers
            {
                self.action_tx
                    .send(Action::SetFocus {
                        component_type: self.component_type(),
                    })
                    .context("Could not send action for focus update")?;
            }
        } else if self.props.is_active_profile_set {
            match key.code {
                val if TUI_CONFIG.list_config.selection_up == val => {
                    self.selected_index = if self.selected_index > 0 {
                        self.selected_index - 1
                    } else {
                        0
                    }
                }
                val if TUI_CONFIG.list_config.selection_down == val => {
                    self.selected_index = min(self.selected_index + 1, self.get_list_len() - 1);
                }
                val if TUI_CONFIG.list_config.do_selection == val => {
                    self.set_active_service(self.selected_index)?;
                }
                _ => {}
            };
        }

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let list_items = self
            .service_names
            .into_iter()
            .enumerate()
            .map(|(index, element)| ListItem::new(self.get_list_item_text(index, element.into())))
            .collect::<Vec<ListItem>>();

        let selected_index = if self.props.is_active_profile_set {
            Some(self.selected_index.into())
        } else {
            None
        };
        let mut list_state = ListState::default().with_selected(selected_index);

        frame.render_stateful_widget(
            List::new(list_items)
                .block(
                    Block::default()
                        .title(format!(
                            " Services [{}] ",
                            TUI_CONFIG.key_config.focus_services.key_string
                        ))
                        .title_alignment(Alignment::Center)
                        .border_style(if self.props.has_focus {
                            TUI_CONFIG.focus_border
                        } else {
                            TUI_CONFIG.non_focus_border
                        })
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                .highlight_style(TUI_CONFIG.list_config.selected_style)
                .highlight_symbol(TUI_CONFIG.list_config.selected_symbol),
            area,
            &mut list_state,
        );
    }
}

impl<'a> ServicesComponent<'a> {
    fn get_list_len(&self) -> u16 {
        self.service_names.len().try_into().unwrap()
    }

    fn get_list_item_text(&self, index: usize, list_item_string: String) -> Text {
        let is_active_service_index = match self.active_service_index {
            None => false,
            Some(active_index) => usize::try_from(active_index)
                .map(|active_index| active_index == index)
                .unwrap_or(false),
        };

        if is_active_service_index {
            Text::styled(
                format!("**{}", list_item_string),
                Style::default().fg(Color::Yellow),
            )
        } else {
            Text::from(list_item_string)
        }
    }

    fn set_active_service(&mut self, index: u16) -> anyhow::Result<()> {
        if let Some(active_index) = self.active_service_index {
            if active_index == index {
                return Ok(());
            }
        }

        self.active_service_index = Some(index);

        self.action_tx.send(Action::Service {
            action: ServiceAction::SelectService {
                service: self.get_variant_for_selected_service(index),
            },
        })?;

        Ok(())
    }

    fn get_variant_for_selected_service(&self, index: u16) -> AWSService {
        let service_name = self.service_names[usize::from(index)];
        match service_name {
            val if TUI_CONFIG.services.cloud_watch_logs == val => AWSService::CloudWatchLogs,
            val if TUI_CONFIG.services.dynamodb == val => AWSService::DynamoDB,
            val if TUI_CONFIG.services.eks == val => AWSService::Eks,
            val if TUI_CONFIG.services.s3_simple_storage_service == val => AWSService::S3,
            val if TUI_CONFIG.services.service_catalog == val => AWSService::ServiceCatalog,
            _ => AWSService::None,
        }
    }
}
