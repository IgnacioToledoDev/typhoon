use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Widget};
use crate::core::domain::Stats;

pub struct StatsBarWidget<'a> {
    pub stats: &'a Stats,
}

fn render_stat_cell(buf: &mut Buffer, area: Rect, label: &str, value: String, value_style: Style) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {label} "))
        .title_alignment(Alignment::Center);
    let inner = block.inner(area);
    block.render(area, buf);

    let vert_pad = inner.height.saturating_sub(1) / 2;
    let mut lines: Vec<Line<'static>> = (0..vert_pad).map(|_| Line::from("")).collect();
    lines.push(Line::from(Span::styled(value, value_style)));

    Paragraph::new(lines)
        .alignment(Alignment::Center)
        .render(inner, buf);
}

impl<'a> Widget for StatsBarWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" thypoon ");
        let inner = block.inner(area);
        block.render(area, buf);

        let cells = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(inner);

        let secs_left = self.stats.remaining.as_secs();
        render_stat_cell(
            buf, cells[0], "TIME",
            format!("{secs_left}s"),
            Style::default().add_modifier(Modifier::BOLD),
        );
        render_stat_cell(
            buf, cells[1], "ERR",
            format!("{}", self.stats.errors),
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        );
        render_stat_cell(
            buf, cells[2], "WPM",
            format!("{:.0}", self.stats.net_wpm),
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        );
        render_stat_cell(
            buf, cells[3], "ACC",
            format!("{:.1}%", self.stats.accuracy * 100.0),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        );
    }
}
