use std::cmp::min;

use ratatui::{
    style::{Color, Style},
    text::Text,
    widgets::ListItem,
};

pub struct ListComponent<'a> {
    selected_index: usize,
    active_index: Option<usize>,
    list_items: Vec<String>,
    tui_list_items: Vec<ListItem<'a>>,
}

impl ListComponent<'_> {
    pub fn new() -> Self {
        ListComponent {
            selected_index: 0,
            active_index: None,
            list_items: vec![],
            tui_list_items: vec![],
        }
    }

    pub fn from(list_items: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let mut list_component = ListComponent::new();
        list_component.create_list_items(list_items);
        list_component
    }

    pub fn set_selected_index(&mut self, index: usize) {
        self.selected_index = index;
    }

    pub fn set_active_index(&mut self, index: usize) {
        self.active_index = Some(index);
    }

    pub fn create_list_items(&mut self, list_items: impl IntoIterator<Item = impl Into<String>>) {
        self.list_items = list_items.into_iter().map(|item| item.into()).collect()
    }

    pub fn get_selected_index(&self) -> usize {
        self.selected_index
    }

    pub fn get_list_len(&self) -> usize {
        self.list_items.len()
    }

    pub fn has_list_elements(&self) -> bool {
        self.get_list_len() > 0
    }

    pub fn move_up(&mut self) {
        self.selected_index = if self.selected_index > 0 {
            self.selected_index - 1
        } else {
            0
        }
    }

    pub fn move_down(&mut self) {
        self.selected_index = min(self.selected_index + 1, self.get_list_len() - 1);
    }

    pub fn set_active_item(&mut self) -> Option<String> {
        if let Some(active_index) = self.active_index {
            if active_index == self.selected_index {
                return None;
            }
        }

        self.active_index = Some(self.selected_index);
        Some(self.list_items[self.selected_index].clone())
    }

    pub fn create_tui_list(&self) -> Vec<ListItem> {
        if !self.tui_list_items.is_empty() {
            return self.tui_list_items.clone();
        }

        self.list_items
            .iter()
            .enumerate()
            .map(|(index, _)| ListItem::new(self.create_text_item(index)))
            .collect::<Vec<ListItem>>()
    }

    pub fn create_text_item(&self, index: usize) -> Text {
        let is_active_index = match self.active_index {
            None => false,
            Some(active_index) => active_index == index,
        };

        let item = &self.list_items[index];

        if is_active_index {
            Text::styled(format!("**{}", item), Style::default().fg(Color::Yellow))
        } else {
            Text::from(item.clone())
        }
    }
}
