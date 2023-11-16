use crossterm::event::KeyEvent;

use ratatui::{
    prelude::{Alignment, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders, List, ListState},
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    state::{
        action_handlers::actions::{Action, ServiceAction},
        appstate::{AWSService, AppState, ComponentType},
    },
    ui::tui_config::TUI_CONFIG,
};

use super::{base::list_component::ListComponent, Component};

pub struct ServicesComponent<'a> {
    action_tx: UnboundedSender<Action>,
    services_list: ListComponent<'a>,
}

impl<'a> Component for ServicesComponent<'a> {
    fn new(action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        ServicesComponent {
            action_tx: action_tx.clone(),
            services_list: ListComponent::from([
                TUI_CONFIG.services.cloud_watch_logs,
                TUI_CONFIG.services.dynamodb,
                TUI_CONFIG.services.eks,
                TUI_CONFIG.services.s3_simple_storage_service,
                TUI_CONFIG.services.service_catalog,
            ]),
        }
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::Services
    }

    fn set_focus(&self) -> anyhow::Result<()> {
        self.action_tx.send(Action::SetBreadcrumbs {
            breadcrumbs: vec![TUI_CONFIG.breadcrumbs.services.into()],
        })?;

        self.action_tx.send(Action::SetMenu {
            menu_items: [
                vec![],
                vec![],
                vec![
                    TUI_CONFIG.menu.down.into(),
                    TUI_CONFIG.menu.up.into(),
                    TUI_CONFIG.menu.select.into(),
                ],
            ],
        })?;

        self.send_focus_action(&self.action_tx)
    }

    fn handle_key_event(&mut self, key: KeyEvent, app_state: &AppState) -> anyhow::Result<()> {
        if !self.has_focus(app_state) {
            if TUI_CONFIG.key_config.focus_services.key_code == key.code
                && TUI_CONFIG.key_config.focus_services.key_modifier == key.modifiers
            {
                self.set_focus()?;
            }
        } else if app_state.active_profile.is_some() {
            match key.code {
                val if TUI_CONFIG.list_config.selection_up == val => self.services_list.move_up(),
                val if TUI_CONFIG.list_config.selection_down == val => {
                    self.services_list.move_down()
                }

                val if TUI_CONFIG.list_config.do_selection == val => {
                    self.set_active_service()?;
                }
                _ => {}
            };
        }

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, app_state: &AppState) {
        let selected_index = if app_state.active_profile.is_some() {
            Some(self.services_list.get_selected_index())
        } else {
            None
        };
        let mut list_state = ListState::default().with_selected(selected_index);

        frame.render_stateful_widget(
            List::new(self.services_list.create_tui_list())
                .block(
                    Block::default()
                        .title(format!(
                            " Services [{}] ",
                            TUI_CONFIG.key_config.focus_services.key_string
                        ))
                        .title_alignment(Alignment::Center)
                        .border_style(Style::new().fg(if self.has_focus(app_state) {
                            TUI_CONFIG.theme.border_highlight
                        } else {
                            TUI_CONFIG.theme.border
                        }))
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
    fn has_focus(&self, app_state: &AppState) -> bool {
        app_state.focus_component == self.component_type()
    }

    fn set_active_service(&mut self) -> anyhow::Result<()> {
        if let Some(service_name) = self.services_list.set_active_item() {
            self.action_tx.send(Action::Service {
                action: ServiceAction::SelectService {
                    service: self.get_variant_for_selected_service(service_name.as_ref()),
                },
            })?
        }

        Ok(())
    }

    fn get_variant_for_selected_service(&self, service_name: &str) -> AWSService {
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
