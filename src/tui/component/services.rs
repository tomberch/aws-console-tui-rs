use ratatui::{prelude::*, widgets::*};

pub struct Services {}

impl Services {
    pub fn new() -> Self {
        Services {}
    }
}

impl Widget for Services {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let items = [
            ListItem::new("Item 1"),
            ListItem::new("Item 2"),
            ListItem::new("Item 3"),
        ];
        ratatui::widgets::Widget::render(
            List::new(items)
                .block(
                    Block::default()
                        .title("Services")
                        .title_alignment(Alignment::Center)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                .highlight_symbol(">>"),
            area,
            buf,
        );
    }
}
