use std::io::Stdout;

use crossterm::event::KeyEvent;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use crate::state::actions::actions::Action;
use crate::state::appstate::{AppState, ComponentType};
use crate::ui::component::profiles::ProfilesComponent;
use crate::ui::component::regions::RegionsComponent;
use crate::ui::component::status::StatusComponent;
use crate::ui::component::Component;
use crate::ui::config::TUI_CONFIG;

struct Props {}

impl From<&AppState> for Props {
    fn from(app_state: &AppState) -> Self {
        Props {}
    }
}

pub struct HomePage {
    pub action_tx: UnboundedSender<Action>,
    props: Props,
    profiles_component: ProfilesComponent,
    regions_component: RegionsComponent,
    status_component: StatusComponent,
}

impl Component for HomePage {
    fn new(app_state: &AppState, action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        HomePage {
            action_tx: action_tx.clone(),
            props: Props::from(app_state),
            profiles_component: ProfilesComponent::new(app_state, action_tx.clone()),
            regions_component: RegionsComponent::new(app_state, action_tx.clone()),
            status_component: StatusComponent::new(app_state, action_tx.clone()),
        }
    }

    fn move_with_state(self, app_state: &AppState) -> Self
    where
        Self: Sized,
    {
        HomePage {
            props: Props::from(app_state),
            // propagate the update to the child components
            profiles_component: self.profiles_component.move_with_state(app_state),
            regions_component: self.regions_component.move_with_state(app_state),
            status_component: self.status_component.move_with_state(app_state),
            ..self
        }
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::Home
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> anyhow::Result<()> {
        self.profiles_component.handle_key_event(key)?;
        self.regions_component.handle_key_event(key)?;

        Ok(())
    }
}

impl HomePage {
    pub fn render(&mut self, frame: &mut Frame<'_, CrosstermBackend<Stdout>>, area: Rect) {
        frame.render_widget(Block::new().style(TUI_CONFIG.root), frame.size());

        let screen_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Min(1), Constraint::Length(4)])
            .split(area);

        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(screen_layout[0]);

        let list_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(50),
            ])
            .split(main_layout[0]);

        frame.render_widget(
            Block::new()
                .border_style(Style::new().fg(Color::White))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
            screen_layout[1],
        );

        self.profiles_component.render(frame, list_layout[0]);
        self.regions_component.render(frame, list_layout[1]);
        // self.components.services.render(frame, list_layout[2]);
        // self.components.aws_service.render(frame, main_layout[1]);
        self.status_component.render(frame, screen_layout[1]);
    }
}
