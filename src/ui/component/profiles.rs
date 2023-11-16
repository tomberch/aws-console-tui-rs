use crossterm::event::KeyEvent;

use ratatui::{
    prelude::{Alignment, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders, List, ListState},
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;

use super::{base::list_component::ListComponent, Component};
use crate::{
    state::{
        action_handlers::actions::{Action, ProfileAction},
        appstate::{AppState, ComponentType},
    },
    ui::tui_config::TUI_CONFIG,
};

pub struct ProfilesComponent<'a> {
    action_tx: UnboundedSender<Action>,
    profile_list: ListComponent<'a>,
}

impl<'a> Component for ProfilesComponent<'a> {
    fn new(action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        ProfilesComponent {
            action_tx: action_tx.clone(),
            profile_list: ListComponent::new(),
        }
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::Profiles
    }

    fn set_focus(&self) -> anyhow::Result<()> {
        self.action_tx.send(Action::SetBreadcrumbs {
            breadcrumbs: vec![TUI_CONFIG.breadcrumbs.profiles.into()],
        })?;

        self.action_tx.send(Action::SetMenu {
            menu_items: [
                vec![],
                vec![],
                vec![
                    TUI_CONFIG.menu.down.into(),
                    TUI_CONFIG.menu.up.into(),
                    TUI_CONFIG.menu.select.into(),
                ],
            ],
        })?;

        self.send_focus_action(&self.action_tx)
    }

    fn handle_key_event(&mut self, key: KeyEvent, app_state: &AppState) -> anyhow::Result<()> {
        if !self.has_focus(app_state) {
            if TUI_CONFIG.key_config.focus_profiles.key_code == key.code
                && TUI_CONFIG.key_config.focus_profiles.key_modifier == key.modifiers
            {
                self.set_focus()?;
            }
        } else if self.get_list_len(app_state) > 0 {
            match key.code {
                val if TUI_CONFIG.list_config.selection_up == val => self.profile_list.move_up(),
                val if TUI_CONFIG.list_config.selection_down == val => {
                    self.profile_list.move_down()
                }

                val if TUI_CONFIG.list_config.do_selection == val => {
                    self.set_active_profile(app_state)?;
                }
                _ => {}
            };
        }

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, app_state: &AppState) {
        if !self.profile_list.has_list_elements() {
            self.profile_list
                .create_list_items(app_state.profile_state.profile_names.keys())
        }

        let mut list_state =
            ListState::default().with_selected(Some(self.profile_list.get_selected_index()));

        frame.render_stateful_widget(
            List::new(self.profile_list.create_tui_list())
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

impl<'a> ProfilesComponent<'a> {
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

    fn set_active_profile(&mut self, app_state: &AppState) -> anyhow::Result<()> {
        if let Some(profile_name) = self.profile_list.set_active_item() {
            self.action_tx.send(Action::Profile {
                action: ProfileAction::SelectProfile {
                    profile: (
                        profile_name.clone(),
                        app_state
                            .profile_state
                            .profile_names
                            .get(&profile_name)
                            .unwrap()
                            .to_owned(),
                    ),
                },
            })?
        }

        Ok(())
    }
}
