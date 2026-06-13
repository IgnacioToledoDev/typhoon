use std::time::{Duration, Instant};
use crate::core::domain::{Snippet, Keystroke, KeystrokeOutcome, Stats};
use crate::core::application::services::{gross_wpm, net_wpm, accuracy};

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

    pub fn elapsed(&self, now: Instant) -> Duration {
        now.saturating_duration_since(self.started_at)
    }

    pub fn is_finished(&self, now: Instant) -> bool {
        if self.elapsed(now) >= self.duration_target {
            return true;
        }
        self.typed.chars().count() >= self.snippet.text().chars().count()
    }

    pub fn stats(&self, now: Instant) -> Stats {
        let elapsed = self.elapsed(now).min(self.duration_target);
        let remaining = self.duration_target.saturating_sub(elapsed);
        let typed_chars = self.typed.chars().count() as u32;
        Stats {
            elapsed,
            remaining,
            typed_chars,
            correct_chars: self.correct_chars,
            errors: self.errors,
            gross_wpm: gross_wpm(typed_chars, elapsed),
            net_wpm: net_wpm(typed_chars, self.errors, elapsed),
            accuracy: accuracy(self.correct_chars, typed_chars),
        }
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

    #[test]
    fn is_finished_when_time_runs_out() {
        let start = Instant::now();
        let s = TypingSession::new(rust_snippet("hello world"), start, Duration::from_secs(60));
        let later = start + Duration::from_secs(61);
        assert!(s.is_finished(later));
        assert!(!s.is_finished(start + Duration::from_secs(59)));
    }

    #[test]
    fn is_finished_when_snippet_complete() {
        let start = Instant::now();
        let mut s = TypingSession::new(rust_snippet("hi"), start, Duration::from_secs(60));
        s.apply(Keystroke::Char('h'));
        s.apply(Keystroke::Char('i'));
        assert!(s.is_finished(start + Duration::from_secs(1)));
    }

    #[test]
    fn stats_reflects_session_state() {
        let start = Instant::now();
        let mut s = TypingSession::new(rust_snippet("ab"), start, Duration::from_secs(60));
        s.apply(Keystroke::Char('a'));
        s.apply(Keystroke::Char('x'));
        let stats = s.stats(start + Duration::from_secs(60));
        assert_eq!(stats.typed_chars, 2);
        assert_eq!(stats.correct_chars, 1);
        assert_eq!(stats.errors, 1);
        assert_eq!(stats.elapsed, Duration::from_secs(60));
        assert_eq!(stats.remaining, Duration::from_secs(0));
        assert!((stats.gross_wpm - 0.4).abs() < 1e-6);
        assert_eq!(stats.net_wpm, 0.0);
        assert!((stats.accuracy - 0.5).abs() < 1e-6);
    }

    #[test]
    fn stats_remaining_clamped_to_zero() {
        let start = Instant::now();
        let s = TypingSession::new(rust_snippet("hi"), start, Duration::from_secs(60));
        let stats = s.stats(start + Duration::from_secs(90));
        assert_eq!(stats.remaining, Duration::ZERO);
    }
}
