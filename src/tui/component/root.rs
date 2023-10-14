use std::io::Stdout;

use crossterm::event::KeyEvent;
use ratatui::{prelude::*, widgets::*};

use super::awsservice::AwsServices;
use super::profiles::Profiles;
use super::regions::Regions;
use super::services::Services;
use crate::tui::app::AppState;
use crate::tui::config::TUI_CONFIG;

pub struct Components<'a> {
    profiles: Profiles<'a>,
    regions: Regions<'a>,
    services: Services<'a>,
    aws_service: AwsServices<'a>,
}

pub struct Root<'a> {
    pub app_state: &'a AppState,
    components: Components<'a>,
}
impl<'a> Root<'a> {
    pub fn new(app_state: &'a AppState) -> Self {
        let profiles = Profiles::new(app_state);
        let regions = Regions::new(app_state);
        let services = Services::new(app_state);
        let aws_services = AwsServices::new(app_state);

        Root {
            app_state,
            components: Components {
                profiles: profiles,
                regions: regions,
                services: services,
                aws_service: aws_services,
            },
        }
    }

    pub fn render(&mut self, frame: &mut Frame<'_, CrosstermBackend<Stdout>>, area: Rect) {
        frame.render_widget(Block::new().style(TUI_CONFIG.root), frame.size());

        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(area);

        let list_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(50),
            ])
            .split(main_layout[0]);

        self.components.profiles.render(frame, list_layout[0]);
        self.components.regions.render(frame, list_layout[1]);
        self.components.services.render(frame, list_layout[2]);
        self.components.aws_service.render(frame, main_layout[1]);
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            val if TUI_CONFIG.key_config.focus_profiles.key_code == val
                && TUI_CONFIG.key_config.focus_profiles.key_modifier == key.modifiers =>
            {
                self.clear_focus();
                self.components.profiles.has_focus = true;
            }
            val if TUI_CONFIG.key_config.focus_regions.key_code == val
                && TUI_CONFIG.key_config.focus_regions.key_modifier == key.modifiers =>
            {
                self.clear_focus();
                self.components.regions.has_focus = true;
            }
            val if TUI_CONFIG.key_config.focus_services.key_code == val
                && TUI_CONFIG.key_config.focus_services.key_modifier == key.modifiers =>
            {
                self.clear_focus();
                self.components.services.has_focus = true;
            }
            val if TUI_CONFIG.key_config.focus_aws_service.key_code == val
                && TUI_CONFIG.key_config.focus_aws_service.key_modifier == key.modifiers =>
            {
                self.clear_focus();
                self.components.aws_service.has_focus = true;
            }
            _ => {}
        };

        if self.components.profiles.has_focus {
            self.components.profiles.handle_key_events(key);
        }
    }

    fn clear_focus(&mut self) {
        self.components.profiles.has_focus = false;
        self.components.regions.has_focus = false;
        self.components.services.has_focus = false;
        self.components.aws_service.has_focus = false;
    }
}
