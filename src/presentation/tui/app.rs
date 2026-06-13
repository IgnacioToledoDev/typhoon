use std::io;
use std::time::Duration;
use crossterm::event::{self, Event};
use crate::core::application::ports::{Clock, CorpusRepository, Rng};
use crate::core::application::use_cases::{
    ProcessKeystrokeUseCase, StartSessionUseCase, StartSessionError, TickUseCase,
};
use crate::core::domain::Language;
use crate::presentation::tui::input_handler::{map_key_event, InputCommand};
use crate::presentation::tui::renderer;
use crate::presentation::tui::terminal_guard::TerminalGuard;

const POLL_INTERVAL: Duration = Duration::from_millis(50);

#[derive(Debug)]
pub enum AppError {
    StartSession(StartSessionError),
    Io(io::Error),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::StartSession(e) => write!(f, "{e}"),
            AppError::Io(e) => write!(f, "{e}"),
        }
    }
}
impl std::error::Error for AppError {}

impl From<StartSessionError> for AppError {
    fn from(e: StartSessionError) -> Self { AppError::StartSession(e) }
}
impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self { AppError::Io(e) }
}

pub fn run<Repo, R, C>(
    mut start_uc: StartSessionUseCase<Repo, R, C>,
    process_uc: ProcessKeystrokeUseCase,
    tick_uc: TickUseCase<C>,
    language: Language,
) -> Result<(), AppError>
where
    Repo: CorpusRepository,
    R: Rng,
    C: Clock,
{
    let mut guard = TerminalGuard::enter()?;
    let mut session = start_uc.execute(language)?;

    loop {
        let stats = tick_uc.execute(&session);
        let snippet_complete =
            session.typed().chars().count() >= session.snippet().text().chars().count();
        let finished = stats.remaining.is_zero() || snippet_complete;

        renderer::draw(&mut guard.terminal, &session, &stats, finished)?;

        if event::poll(POLL_INTERVAL)? {
            if let Event::Key(key_event) = event::read()? {
                match map_key_event(key_event, finished) {
                    InputCommand::Quit => break,
                    InputCommand::Restart => {
                        session = start_uc.execute(language)?;
                    }
                    InputCommand::Apply(k) if !finished => {
                        process_uc.execute(&mut session, k);
                    }
                    InputCommand::Apply(_) | InputCommand::Ignore => {}
                }
            }
        }
    }
    Ok(())
}
