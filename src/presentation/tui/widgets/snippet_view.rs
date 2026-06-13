use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Widget, Wrap};

const TAB_WIDTH: usize = 4;

pub struct SnippetViewWidget<'a> {
    pub target: &'a str,
    pub typed: &'a str,
}

fn mistyped_glyph(c: char) -> String {
    match c {
        '\n' => "⏎".to_string(),
        '\t' => "→".to_string(),
        ' ' => "␣".to_string(),
        _ => c.to_string(),
    }
}

impl<'a> Widget for SnippetViewWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let typed_chars: Vec<char> = self.typed.chars().collect();
        let cursor_pos = typed_chars.len();
        let pending_style = Style::default().fg(Color::DarkGray);
        let correct_style = Style::default().fg(Color::Green);
        let incorrect_style = Style::default()
            .fg(Color::White)
            .bg(Color::Red)
            .add_modifier(Modifier::UNDERLINED);
        let cursor_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .add_modifier(Modifier::BOLD);

        let mut lines: Vec<Line<'static>> = Vec::new();
        let mut current: Vec<Span<'static>> = Vec::new();

        for (idx, ch) in self.target.chars().enumerate() {
            let style = if idx < cursor_pos {
                if typed_chars[idx] == ch { correct_style } else { incorrect_style }
            } else if idx == cursor_pos {
                cursor_style
            } else {
                pending_style
            };

            let mistyped = idx < cursor_pos && typed_chars[idx] != ch;

            match ch {
                '\n' => {
                    let glyph = if mistyped {
                        mistyped_glyph(typed_chars[idx])
                    } else {
                        "⏎".to_string()
                    };
                    current.push(Span::styled(glyph, style));
                    lines.push(Line::from(std::mem::take(&mut current)));
                }
                '\t' => {
                    if mistyped {
                        current.push(Span::styled(mistyped_glyph(typed_chars[idx]), style));
                    } else {
                        let pad = " ".repeat(TAB_WIDTH - 1);
                        current.push(Span::styled("→".to_string(), style));
                        let pad_style = if idx < cursor_pos { style } else { pending_style };
                        current.push(Span::styled(pad, pad_style));
                    }
                }
                _ => {
                    let display = if mistyped {
                        mistyped_glyph(typed_chars[idx])
                    } else {
                        ch.to_string()
                    };
                    current.push(Span::styled(display, style));
                }
            }
        }
        if !current.is_empty() || lines.is_empty() {
            lines.push(Line::from(current));
        }

        let paragraph = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title(" snippet "))
            .wrap(Wrap { trim: false });
        paragraph.render(area, buf);
    }
}
