use std::io::Stdout;

use ratatui::{
    prelude::{CrosstermBackend, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::tui::{app::AppState, config::TUI_CONFIG};

pub struct Status<'a> {
    app_state: &'a AppState,
}

impl<'a> Status<'a> {
    pub fn new(app_state: &'a AppState) -> Self {
        Status { app_state }
    }

    pub fn render(&mut self, frame: &mut Frame<'_, CrosstermBackend<Stdout>>, area: Rect) {
        let status_text: Vec<Line> = match self.app_state.profile {
            None => {
                vec![Line::styled(
                    "No profile active. Please select profile and press <Enter>",
                    Style::default().fg(Color::DarkGray),
                )]
            }
            _ => vec![],
        };

        frame.render_widget(
            Paragraph::new(status_text).block(
                Block::default()
                    .border_style(Style::new().fg(Color::White))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            ),
            area,
        );
    }
}
