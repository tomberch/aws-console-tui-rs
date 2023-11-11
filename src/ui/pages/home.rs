use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use crate::state::actions::actions::Action;
use crate::state::appstate::{AppState, ComponentType};
use crate::ui::component::profiles::ProfilesComponent;
use crate::ui::component::regions::RegionsComponent;
use crate::ui::component::services::ServicesComponent;
use crate::ui::component::status::StatusComponent;
use crate::ui::component::toolbar::ToolbarComponent;
use crate::ui::component::Component;
use crate::ui::config::TUI_CONFIG;

use super::service::AWSServicePage;

pub struct HomePage<'a> {
    toolbar_component: ToolbarComponent,
    profiles_component: ProfilesComponent,
    regions_component: RegionsComponent,
    services_component: ServicesComponent<'a>,
    aws_service_page: AWSServicePage,
    status_component: StatusComponent,
}

impl<'a> Component for HomePage<'a> {
    fn new(action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        HomePage {
            toolbar_component: ToolbarComponent::new(action_tx.clone()),
            profiles_component: ProfilesComponent::new(action_tx.clone()),
            regions_component: RegionsComponent::new(action_tx.clone()),
            services_component: ServicesComponent::new(action_tx.clone()),
            aws_service_page: AWSServicePage::new(action_tx.clone()),
            status_component: StatusComponent::new(action_tx.clone()),
        }
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::Home
    }

    fn handle_key_event(&mut self, key: KeyEvent, app_state: &AppState) -> anyhow::Result<()> {
        if app_state.status_state.action_pending {
            return Ok(());
        }

        match key.code {
            KeyCode::Tab => {
                let component_type = match app_state.focus_component {
                    ComponentType::Profiles => ComponentType::Regions,
                    ComponentType::Regions => ComponentType::Services,
                    ComponentType::Services => ComponentType::AWSService,
                    ComponentType::AWSService => ComponentType::Profiles,
                    _ => return Ok(()),
                };
                self.send_focus_action(component_type)?;
            }
            KeyCode::BackTab => {
                let component_type = match app_state.focus_component {
                    ComponentType::Profiles => ComponentType::AWSService,
                    ComponentType::Regions => ComponentType::Profiles,
                    ComponentType::Services => ComponentType::Regions,
                    ComponentType::AWSService => ComponentType::Services,
                    _ => return Ok(()),
                };
                self.send_focus_action(component_type)?;
            }
            _ => {
                self.profiles_component.handle_key_event(key, app_state)?;
                self.regions_component.handle_key_event(key, app_state)?;
                self.services_component.handle_key_event(key, app_state)?;
                self.aws_service_page.handle_key_event(key, app_state)?;
            }
        }

        Ok(())
    }

    fn send_focus_action(&mut self, component_type: ComponentType) -> Result<(), anyhow::Error> {
        match component_type {
            ComponentType::Profiles => self.profiles_component.send_focus_action(component_type),
            ComponentType::Regions => self.regions_component.send_focus_action(component_type),
            ComponentType::Services => self.services_component.send_focus_action(component_type),
            ComponentType::AWSService => self.aws_service_page.send_focus_action(component_type),
            _ => Ok(()),
        }
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, app_state: &AppState) {
        frame.render_widget(
            Block::new().style(Style::new().bg(TUI_CONFIG.theme.background)),
            frame.size(),
        );

        let screen_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(7),
                Constraint::Min(1),
                Constraint::Length(2),
            ])
            .split(area);

        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(screen_layout[1]);

        let list_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(50),
            ])
            .split(main_layout[0]);

        self.toolbar_component
            .render(frame, screen_layout[0], app_state);
        self.profiles_component
            .render(frame, list_layout[0], app_state);
        self.regions_component
            .render(frame, list_layout[1], app_state);
        self.services_component
            .render(frame, list_layout[2], app_state);
        self.aws_service_page
            .render(frame, main_layout[1], app_state);
        self.status_component
            .render(frame, screen_layout[2], app_state);
    }
}
