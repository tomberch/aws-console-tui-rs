use std::cmp::min;

use anyhow::Context;
use crossterm::event::KeyEvent;

use ratatui::{
    prelude::{Alignment, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{
    state::{
        actions::actions::{Action, ProfileAction},
        appstate::{AppState, ComponentType},
    },
    ui::config::TUI_CONFIG,
};

pub struct ProfilesComponent {
    action_tx: UnboundedSender<Action>,
    selected_index: u16,
    active_profile_index: Option<u16>,
}

impl Component for ProfilesComponent {
    fn new(action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        ProfilesComponent {
            action_tx: action_tx.clone(),
            selected_index: 0,
            active_profile_index: None,
        }
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::Profiles
    }

    fn handle_key_event(&mut self, key: KeyEvent, app_state: &AppState) -> anyhow::Result<()> {
        if !self.has_focus(app_state) {
            if TUI_CONFIG.key_config.focus_profiles.key_code == key.code
                && TUI_CONFIG.key_config.focus_profiles.key_modifier == key.modifiers
            {
                self.send_focus_action(self.component_type())?;
            }
        } else if self.get_list_len(app_state) > 0 {
            match key.code {
                val if TUI_CONFIG.list_config.selection_up == val => {
                    self.selected_index = if self.selected_index > 0 {
                        self.selected_index - 1
                    } else {
                        0
                    }
                }
                val if TUI_CONFIG.list_config.selection_down == val => {
                    self.selected_index =
                        min(self.selected_index + 1, self.get_list_len(app_state) - 1);
                }
                val if TUI_CONFIG.list_config.do_selection == val => {
                    self.set_active_profile(self.selected_index, app_state)?;
                }
                _ => {}
            };
        }

        Ok(())
    }

    fn send_focus_action(&mut self, component_type: ComponentType) -> Result<(), anyhow::Error> {
        self.action_tx
            .send(Action::SetFocus {
                component_type,
                breadcrumbs: vec![TUI_CONFIG.breadcrumbs.profiles.into()],
                menu: vec![],
            })
            .context("Could not send action for focus update")?;
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, app_state: &AppState) {
        let list_items = app_state
            .profile_state
            .profile_names
            .keys()
            .enumerate()
            .map(|(index, element)| {
                ListItem::new(self.create_list_item_text(index, element.into()))
            })
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
                        .border_style(Style::new().fg(if self.has_focus(app_state) {
                            TUI_CONFIG.theme.border_highlight
                        } else {
                            TUI_CONFIG.theme.border
                        }))
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                .highlight_style(TUI_CONFIG.list_config.selected_style)
                .highlight_symbol(TUI_CONFIG.list_config.selected_symbol),
            area,
            &mut list_state,
        );
    }
}

impl ProfilesComponent {
    fn has_focus(&self, app_state: &AppState) -> bool {
        app_state.focus_component == self.component_type()
    }

    fn get_list_len(&self, app_state: &AppState) -> u16 {
        app_state
            .profile_state
            .profile_names
            .len()
            .try_into()
            .unwrap()
    }

    fn create_list_item_text(&self, index: usize, list_item_string: String) -> Text {
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

    fn set_active_profile(&mut self, index: u16, app_state: &AppState) -> anyhow::Result<()> {
        if let Some(active_index) = self.active_profile_index {
            if active_index == index {
                return Ok(());
            }
        }

        self.active_profile_index = Some(index);

        let profile_name = &app_state
            .profile_state
            .profile_names
            .keys()
            .cloned()
            .collect::<Vec<String>>()[usize::from(index)];

        self.action_tx.send(Action::Profile {
            action: ProfileAction::SelectProfile {
                profile: (
                    profile_name.clone(),
                    app_state
                        .profile_state
                        .profile_names
                        .get(profile_name)
                        .unwrap()
                        .to_owned(),
                ),
            },
        })?;

        Ok(())
    }
}
