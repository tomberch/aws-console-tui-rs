use anyhow::Context;
use crossterm::event::KeyEvent;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use crate::state::actions::actions::Action;
use crate::state::appstate::{AWSService, AppState, ComponentType};

use crate::ui::component::cloud_watch_logs::CloudWatchLogsComponent;
use crate::ui::component::Component;
use crate::ui::config::TUI_CONFIG;

pub struct AWSServicePage {
    pub action_tx: UnboundedSender<Action>,
    cloud_watch_component: CloudWatchLogsComponent,
}

impl Component for AWSServicePage {
    fn new(action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        AWSServicePage {
            action_tx: action_tx.clone(),
            cloud_watch_component: CloudWatchLogsComponent::new(action_tx.clone()),
        }
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::AWSService
    }

    fn handle_key_event(&mut self, key: KeyEvent, app_state: &AppState) -> anyhow::Result<()> {
        if !self.has_focus(app_state) {
            if TUI_CONFIG.key_config.focus_profiles.key_code == key.code
                && TUI_CONFIG.key_config.focus_profiles.key_modifier == key.modifiers
            {
                self.action_tx
                    .send(Action::SetFocus {
                        component_type: self.component_type(),
                    })
                    .context("Could not send action for focus update")?;
            }
        } else {
            match self.active_aws_service(app_state) {
                AWSService::CloudWatchLogs => self
                    .cloud_watch_component
                    .handle_key_event(key, app_state)?,
                AWSService::DynamoDB => {}
                _ => {}
            };
        }

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, app_state: &AppState) {
        match self.active_aws_service(app_state) {
            AWSService::CloudWatchLogs => self.cloud_watch_component.render(frame, area, app_state),
            _ => {
                frame.render_widget(
                    Block::new()
                        .title("AWS Service")
                        .title_alignment(Alignment::Center)
                        .border_style(Style::new().fg(TUI_CONFIG.theme.border))
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                    area,
                );
            }
        };
    }
}

impl AWSServicePage {
    fn has_focus(&self, app_state: &AppState) -> bool {
        app_state.focus_component == self.component_type()
    }

    fn active_aws_service(&self, app_state: &AppState) -> AWSService {
        if let Some(active_profile) = &app_state.active_profile {
            active_profile.selected_service.clone()
        } else {
            AWSService::None
        }
    }
}
