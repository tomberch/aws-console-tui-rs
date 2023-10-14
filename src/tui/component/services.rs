use std::io::Stdout;

use ratatui::{
    prelude::{Alignment, CrosstermBackend, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem},
    Frame,
};

use crate::tui::{app::AppState, config::TUI_CONFIG};

pub struct Services<'a> {
    app_state: &'a AppState,
    pub has_focus: bool,
}

impl<'a> Services<'a> {
    pub fn new(app_state: &'a AppState) -> Self {
        Services {
            app_state,
            has_focus: false,
        }
    }

    pub fn render(&mut self, frame: &mut Frame<'_, CrosstermBackend<Stdout>>, area: Rect) {
        let items = [
            ListItem::new("Item 1"),
            ListItem::new("Item 2"),
            ListItem::new("Item 3"),
        ];
        frame.render_widget(
            List::new(items)
                .block(
                    Block::default()
                        .title(format!(
                            "Services [{}]",
                            TUI_CONFIG.key_config.focus_services.key_string
                        ))
                        .title_alignment(Alignment::Center)
                        .border_style(if self.has_focus {
                            TUI_CONFIG.focus_border
                        } else {
                            TUI_CONFIG.non_focus_border
                        })
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                .highlight_symbol(">>"),
            area,
        );
    }
}
