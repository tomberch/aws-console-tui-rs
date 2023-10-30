use std::io::Stdout;

use anyhow::Context;
use crossterm::event::KeyEvent;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use crate::state::actions::actions::Action;
use crate::state::appstate::{AWSService, AppState, ComponentType};

use crate::ui::component::cloud_watch_logs::CloudWatchLogsComponent;
use crate::ui::component::Component;
use crate::ui::config::TUI_CONFIG;

struct Props {
    has_focus: bool,
    active_aws_service: AWSService,
}

impl From<&AppState> for Props {
    fn from(app_state: &AppState) -> Self {
        Props {
            has_focus: matches!(app_state.focus_component, ComponentType::AWSService),
            active_aws_service: if let Some(active_profile) = &app_state.active_profile {
                active_profile.selected_service.clone()
            } else {
                AWSService::None
            },
        }
    }
}

pub struct AWSServicePage {
    pub action_tx: UnboundedSender<Action>,
    cloud_watch_component: CloudWatchLogsComponent,
    props: Props,
}

impl Component for AWSServicePage {
    fn new(app_state: &AppState, action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        AWSServicePage {
            action_tx: action_tx.clone(),
            cloud_watch_component: CloudWatchLogsComponent::new(app_state, action_tx.clone()),
            props: Props::from(app_state),
        }
    }

    fn move_with_state(self, app_state: &AppState) -> Self
    where
        Self: Sized,
    {
        AWSServicePage {
            props: Props::from(app_state),
            cloud_watch_component: self.cloud_watch_component.move_with_state(app_state),
            ..self
        }
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::AWSService
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> anyhow::Result<()> {
        if !self.props.has_focus {
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
            match self.props.active_aws_service {
                AWSService::CloudWatchLogs => self.cloud_watch_component.handle_key_event(key)?,
                AWSService::DynamoDB => {}
                _ => {}
            };
        }

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame<'_, CrosstermBackend<Stdout>>, area: Rect) {
        match self.props.active_aws_service {
            AWSService::CloudWatchLogs => self.cloud_watch_component.render(frame, area),
            _ => {
                frame.render_widget(
                    Block::new()
                        .title("AWS Service")
                        .title_alignment(Alignment::Center)
                        .border_style(Style::new().fg(Color::White))
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                    area,
                );
            }
        };
    }
}
