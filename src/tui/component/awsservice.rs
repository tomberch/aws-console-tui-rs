use ratatui::{prelude::*, widgets::*};

pub struct AwsService {}

impl AwsService {
    pub fn new() -> Self {
        AwsService {}
    }
}

impl Widget for AwsService {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::new()
            .title("AWS Service")
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded)
            .render(area, buf);
    }
}
