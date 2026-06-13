use std::process::ExitCode;
use std::time::Duration;
use clap::Parser;
use thypoon::core::application::use_cases::{
    ProcessKeystrokeUseCase, StartSessionUseCase, TickUseCase,
};
use thypoon::infrastructure::clock::SystemClock;
use thypoon::infrastructure::corpus::FsCorpusRepository;
use thypoon::infrastructure::rng::Xorshift64;
use thypoon::presentation::cli::Cli;
use thypoon::presentation::tui;

const SESSION_DURATION: Duration = Duration::from_secs(60);

fn main() -> ExitCode {
    let cli = Cli::parse();
    let language = cli.language();

    let clock = SystemClock;
    let rng = Xorshift64::from_entropy();
    let repo = FsCorpusRepository::from_env();

    let start_uc = StartSessionUseCase::new(repo, rng, clock, SESSION_DURATION);
    let process_uc = ProcessKeystrokeUseCase::new();
    let tick_uc = TickUseCase::new(clock);

    match tui::run(start_uc, process_uc, tick_uc, language) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("thypoon: {e}");
            ExitCode::from(1)
        }
    }
}
