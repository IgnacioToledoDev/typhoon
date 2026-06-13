use std::io::Stdout;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::Terminal;
use crate::core::domain::{Stats, TypingSession};
use crate::presentation::tui::widgets::stats_bar::StatsBarWidget;
use crate::presentation::tui::widgets::snippet_view::SnippetViewWidget;
use crate::presentation::tui::widgets::footer::FooterWidget;

pub fn draw(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    session: &TypingSession,
    stats: &Stats,
    finished: bool,
) -> std::io::Result<()> {
    terminal.draw(|frame| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(frame.area());

        frame.render_widget(StatsBarWidget { stats }, chunks[0]);
        frame.render_widget(
            SnippetViewWidget {
                target: session.snippet().text(),
                typed: session.typed(),
            },
            chunks[1],
        );
        frame.render_widget(FooterWidget { finished }, chunks[2]);
    })?;
    Ok(())
}
