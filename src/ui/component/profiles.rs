use std::{cmp::min, collections::HashMap, io::Stdout};

use aws_config::SdkConfig;
use crossterm::event::KeyEvent;
use ratatui::{
    prelude::{Alignment, CrosstermBackend, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Scrollbar, ScrollbarState},
    Frame,
};

use crate::{
    repository::{login::create_aws_config, profile::get_available_profiles},
    tui::{
        app::AppState,
        config::TUI_CONFIG,
    }, config::config::AppConfig,
};

pub struct Profile {
    pub name: String,
    pub sdk_config: SdkConfig,
    pub account: String,
    pub user: String,
    pub err_message: Option<String>,
    pub regions: Vec<String>,
}

pub struct Profiles<'a> {
    app_config:&'a AppConfig,
    profile_names: Vec<String>,
    selected_index: u16,
    active_profile_index: Option<u16>,
    pub has_focus: bool,
    profiles: HashMap<String, Profile>,
}

impl<'a> Profiles<'a> {
    pub fn new(app_state: &'a AppState) -> Self {
        let items = get_available_profiles(&app_state.aws_config).unwrap();
        Profiles {
            app_state,
            profile_names: items.clone(),
            selected_index: 0,
            active_profile_index: None,
            has_focus: true,
            profiles: HashMap::new(),
        }
    }

    pub fn render(&mut self, frame: &mut Frame<'_, CrosstermBackend<Stdout>>, area: Rect) {
        let list_items = self
            .profile_names
            .iter()
            .enumerate()
            .map(|(index, element)| ListItem::new(self.get_list_item_text(index, element)))
            .collect::<Vec<ListItem>>();

        let mut list_state = ListState::default().with_selected(Some(self.selected_index.into()));

        frame.render_stateful_widget(
            List::new(list_items)
                .block(
                    Block::default()
                        .title(format!(
                            " Profiles [{}] ",
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
            val if TUI_CONFIG.list_config.selection_up == val => {
                self.selected_index = if self.selected_index > 0 {
                    self.selected_index - 1
                } else {
                    0
                }
            }
            val if TUI_CONFIG.list_config.selection_down == val => {
                self.selected_index = min(self.selected_index + 1, self.get_list_len() - 1);
            }
            val if TUI_CONFIG.list_config.do_selection == val => {
                self.set_active_profile(self.selected_index);
            }
            _ => {}
        };
    }

    fn get_list_len(&self) -> u16 {
        self.profile_names.len().try_into().unwrap()
    }

    fn get_list_item_text(&self, index: usize, list_item_string: &'a str) -> Text<'a> {
        let is_active_profile_index = match self.active_profile_index {
            None => false,
            Some(active_index) => usize::try_from(active_index)
                .map(|active_index| active_index == index)
                .unwrap_or(false),
        };

        if is_active_profile_index {
            Text::styled(
                format!("**{}", list_item_string),
                Style::default().fg(Color::Yellow),
            )
        } else {
            Text::from(list_item_string)
        }
    }

    async fn set_active_profile(&'a mut self, index: u16) {
        self.active_profile_index = Some(index);
        let profile_name = &self.profile_names[usize::from(index)];

        if let Some(active_profile) = self.profiles.get(profile_name) {
            self.app_state.profile = &'a mut Some(active_profile);
        } else {
            let aws_config = create_aws_config(&self.app_state.aws_config).await;
        }
    }
}
