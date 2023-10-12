use ratatui::{prelude::*, widgets::*};

use crate::tui::theme::THEME;

pub struct Profiles {}

impl Profiles {
    pub fn new() -> Self {
        Profiles {}
    }
}

impl Widget for Profiles {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = THEME.email;
        let highlight_symbol = ">>";
        let selected_index = 1;
        let items = [
            ListItem::new("Item 1"),
            ListItem::new("Item 2"),
            ListItem::new("Item 3"),
        ];
        let length = items.len().try_into().unwrap();

        let mut state = ListState::default().with_selected(Some(1));
        StatefulWidget::render(
            List::new(items)
                .block(
                    Block::default()
                        .title("Profiles")
                        .title_alignment(Alignment::Center)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                .style(theme.inbox)
                .highlight_style(theme.selected_item)
                .highlight_symbol(highlight_symbol),
            area,
            buf,
            &mut state,
        );
        let mut scrollbar_state = ScrollbarState::default()
            .content_length(length)
            .position(selected_index);
        Scrollbar::default()
            .begin_symbol(None)
            .end_symbol(None)
            .track_symbol(None)
            .thumb_symbol("▐")
            .render(area, buf, &mut scrollbar_state);
    }
}
