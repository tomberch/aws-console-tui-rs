use std::{cmp::min, collections::HashMap};

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

use crate::{
    state::{
        actions::actions::{Action, ProfileAction},
        appstate::{AppState, ComponentType, ProfileSource},
    },
    ui::config::TUI_CONFIG,
};

use super::Component;

struct Props {
    items: HashMap<String, ProfileSource>,
    has_focus: bool,
}

impl From<&AppState> for Props {
    fn from(app_state: &AppState) -> Self {
        Props {
            items: app_state.profile_state.profile_names.clone(),
            has_focus: matches!(app_state.focus_component, ComponentType::Profiles),
        }
    }
}
pub struct ProfilesComponent {
    action_tx: UnboundedSender<Action>,
    props: Props,
    selected_index: u16,
    active_profile_index: Option<u16>,
}

impl Component for ProfilesComponent {
    fn new(app_state: &AppState, action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        ProfilesComponent {
            action_tx: action_tx.clone(),
            props: Props::from(app_state),
            selected_index: 0,
            active_profile_index: None,
        }
    }

    fn move_with_state(self, app_state: &AppState) -> Self
    where
        Self: Sized,
    {
        ProfilesComponent {
            props: Props::from(app_state),
            ..self
        }
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::Profiles
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> anyhow::Result<()> {
        if !self.props.has_focus {
            if TUI_CONFIG.key_config.focus_profiles.key_code == key.code
                && TUI_CONFIG.key_config.focus_profiles.key_modifier == key.modifiers
            {
                self.action_tx
                    .send(Action::SetFocus {
                        component_type: self.component_type(),
                    })
                    .context("Could not send action for focus update")?;
            }
        } else if self.get_list_len() > 0 {
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
                    self.set_active_profile(self.selected_index)?;
                }
                _ => {}
            };
        }

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let list_items = self
            .props
            .items
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
                        .border_style(if self.props.has_focus {
                            TUI_CONFIG.focus_border
                        } else {
                            TUI_CONFIG.non_focus_border
                        })
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
    fn get_list_len(&self) -> u16 {
        self.props.items.len().try_into().unwrap()
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

    fn set_active_profile(&mut self, index: u16) -> anyhow::Result<()> {
        if let Some(active_index) = self.active_profile_index {
            if active_index == index {
                return Ok(());
            }
        }

        self.active_profile_index = Some(index);

        let profile_name =
            &self.props.items.keys().cloned().collect::<Vec<String>>()[usize::from(index)];

        self.action_tx.send(Action::Profile {
            action: ProfileAction::SelectProfile {
                profile: (
                    profile_name.clone(),
                    self.props.items.get(profile_name).unwrap().to_owned(),
                ),
            },
        })?;

        Ok(())
    }
}
