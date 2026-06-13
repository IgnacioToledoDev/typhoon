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

    pub fn apply(&mut self, key: Keystroke) -> KeystrokeOutcome {
        match key {
            Keystroke::Char(c) => self.apply_char(c),
            Keystroke::Backspace => self.apply_backspace(),
        }
    }

    fn apply_char(&mut self, c: char) -> KeystrokeOutcome {
        let target = self.snippet.text();
        let pos = self.typed.chars().count();
        let Some(expected) = target.chars().nth(pos) else {
            return KeystrokeOutcome::IgnoredBeyondEnd;
        };
        self.typed.push(c);
        if c == expected {
            self.correct_chars += 1;
            KeystrokeOutcome::Correct
        } else {
            self.errors += 1;
            KeystrokeOutcome::Incorrect
        }
    }

    fn apply_backspace(&mut self) -> KeystrokeOutcome {
        let Some(last_char) = self.typed.chars().last() else {
            return KeystrokeOutcome::IgnoredBeforeStart;
        };
        let target = self.snippet.text();
        let pos = self.typed.chars().count() - 1;
        let was_correct = target.chars().nth(pos) == Some(last_char);
        let new_len = self.typed.len() - last_char.len_utf8();
        self.typed.truncate(new_len);
        if was_correct {
            self.correct_chars = self.correct_chars.saturating_sub(1);
        }
        KeystrokeOutcome::Backspaced
    }
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

    #[test]
    fn apply_correct_char_advances_typed_and_increments_correct() {
        let now = Instant::now();
        let mut s = TypingSession::new(rust_snippet("hi"), now, Duration::from_secs(60));
        let outcome = s.apply(Keystroke::Char('h'));
        assert_eq!(outcome, KeystrokeOutcome::Correct);
        assert_eq!(s.typed(), "h");
        assert_eq!(s.correct_chars(), 1);
        assert_eq!(s.errors(), 0);
    }

    #[test]
    fn apply_incorrect_char_advances_typed_and_increments_errors() {
        let now = Instant::now();
        let mut s = TypingSession::new(rust_snippet("hi"), now, Duration::from_secs(60));
        let outcome = s.apply(Keystroke::Char('x'));
        assert_eq!(outcome, KeystrokeOutcome::Incorrect);
        assert_eq!(s.typed(), "x");
        assert_eq!(s.correct_chars(), 0);
        assert_eq!(s.errors(), 1);
    }

    #[test]
    fn apply_beyond_end_is_ignored() {
        let now = Instant::now();
        let mut s = TypingSession::new(rust_snippet("a"), now, Duration::from_secs(60));
        s.apply(Keystroke::Char('a'));
        let outcome = s.apply(Keystroke::Char('b'));
        assert_eq!(outcome, KeystrokeOutcome::IgnoredBeyondEnd);
        assert_eq!(s.typed(), "a");
    }

    #[test]
    fn backspace_at_start_is_ignored() {
        let now = Instant::now();
        let mut s = TypingSession::new(rust_snippet("hi"), now, Duration::from_secs(60));
        let outcome = s.apply(Keystroke::Backspace);
        assert_eq!(outcome, KeystrokeOutcome::IgnoredBeforeStart);
        assert_eq!(s.typed(), "");
    }

    #[test]
    fn backspace_shrinks_typed_but_keeps_errors() {
        let now = Instant::now();
        let mut s = TypingSession::new(rust_snippet("hi"), now, Duration::from_secs(60));
        s.apply(Keystroke::Char('x'));
        let outcome = s.apply(Keystroke::Backspace);
        assert_eq!(outcome, KeystrokeOutcome::Backspaced);
        assert_eq!(s.typed(), "");
        assert_eq!(s.errors(), 1);
        assert_eq!(s.correct_chars(), 0);
    }

    #[test]
    fn backspace_after_correct_decrements_correct() {
        let now = Instant::now();
        let mut s = TypingSession::new(rust_snippet("hi"), now, Duration::from_secs(60));
        s.apply(Keystroke::Char('h'));
        s.apply(Keystroke::Backspace);
        assert_eq!(s.typed(), "");
        assert_eq!(s.correct_chars(), 0);
        assert_eq!(s.errors(), 0);
    }
}
