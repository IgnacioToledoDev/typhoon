use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Widget};
use crate::core::domain::Stats;

pub struct StatsBarWidget<'a> {
    pub stats: &'a Stats,
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
        let timer = Paragraph::new(Line::from(vec![
            Span::styled("TIME ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{secs_left}s"), Style::default().add_modifier(Modifier::BOLD)),
        ]));
        let errors = Paragraph::new(Line::from(vec![
            Span::styled("ERR ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}", self.stats.errors),
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
        ]));
        let wpm = Paragraph::new(Line::from(vec![
            Span::styled("WPM ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{:.0}", self.stats.net_wpm),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            ),
        ]));
        let acc = Paragraph::new(Line::from(vec![
            Span::styled("ACC ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{:.1}%", self.stats.accuracy * 100.0),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
        ]));

        timer.render(cells[0], buf);
        errors.render(cells[1], buf);
        wpm.render(cells[2], buf);
        acc.render(cells[3], buf);
    }
}
