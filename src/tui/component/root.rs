use ratatui::{prelude::*, widgets::*};

use crate::tui::theme::THEME;

use super::awsservice::AwsService;
use super::profiles::Profiles;
use super::regions::Regions;
use super::services::Services;

pub struct Root {
    profiles: Profiles,
    regions: Regions,
    services: Services,
    aws_service: AwsService,
}

impl Root {
    pub fn new() -> Self {
        Root {
            profiles: Profiles::new(),
            regions: Regions::new(),
            services: Services::new(),
            aws_service: AwsService::new(),
        }
    }
}

impl Widget for Root {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::new().style(THEME.root).render(area, buf);

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

        self.profiles.render(list_layout[0], buf);
        self.regions.render(list_layout[1], buf);
        self.services.render(list_layout[2], buf);

        self.aws_service.render(main_layout[1], buf);
    }
}

impl Root {
    fn render_title_bar(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        Paragraph::new(Span::styled("Ratatui", THEME.app_title)).render(layout[0], buf);
        let titles = vec!["", " Recipe ", " Email ", " Traceroute ", " Weather "];
        Tabs::new(titles)
            .style(THEME.tabs)
            .highlight_style(THEME.tabs_selected)
            .select(1)
            .divider("")
            .render(layout[1], buf);
    }
}
