use std::io::Stdout;

use ratatui::{
    prelude::{Alignment, CrosstermBackend, Rect},
    widgets::{Block, BorderType, Borders},
    Frame,
};

use crate::tui::{app::AppState, config::TUI_CONFIG};

pub struct AwsServices<'a> {
    app_state: &'a AppState,
    pub has_focus: bool,
}

impl<'a> AwsServices<'a> {
    pub fn new(app_state: &'a AppState) -> Self {
        AwsServices {
            app_state,
            has_focus: false,
        }
    }

    pub fn render(&mut self, frame: &mut Frame<'_, CrosstermBackend<Stdout>>, area: Rect) {
        frame.render_widget(
            Block::default()
                .title(format!(
                    "AWS Service [{}]",
                    TUI_CONFIG.key_config.focus_aws_service.key_string
                ))
                .title_alignment(Alignment::Center)
                .border_style(if self.has_focus {
                    TUI_CONFIG.focus_border
                } else {
                    TUI_CONFIG.non_focus_border
                })
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
            area,
        );
    }
}
