use std::io::Stdout;

use crossterm::event::KeyEvent;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use crate::state::actions::actions::Action;
use crate::state::appstate::{AWSService, AppState, ComponentType};

use crate::ui::component::cloud_watch::CloudWatchComponent;
use crate::ui::component::{self, Component};

struct Props {
    action_pending: bool,
    focus_component: ComponentType,
}

impl From<&AppState> for Props {
    fn from(app_state: &AppState) -> Self {
        Props {
            action_pending: app_state.status_state.action_pending,
            focus_component: app_state.focus_component.clone(),
        }
    }
}

pub struct AWSServicePage {
    pub action_tx: UnboundedSender<Action>,
    active_aws_service: AWSService,
    aws_service_component: Option<Box<dyn Component>>,
    props: Props,
}

impl Component for AWSServicePage {
    fn new(app_state: &AppState, action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        AWSServicePage {
            action_tx: action_tx.clone(),
            active_aws_service: AWSService::None,
            aws_service_component: None,
            props: Props::from(app_state),
        }
    }

    fn move_with_state(self, app_state: &AppState) -> Self
    where
        Self: Sized,
    {
        let mut aws_service_component = None;

        if let Some(active_profile) = app_state.active_profile.as_ref() {
            if active_profile.selected_service != self.active_aws_service {
                aws_service_component = self.get_component(app_state);
            } else {
                aws_service_component = self
                    .aws_service_component
                    .map(|component| component.move_with_state(app_state))
            }
        }

        AWSServicePage {
            props: Props::from(app_state),
            aws_service_component,
            ..self
        }
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::AWSService
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> anyhow::Result<()> {
        if self.props.action_pending {
            return Ok(());
        }

        if let Some(component) = self.aws_service_component.as_mut() {
            component.handle_key_event(key);
        }

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame<'_, CrosstermBackend<Stdout>>, area: Rect) {
        if let Some(component) = self.aws_service_component.as_mut() {
            component.render(frame, area);
        } else {
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
    }
}

impl AWSServicePage {
    fn get_component(&self, app_state: &AppState) -> Option<Box<dyn Component>> {
        match app_state.active_profile.as_ref().unwrap().selected_service {
            AWSService::CloudWatchLogs => Some(Box::new(CloudWatchComponent::new(
                app_state,
                self.action_tx.clone(),
            ))),
            _ => None,
        }
    }
}
