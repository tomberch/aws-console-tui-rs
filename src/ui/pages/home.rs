use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use crate::state::action_handlers::actions::Action;
use crate::state::appstate::{AWSService, AppState, ComponentType};
use crate::ui::component::profiles::ProfilesComponent;
use crate::ui::component::regions::RegionsComponent;
use crate::ui::component::services::ServicesComponent;
use crate::ui::component::status::StatusComponent;
use crate::ui::component::toolbar::ToolbarComponent;
use crate::ui::component::Component;
use crate::ui::tui_config::TUI_CONFIG;

use super::service::AWSServicePage;

pub struct HomePage<'a> {
    action_tx: UnboundedSender<Action>,
    toolbar_component: ToolbarComponent,
    profiles_component: ProfilesComponent<'a>,
    regions_component: RegionsComponent<'a>,
    services_component: ServicesComponent<'a>,
    aws_service_page: AWSServicePage,
    status_component: StatusComponent,
    old_focus_component_type: ComponentType,
}

impl<'a> Component for HomePage<'a> {
    fn new(action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        HomePage {
            action_tx: action_tx.clone(),
            toolbar_component: ToolbarComponent::new(action_tx.clone()),
            profiles_component: ProfilesComponent::new(action_tx.clone()),
            regions_component: RegionsComponent::new(action_tx.clone()),
            services_component: ServicesComponent::new(action_tx.clone()),
            aws_service_page: AWSServicePage::new(action_tx.clone()),
            status_component: StatusComponent::new(action_tx.clone()),
            old_focus_component_type: ComponentType::Profiles,
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
            KeyCode::Char('x')
                if app_state.focus_component == ComponentType::AWSService
                    && !app_state.is_expanded =>
            {
                self.action_tx.send(Action::ToggleSidePane)?
            }

            KeyCode::Char('c') if app_state.is_expanded => {
                self.action_tx.send(Action::ToggleSidePane)?
            }

            KeyCode::Tab if app_state.active_profile.is_some() => {
                let component_type = match app_state.focus_component {
                    ComponentType::Profiles => ComponentType::Regions,
                    ComponentType::Regions => ComponentType::Services,
                    ComponentType::Services
                        if app_state.active_profile.as_ref().unwrap().selected_service
                            != AWSService::None =>
                    {
                        ComponentType::AWSService
                    }
                    ComponentType::Services
                        if app_state.active_profile.as_ref().unwrap().selected_service
                            == AWSService::None =>
                    {
                        ComponentType::Profiles
                    }
                    ComponentType::AWSService => ComponentType::Profiles,
                    _ => return Ok(()),
                };
                self.set_child_component_focus(&component_type)?;
            }

            KeyCode::BackTab => {
                if app_state.active_profile.is_some() {
                    let component_type = match app_state.focus_component {
                        ComponentType::Profiles
                            if app_state.active_profile.as_ref().unwrap().selected_service
                                != AWSService::None =>
                        {
                            ComponentType::AWSService
                        }
                        ComponentType::Profiles
                            if app_state.active_profile.as_ref().unwrap().selected_service
                                == AWSService::None =>
                        {
                            ComponentType::Services
                        }
                        ComponentType::Regions => ComponentType::Profiles,
                        ComponentType::Services => ComponentType::Regions,
                        ComponentType::AWSService => ComponentType::Services,
                        _ => return Ok(()),
                    };
                    self.set_child_component_focus(&component_type)?;
                }
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

    fn render(&mut self, frame: &mut Frame, area: Rect, app_state: &AppState) {
        if self.old_focus_component_type != app_state.focus_component {
            self.old_focus_component_type = app_state.focus_component.clone();
            let _ = self.set_child_component_focus(&app_state.focus_component);
        }

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

        if app_state.is_expanded {
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

            self.profiles_component
                .render(frame, list_layout[0], app_state);
            self.regions_component
                .render(frame, list_layout[1], app_state);
            self.services_component
                .render(frame, list_layout[2], app_state);
            self.aws_service_page
                .render(frame, main_layout[1], app_state);
        } else {
            self.aws_service_page
                .render(frame, screen_layout[1], app_state);
        }

        self.toolbar_component
            .render(frame, screen_layout[0], app_state);
        self.status_component
            .render(frame, screen_layout[2], app_state);
    }
}

impl<'a> HomePage<'a> {
    pub fn set_initial_focus(&self) -> anyhow::Result<()> {
        self.profiles_component.set_focus()
    }

    fn set_child_component_focus(
        &self,
        component_type: &ComponentType,
    ) -> Result<(), anyhow::Error> {
        match component_type {
            ComponentType::Profiles => self.profiles_component.set_focus(),
            ComponentType::Regions => self.regions_component.set_focus(),
            ComponentType::Services => self.services_component.set_focus(),
            ComponentType::AWSService => self.aws_service_page.set_focus(),
            _ => Ok(()),
        }
    }
}
