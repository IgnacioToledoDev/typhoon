use std::time::{Duration, Instant};
use crate::core::domain::{Snippet, Keystroke, KeystrokeOutcome, Stats};

pub struct TypingSession {
    snippet: Snippet,
    typed: String,
    started_at: Instant,
    duration_target: Duration,
    errors: u32,
    correct_chars: u32,
}

impl TypingSession {
    pub fn new(snippet: Snippet, started_at: Instant, duration_target: Duration) -> Self {
        Self {
            snippet,
            typed: String::new(),
            started_at,
            duration_target,
            errors: 0,
            correct_chars: 0,
        }
    }

    pub fn snippet(&self) -> &Snippet { &self.snippet }
    pub fn typed(&self) -> &str { &self.typed }
    pub fn started_at(&self) -> Instant { self.started_at }
    pub fn errors(&self) -> u32 { self.errors }
    pub fn correct_chars(&self) -> u32 { self.correct_chars }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use crate::core::domain::Language;

    fn rust_snippet(text: &str) -> Snippet {
        Snippet::new(text.into(), PathBuf::from("t.rs"), Language::Rust).unwrap()
    }

    #[test]
    fn new_session_has_empty_typed() {
        let now = Instant::now();
        let s = TypingSession::new(rust_snippet("hello"), now, Duration::from_secs(60));
        assert_eq!(s.typed(), "");
        assert_eq!(s.snippet().text(), "hello");
    }
}
