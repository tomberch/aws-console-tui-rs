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

use crate::{
    state::{
        actions::actions::{Action, RegionAction},
        appstate::{AppState, ComponentType},
    },
    ui::config::TUI_CONFIG,
};

use super::Component;

pub struct RegionsComponent {
    action_tx: UnboundedSender<Action>,
    region_names: Vec<String>,
    selected_index: u16,
    active_region_index: Option<u16>,
    active_profile_name: Option<String>,
}

impl Component for RegionsComponent {
    fn new(action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        RegionsComponent {
            action_tx: action_tx.clone(),
            region_names: vec![],
            selected_index: 0,
            active_region_index: None,
            active_profile_name: None,
        }
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::Regions
    }

    fn handle_key_event(&mut self, key: KeyEvent, app_state: &AppState) -> anyhow::Result<()> {
        if !self.has_focus(app_state) {
            if TUI_CONFIG.key_config.focus_regions.key_code == key.code
                && TUI_CONFIG.key_config.focus_regions.key_modifier == key.modifiers
            {
                self.send_focus_action(self.component_type())?;
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
                    self.set_active_region(self.selected_index)?;
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
                breadcrumbs: vec![TUI_CONFIG.breadcrumbs.regions.into()],
                menu: vec![],
            })
            .context("Could not send action for focus update")?;
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, app_state: &AppState) {
        self.handle_profile_change(app_state);
        self.region_names = match &app_state.active_profile {
            Some(active_profile) => active_profile.regions.clone(),
            None => vec![],
        };

        let list_items = self
            .region_names
            .iter()
            .enumerate()
            .map(|(index, element)| ListItem::new(self.get_list_item_text(index, element.into())))
            .collect::<Vec<ListItem>>();

        let mut list_state = ListState::default().with_selected(Some(self.selected_index.into()));

        frame.render_stateful_widget(
            List::new(list_items)
                .block(
                    Block::default()
                        .title(format!(
                            " Regions [{}] ",
                            TUI_CONFIG.key_config.focus_regions.key_string
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

impl RegionsComponent {
    fn has_focus(&self, app_state: &AppState) -> bool {
        app_state.focus_component == self.component_type()
    }

    fn handle_profile_change(&mut self, app_state: &AppState) {
        if let Some(active_profile) = &app_state.active_profile {
            if self.active_profile_name.as_ref() != Some(&active_profile.name) {
                let _ = self.active_profile_name.insert(active_profile.name.clone());
                if let Some(selected_region) = &active_profile.selected_region {
                    self.region_names = active_profile.regions.clone();
                    if let Some(index) = active_profile
                        .regions
                        .iter()
                        .position(|x| x == selected_region)
                    {
                        let int_index: u16 = index.try_into().unwrap();
                        self.selected_index = int_index;
                        self.active_region_index = Some(int_index);
                    }
                }
            }
        }
    }

    fn get_list_len(&self) -> u16 {
        self.region_names.len().try_into().unwrap()
    }

    fn get_list_item_text(&self, index: usize, list_item_string: String) -> Text {
        let is_active_profile_index = match self.active_region_index {
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

    fn set_active_region(&mut self, index: u16) -> anyhow::Result<()> {
        if let Some(active_index) = self.active_region_index {
            if active_index == index {
                return Ok(());
            }
        }
        self.active_region_index = Some(index);
        let region_name = &self.region_names[usize::from(index)];

        self.action_tx.send(Action::Region {
            action: RegionAction::SelectRegion {
                region_name: (region_name.into()),
            },
        })?;

        Ok(())
    }
}
