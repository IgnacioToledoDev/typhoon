use std::time::Duration;
use crate::core::domain::{Language, TypingSession};
use crate::core::application::ports::{CorpusRepository, CorpusError, Clock, Rng};

pub struct StartSessionUseCase<Repo, R, C> {
    repo: Repo,
    rng: R,
    clock: C,
    duration: Duration,
}

#[derive(Debug)]
pub enum StartSessionError {
    Corpus(CorpusError),
}

impl std::fmt::Display for StartSessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StartSessionError::Corpus(e) => write!(f, "{e}"),
        }
    }
}
impl std::error::Error for StartSessionError {}

impl From<CorpusError> for StartSessionError {
    fn from(e: CorpusError) -> Self { StartSessionError::Corpus(e) }
}

impl<Repo, R, C> StartSessionUseCase<Repo, R, C>
where
    Repo: CorpusRepository,
    R: Rng,
    C: Clock,
{
    pub fn new(repo: Repo, rng: R, clock: C, duration: Duration) -> Self {
        Self { repo, rng, clock, duration }
    }

    pub fn execute(&mut self, language: Language) -> Result<TypingSession, StartSessionError> {
        let mut snippets = self.repo.list(language)?;
        if snippets.is_empty() {
            return Err(StartSessionError::Corpus(CorpusError::NoSnippets(language)));
        }
        let idx = (self.rng.next_u64() as usize) % snippets.len();
        let chosen = snippets.swap_remove(idx);
        Ok(TypingSession::new(chosen, self.clock.now(), self.duration))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::Instant;
    use crate::core::domain::Snippet;
    use crate::core::application::ports::{Clock, Rng};

    struct InMemoryRepo { snippets: Vec<Snippet> }
    impl CorpusRepository for InMemoryRepo {
        fn list(&self, _language: Language) -> Result<Vec<Snippet>, CorpusError> {
            Ok(self.snippets.clone())
        }
    }

    struct FixedClock(Instant);
    impl Clock for FixedClock { fn now(&self) -> Instant { self.0 } }

    struct SeededRng(u64);
    impl Rng for SeededRng { fn next_u64(&mut self) -> u64 { self.0 } }

    fn snippet(text: &str) -> Snippet {
        Snippet::new(text.into(), PathBuf::from(format!("{text}.rs")), Language::Rust).unwrap()
    }

    #[test]
    fn execute_picks_snippet_by_rng_modulo() {
        let repo = InMemoryRepo {
            snippets: vec![snippet("a"), snippet("b"), snippet("c")],
        };
        let rng = SeededRng(7);
        let clock = FixedClock(Instant::now());
        let mut uc = StartSessionUseCase::new(repo, rng, clock, Duration::from_secs(60));
        let session = uc.execute(Language::Rust).unwrap();
        assert_eq!(session.snippet().text(), "b");
    }

    #[test]
    fn execute_returns_no_snippets_when_repo_empty() {
        let repo = InMemoryRepo { snippets: vec![] };
        let rng = SeededRng(0);
        let clock = FixedClock(Instant::now());
        let mut uc = StartSessionUseCase::new(repo, rng, clock, Duration::from_secs(60));
        let err = uc.execute(Language::Rust).unwrap_err();
        assert!(matches!(err, StartSessionError::Corpus(CorpusError::NoSnippets(Language::Rust))));
    }
}
