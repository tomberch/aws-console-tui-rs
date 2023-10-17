use std::io::Stdout;

use crossterm::event::KeyEvent;
use ratatui::{prelude::*, widgets::*};

use super::awsservice::AwsServices;
use super::profiles::Profiles;
use super::regions::Regions;
use super::services::Services;
use super::status::Status;
use crate::config::config::AppConfig;
use crate::tui::app::AppState;
use crate::tui::config::TUI_CONFIG;

pub struct Components<'a> {
    profiles: Profiles<'a>,
    regions: Regions<'a>,
    services: Services<'a>,
    aws_service: AwsServices<'a>,
    status: Status<'a>,
}

pub struct Root<'a> {
    pub app_config: &'a AppConfig,
    components: Components<'a>,
}
impl<'a> Root<'a> {
    pub fn new(app_config: &'a AppConfig) -> Self {
        Root {
            app_config,
            components: Components {
                profiles: Profiles::new(app_config),
                regions: Regions::new(app_config),
                services: Services::new(app_config),
                aws_service: AwsServices::new(app_config),
                status: Status::new(app_config),
            },
        }
    }

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

        self.components.profiles.render(frame, list_layout[0]);
        self.components.regions.render(frame, list_layout[1]);
        self.components.services.render(frame, list_layout[2]);
        self.components.aws_service.render(frame, main_layout[1]);
        self.components.status.render(frame, screen_layout[1]);
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
