use crossterm::event::KeyEvent;
use ratatui::{
    prelude::{Alignment, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders, List, ListState},
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    state::{
        action_handlers::actions::{Action, RegionAction},
        appstate::{AppState, ComponentType},
    },
    ui::tui_config::TUI_CONFIG,
};

use super::{base::list_component::ListComponent, Component};

pub struct RegionsComponent<'a> {
    action_tx: UnboundedSender<Action>,
    region_list: ListComponent<'a>,

    active_profile_name: Option<String>,
}

impl<'a> Component for RegionsComponent<'a> {
    fn new(action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        RegionsComponent {
            action_tx: action_tx.clone(),
            region_list: ListComponent::new(),
            active_profile_name: None,
        }
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::Regions
    }

    fn set_focus(&self) -> anyhow::Result<()> {
        self.action_tx.send(Action::SetBreadcrumbs {
            breadcrumbs: vec![TUI_CONFIG.breadcrumbs.regions.into()],
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
            if TUI_CONFIG.key_config.focus_regions.key_code == key.code
                && TUI_CONFIG.key_config.focus_regions.key_modifier == key.modifiers
            {
                self.set_focus()?;
            }
        } else if self.region_list.has_list_elements() {
            match key.code {
                val if TUI_CONFIG.list_config.selection_up == val => self.region_list.move_up(),
                val if TUI_CONFIG.list_config.selection_down == val => self.region_list.move_down(),
                val if TUI_CONFIG.list_config.do_selection == val => {
                    self.set_active_region()?;
                }
                _ => {}
            };
        }

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, app_state: &AppState) {
        self.handle_profile_change(app_state);
        self.region_list
            .create_list_items(match &app_state.active_profile {
                Some(active_profile) => active_profile.regions.clone(),
                None => vec![],
            });

        let mut list_state =
            ListState::default().with_selected(Some(self.region_list.get_selected_index()));

        frame.render_stateful_widget(
            List::new(self.region_list.create_tui_list())
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

impl<'a> RegionsComponent<'a> {
    fn has_focus(&self, app_state: &AppState) -> bool {
        app_state.focus_component == self.component_type()
    }

    fn handle_profile_change(&mut self, app_state: &AppState) {
        if let Some(active_profile) = &app_state.active_profile {
            if self.active_profile_name.as_ref() != Some(&active_profile.name) {
                let _ = self.active_profile_name.insert(active_profile.name.clone());
                if let Some(selected_region) = &active_profile.selected_region {
                    self.region_list
                        .create_list_items(active_profile.regions.clone());
                    if let Some(index) = active_profile
                        .regions
                        .iter()
                        .position(|x| x == selected_region)
                    {
                        self.region_list.set_selected_index(index);
                        self.region_list.set_active_index(index);
                    }
                }
            }
        }
    }

    fn set_active_region(&mut self) -> anyhow::Result<()> {
        if let Some(region_name) = self.region_list.set_active_item() {
            self.action_tx.send(Action::Region {
                action: RegionAction::SelectRegion { region_name },
            })?;
        }

        Ok(())
    }
}
