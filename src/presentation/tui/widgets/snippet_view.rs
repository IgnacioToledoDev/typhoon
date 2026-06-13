use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Widget, Wrap};

pub struct SnippetViewWidget<'a> {
    pub target: &'a str,
    pub typed: &'a str,
}

impl<'a> Widget for SnippetViewWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let typed_chars: Vec<char> = self.typed.chars().collect();
        let cursor_pos = typed_chars.len();
        let mut spans: Vec<Span<'static>> = Vec::new();
        let pending_style = Style::default().fg(Color::DarkGray);
        let correct_style = Style::default().fg(Color::Green);
        let incorrect_style = Style::default()
            .fg(Color::White)
            .bg(Color::Red)
            .add_modifier(Modifier::UNDERLINED);
        let cursor_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .add_modifier(Modifier::BOLD);

        for (idx, ch) in self.target.chars().enumerate() {
            let visible = if ch == '\n' { '⏎' } else { ch };
            let render_char = if idx < cursor_pos {
                let typed_ch = typed_chars[idx];
                let style = if typed_ch == ch { correct_style } else { incorrect_style };
                let display = if typed_ch == ch { visible } else { typed_ch };
                Span::styled(display.to_string(), style)
            } else if idx == cursor_pos {
                Span::styled(visible.to_string(), cursor_style)
            } else {
                Span::styled(visible.to_string(), pending_style)
            };
            spans.push(render_char);
        }

        let paragraph = Paragraph::new(Line::from(spans))
            .block(Block::default().borders(Borders::ALL).title(" snippet "))
            .wrap(Wrap { trim: false });
        paragraph.render(area, buf);
    }
}
