use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget};

pub struct FooterWidget {
    pub finished: bool,
}

impl Widget for FooterWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let hint = if self.finished {
            "ENTER restart · ESC quit"
        } else {
            "ESC quit"
        };
        let line = Line::from(Span::styled(hint, Style::default().fg(Color::Gray)));
        Paragraph::new(line).render(area, buf);
    }
}
