use std::{cmp::min, io::Stdout};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::{Alignment, CrosstermBackend, Rect},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Scrollbar, ScrollbarState},
    Frame,
};

use crate::{
    repository::profile::get_available_profiles,
    tui::{app::AppState, config::TUI_CONFIG},
};

pub struct Profiles<'a> {
    app_state: &'a AppState,
    items: Vec<String>,
    selected_index: u16,
    pub has_focus: bool,
}

impl<'a> Profiles<'a> {
    pub fn new(app_state: &'a AppState) -> Self {
        //
        //  TODO: Implement Error Handling with app state
        //
        let items = get_available_profiles(&app_state.aws_config).unwrap();
        Profiles {
            app_state,
            items: items.clone(),
            selected_index: items
                .iter()
                .position(|element| element.as_str() == app_state.selected_profile)
                .unwrap_or(0)
                .try_into()
                .unwrap(),
            has_focus: true,
        }
    }

    pub fn render(&mut self, frame: &mut Frame<'_, CrosstermBackend<Stdout>>, area: Rect) {
        let list_items = self
            .items
            .iter()
            .map(|element| ListItem::new(element.as_str()))
            .collect::<Vec<ListItem>>();

        let mut list_state = ListState::default().with_selected(Some(self.selected_index.into()));

        frame.render_stateful_widget(
            List::new(list_items)
                .block(
                    Block::default()
                        .title(format!(
                            "Profiles [{}]",
                            TUI_CONFIG.key_config.focus_profiles.key_string
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
                .highlight_style(TUI_CONFIG.list_config.selected_style)
                .highlight_symbol(&TUI_CONFIG.list_config.selected_symbol),
            area,
            &mut list_state,
        );

        let mut scrollbar_state = ScrollbarState::default()
            .content_length(self.get_list_len())
            .position(self.selected_index);
        frame.render_stateful_widget(
            Scrollbar::default()
                .begin_symbol(None)
                .end_symbol(None)
                .track_symbol(None)
                .thumb_symbol("▐"),
            area,
            &mut scrollbar_state,
        );
    }

    pub fn handle_key_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('u') => {
                self.selected_index = if self.selected_index > 0 {
                    self.selected_index - 1
                } else {
                    0
                }
            }
            KeyCode::Char('d') => {
                self.selected_index = min(self.selected_index + 1, self.get_list_len() - 1);
            }
            _ => {}
        };
    }

    fn get_list_len(&self) -> u16 {
        self.items.len().try_into().unwrap()
    }
}
