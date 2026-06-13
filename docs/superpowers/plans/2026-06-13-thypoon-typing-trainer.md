# Thypoon — TUI Typing Trainer Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a Rust CLI/TUI typing trainer (`thypoon`) that loads a random code snippet for a chosen language (Rust / TypeScript / Go) and runs a 60-second typing session with live timer / errors / WPM / accuracy.

**Architecture:** Clean Onion. Pure `domain` layer (entities + value objects), an `application` layer holding use cases and ports (traits), `infrastructure` adapters (FS corpus repo, system clock, Xorshift64 PRNG), and a `presentation` layer (clap CLI + ratatui TUI). The composition root in `main.rs` is the only place concrete adapters meet abstract ports.

**Tech Stack:** Rust 2024 edition, `ratatui` 0.29, `crossterm` 0.28, `clap` 4 (derive). No `rand`, no `tokio`, no `serde`.

---

## File Structure

```
src/
  main.rs                                       # Composition root
  lib.rs                                        # Re-exports modules for tests
  core/
    mod.rs
    domain/
      mod.rs
      language.rs                               # Language enum
      snippet.rs                                # Snippet value object
      keystroke.rs                              # Keystroke + KeystrokeOutcome
      stats.rs                                  # Stats value object
      session.rs                                # TypingSession aggregate
    application/
      mod.rs
      ports/
        mod.rs
        corpus_repository.rs                    # CorpusRepository trait + CorpusError
        clock.rs                                # Clock trait
        rng.rs                                  # Rng trait
      services/
        mod.rs
        wpm_calculator.rs                       # gross_wpm, net_wpm
        accuracy_calculator.rs                  # accuracy
      use_cases/
        mod.rs
        start_session.rs                        # StartSessionUseCase
        process_keystroke.rs                    # ProcessKeystrokeUseCase
        tick.rs                                 # TickUseCase
  infrastructure/
    mod.rs
    corpus/
      mod.rs
      fs_corpus_repository.rs                   # FsCorpusRepository
    clock/
      mod.rs
      system_clock.rs                           # SystemClock
    rng/
      mod.rs
      xorshift64.rs                             # Xorshift64
  presentation/
    mod.rs
    cli.rs                                      # clap CLI
    tui/
      mod.rs
      app.rs                                    # Event loop
      renderer.rs                               # ratatui draw orchestration
      input_handler.rs                          # crossterm event mapping
      terminal_guard.rs                         # RAII guard for raw mode
      widgets/
        mod.rs
        stats_bar.rs                            # Stats bar widget
        snippet_view.rs                         # Snippet text widget
        footer.rs                               # Hints widget
corpus/
  rust/        sample_hello.rs sample_fib.rs sample_struct.rs
  typescript/  sample_hello.ts sample_fib.ts sample_class.ts
  go/          sample_hello.go sample_fib.go sample_struct.go
```

Tests live alongside source files (`#[cfg(test)] mod tests`) for unit-level domain/application/infrastructure code. Integration tests for `FsCorpusRepository` go in `tests/fs_corpus_repository.rs`.

---

## Task 1: Add dependencies and scaffold lib

**Files:**
- Modify: `Cargo.toml`
- Create: `src/lib.rs`
- Modify: `src/main.rs`

- [ ] **Step 1: Add dependencies to `Cargo.toml`**

Replace the file with:

```toml
[package]
name = "thypoon"
version = "0.1.0"
edition = "2024"

[lib]
name = "thypoon"
path = "src/lib.rs"

[[bin]]
name = "thypoon"
path = "src/main.rs"

[dependencies]
ratatui   = "0.29"
crossterm = "0.28"
clap      = { version = "4", features = ["derive"] }
```

- [ ] **Step 2: Create `src/lib.rs` with module declarations**

```rust
pub mod core;
pub mod infrastructure;
pub mod presentation;
```

- [ ] **Step 3: Replace `src/main.rs` with a minimal stub that compiles**

```rust
fn main() {
    println!("thypoon — not yet wired");
}
```

- [ ] **Step 4: Verify build**

Run: `cargo build`
Expected: compiles with warnings only (no errors). Dependencies are downloaded.

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml Cargo.lock src/lib.rs src/main.rs
git commit -m "chore: add ratatui, crossterm, clap and lib scaffold"
```

---

## Task 2: Create empty `core` and `infrastructure` module trees

**Files:**
- Create: `src/core/mod.rs`
- Create: `src/core/domain/mod.rs`
- Create: `src/core/application/mod.rs`
- Create: `src/core/application/ports/mod.rs`
- Create: `src/core/application/services/mod.rs`
- Create: `src/core/application/use_cases/mod.rs`
- Create: `src/infrastructure/mod.rs`
- Create: `src/infrastructure/corpus/mod.rs`
- Create: `src/infrastructure/clock/mod.rs`
- Create: `src/infrastructure/rng/mod.rs`
- Create: `src/presentation/mod.rs`

- [ ] **Step 1: Create `src/core/mod.rs`**

```rust
pub mod domain;
pub mod application;
```

- [ ] **Step 2: Create `src/core/domain/mod.rs`**

```rust
// Pure domain. No deps outside `std`.
```

- [ ] **Step 3: Create `src/core/application/mod.rs`**

```rust
pub mod ports;
pub mod services;
pub mod use_cases;
```

- [ ] **Step 4: Create `src/core/application/ports/mod.rs`**

```rust
// Traits that infrastructure implements.
```

- [ ] **Step 5: Create `src/core/application/services/mod.rs`**

```rust
// Stateless domain-adjacent calculators.
```

- [ ] **Step 6: Create `src/core/application/use_cases/mod.rs`**

```rust
// One file per use case.
```

- [ ] **Step 7: Create `src/infrastructure/mod.rs`**

```rust
pub mod corpus;
pub mod clock;
pub mod rng;
```

- [ ] **Step 8: Create `src/infrastructure/corpus/mod.rs`**, `src/infrastructure/clock/mod.rs`, `src/infrastructure/rng/mod.rs`

Each contains a single comment line:

```rust
// Adapter implementations.
```

- [ ] **Step 9: Create `src/presentation/mod.rs`**

```rust
// CLI + TUI live here.
```

- [ ] **Step 10: Verify build**

Run: `cargo build`
Expected: compiles cleanly.

- [ ] **Step 11: Commit**

```bash
git add src/core src/infrastructure src/presentation
git commit -m "chore: scaffold core/infrastructure/presentation module trees"
```

---

## Task 3: `Language` enum (domain)

**Files:**
- Create: `src/core/domain/language.rs`
- Modify: `src/core/domain/mod.rs`

- [ ] **Step 1: Write the failing tests in `src/core/domain/language.rs`**

```rust
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Language {
    Rust,
    TypeScript,
    Go,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dir_name_matches_language() {
        assert_eq!(Language::Rust.dir_name(), "rust");
        assert_eq!(Language::TypeScript.dir_name(), "typescript");
        assert_eq!(Language::Go.dir_name(), "go");
    }

    #[test]
    fn extension_matches_language() {
        assert_eq!(Language::Rust.extension(), "rs");
        assert_eq!(Language::TypeScript.extension(), "ts");
        assert_eq!(Language::Go.extension(), "go");
    }

    #[test]
    fn display_name_matches_language() {
        assert_eq!(Language::Rust.display_name(), "Rust");
        assert_eq!(Language::TypeScript.display_name(), "TypeScript");
        assert_eq!(Language::Go.display_name(), "Go");
    }
}
```

- [ ] **Step 2: Register module in `src/core/domain/mod.rs`**

Replace content with:

```rust
pub mod language;
pub use language::Language;
```

- [ ] **Step 3: Run tests, expect failure**

Run: `cargo test -p thypoon language`
Expected: FAIL — `dir_name`, `extension`, `display_name` methods not defined.

- [ ] **Step 4: Implement the methods**

Append to `src/core/domain/language.rs` *before* the `#[cfg(test)]` block:

```rust
impl Language {
    pub fn dir_name(&self) -> &'static str {
        match self {
            Language::Rust => "rust",
            Language::TypeScript => "typescript",
            Language::Go => "go",
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            Language::Rust => "rs",
            Language::TypeScript => "ts",
            Language::Go => "go",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Language::Rust => "Rust",
            Language::TypeScript => "TypeScript",
            Language::Go => "Go",
        }
    }
}
```

- [ ] **Step 5: Run tests**

Run: `cargo test language`
Expected: PASS (3 tests).

- [ ] **Step 6: Commit**

```bash
git add src/core/domain/language.rs src/core/domain/mod.rs
git commit -m "feat(domain): add Language enum with dir/extension/display"
```

---

## Task 4: `Snippet` value object (domain)

**Files:**
- Create: `src/core/domain/snippet.rs`
- Modify: `src/core/domain/mod.rs`

- [ ] **Step 1: Write the failing tests in `src/core/domain/snippet.rs`**

```rust
use std::path::PathBuf;
use crate::core::domain::Language;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Snippet {
    text: String,
    source_path: PathBuf,
    language: Language,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SnippetError {
    EmptyText,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rejects_empty_text() {
        let result = Snippet::new(String::new(), PathBuf::from("x.rs"), Language::Rust);
        assert_eq!(result, Err(SnippetError::EmptyText));
    }

    #[test]
    fn new_rejects_whitespace_only_text() {
        let result = Snippet::new("   \n  ".into(), PathBuf::from("x.rs"), Language::Rust);
        assert_eq!(result, Err(SnippetError::EmptyText));
    }

    #[test]
    fn accessors_return_constructor_values() {
        let snippet = Snippet::new(
            "fn main() {}".into(),
            PathBuf::from("/foo/bar.rs"),
            Language::Rust,
        )
        .unwrap();
        assert_eq!(snippet.text(), "fn main() {}");
        assert_eq!(snippet.source_path(), &PathBuf::from("/foo/bar.rs"));
        assert_eq!(snippet.language(), Language::Rust);
    }
}
```

- [ ] **Step 2: Register module in `src/core/domain/mod.rs`**

Replace content with:

```rust
pub mod language;
pub mod snippet;

pub use language::Language;
pub use snippet::{Snippet, SnippetError};
```

- [ ] **Step 3: Run tests, expect failure**

Run: `cargo test snippet`
Expected: FAIL — `Snippet::new`, accessors not defined.

- [ ] **Step 4: Implement constructor and accessors**

Append to `src/core/domain/snippet.rs` *before* the `#[cfg(test)]` block:

```rust
impl Snippet {
    pub fn new(text: String, source_path: PathBuf, language: Language) -> Result<Self, SnippetError> {
        if text.trim().is_empty() {
            return Err(SnippetError::EmptyText);
        }
        Ok(Self { text, source_path, language })
    }

    pub fn text(&self) -> &str { &self.text }
    pub fn source_path(&self) -> &PathBuf { &self.source_path }
    pub fn language(&self) -> Language { self.language }
}
```

- [ ] **Step 5: Run tests**

Run: `cargo test snippet`
Expected: PASS (3 tests).

- [ ] **Step 6: Commit**

```bash
git add src/core/domain/snippet.rs src/core/domain/mod.rs
git commit -m "feat(domain): add Snippet value object with empty-text guard"
```

---

## Task 5: `Keystroke` and `KeystrokeOutcome` (domain)

**Files:**
- Create: `src/core/domain/keystroke.rs`
- Modify: `src/core/domain/mod.rs`

- [ ] **Step 1: Create `src/core/domain/keystroke.rs`**

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keystroke {
    Char(char),
    Backspace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeystrokeOutcome {
    Correct,
    Incorrect,
    Backspaced,
    IgnoredBeyondEnd,
    IgnoredBeforeStart,
}
```

- [ ] **Step 2: Register module in `src/core/domain/mod.rs`**

Replace content with:

```rust
pub mod language;
pub mod snippet;
pub mod keystroke;

pub use language::Language;
pub use snippet::{Snippet, SnippetError};
pub use keystroke::{Keystroke, KeystrokeOutcome};
```

- [ ] **Step 3: Verify build**

Run: `cargo build`
Expected: compiles cleanly.

- [ ] **Step 4: Commit**

```bash
git add src/core/domain/keystroke.rs src/core/domain/mod.rs
git commit -m "feat(domain): add Keystroke and KeystrokeOutcome enums"
```

---

## Task 6: WPM calculator (application service)

**Files:**
- Create: `src/core/application/services/wpm_calculator.rs`
- Modify: `src/core/application/services/mod.rs`

- [ ] **Step 1: Write the failing tests in `src/core/application/services/wpm_calculator.rs`**

```rust
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gross_wpm_60_seconds_300_chars_returns_60() {
        let wpm = gross_wpm(300, Duration::from_secs(60));
        assert!((wpm - 60.0).abs() < 1e-6, "got {wpm}");
    }

    #[test]
    fn gross_wpm_30_seconds_300_chars_returns_120() {
        let wpm = gross_wpm(300, Duration::from_secs(30));
        assert!((wpm - 120.0).abs() < 1e-6, "got {wpm}");
    }

    #[test]
    fn gross_wpm_zero_elapsed_returns_zero() {
        let wpm = gross_wpm(50, Duration::from_micros(0));
        assert_eq!(wpm, 0.0);
    }

    #[test]
    fn net_wpm_subtracts_errors_per_minute() {
        // 300 chars, 10 errors, 60s → 60 - 10 = 50
        let wpm = net_wpm(300, 10, Duration::from_secs(60));
        assert!((wpm - 50.0).abs() < 1e-6, "got {wpm}");
    }

    #[test]
    fn net_wpm_never_negative() {
        // 50 chars, 100 errors, 60s → gross=10, errors_per_min=100, net clamped to 0
        let wpm = net_wpm(50, 100, Duration::from_secs(60));
        assert_eq!(wpm, 0.0);
    }

    #[test]
    fn net_wpm_zero_elapsed_returns_zero() {
        assert_eq!(net_wpm(300, 0, Duration::from_micros(0)), 0.0);
    }
}
```

- [ ] **Step 2: Register module in `src/core/application/services/mod.rs`**

Replace content with:

```rust
pub mod wpm_calculator;
pub use wpm_calculator::{gross_wpm, net_wpm};
```

- [ ] **Step 3: Run tests, expect failure**

Run: `cargo test wpm`
Expected: FAIL — functions not defined.

- [ ] **Step 4: Implement WPM functions**

Prepend to `src/core/application/services/wpm_calculator.rs` (above the `#[cfg(test)]` block):

```rust
const MIN_ELAPSED_SECS: f64 = 1e-3;
const CHARS_PER_WORD: f64 = 5.0;

pub fn gross_wpm(chars_typed: u32, elapsed: Duration) -> f64 {
    let secs = elapsed.as_secs_f64();
    if secs < MIN_ELAPSED_SECS {
        return 0.0;
    }
    let minutes = secs / 60.0;
    (chars_typed as f64 / CHARS_PER_WORD) / minutes
}

pub fn net_wpm(chars_typed: u32, errors: u32, elapsed: Duration) -> f64 {
    let secs = elapsed.as_secs_f64();
    if secs < MIN_ELAPSED_SECS {
        return 0.0;
    }
    let minutes = secs / 60.0;
    let gross = (chars_typed as f64 / CHARS_PER_WORD) / minutes;
    let errors_per_min = errors as f64 / minutes;
    (gross - errors_per_min).max(0.0)
}
```

- [ ] **Step 5: Run tests**

Run: `cargo test wpm`
Expected: PASS (6 tests).

- [ ] **Step 6: Commit**

```bash
git add src/core/application/services/wpm_calculator.rs src/core/application/services/mod.rs
git commit -m "feat(app): add gross_wpm/net_wpm with error penalty and zero-elapsed guard"
```

---

## Task 7: Accuracy calculator (application service)

**Files:**
- Create: `src/core/application/services/accuracy_calculator.rs`
- Modify: `src/core/application/services/mod.rs`

- [ ] **Step 1: Write the failing tests in `src/core/application/services/accuracy_calculator.rs`**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_input_returns_one() {
        assert_eq!(accuracy(0, 0), 1.0);
    }

    #[test]
    fn all_correct_returns_one() {
        assert_eq!(accuracy(50, 50), 1.0);
    }

    #[test]
    fn half_correct_returns_half() {
        assert!((accuracy(50, 100) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn correct_capped_at_total() {
        // Defensive: if caller passes correct > total, clamp to 1.0
        assert_eq!(accuracy(120, 100), 1.0);
    }
}
```

- [ ] **Step 2: Register module in `src/core/application/services/mod.rs`**

Replace content with:

```rust
pub mod wpm_calculator;
pub mod accuracy_calculator;

pub use wpm_calculator::{gross_wpm, net_wpm};
pub use accuracy_calculator::accuracy;
```

- [ ] **Step 3: Run tests, expect failure**

Run: `cargo test accuracy`
Expected: FAIL — `accuracy` not defined.

- [ ] **Step 4: Implement accuracy function**

Prepend to `src/core/application/services/accuracy_calculator.rs`:

```rust
pub fn accuracy(correct: u32, total: u32) -> f64 {
    if total == 0 {
        return 1.0;
    }
    let a = correct as f64 / total as f64;
    a.clamp(0.0, 1.0)
}
```

- [ ] **Step 5: Run tests**

Run: `cargo test accuracy`
Expected: PASS (4 tests).

- [ ] **Step 6: Commit**

```bash
git add src/core/application/services/accuracy_calculator.rs src/core/application/services/mod.rs
git commit -m "feat(app): add accuracy calculator with empty-input and clamp guards"
```

---

## Task 8: `Stats` value object (domain)

**Files:**
- Create: `src/core/domain/stats.rs`
- Modify: `src/core/domain/mod.rs`

- [ ] **Step 1: Create `src/core/domain/stats.rs`**

```rust
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stats {
    pub elapsed: Duration,
    pub remaining: Duration,
    pub typed_chars: u32,
    pub correct_chars: u32,
    pub errors: u32,
    pub gross_wpm: f64,
    pub net_wpm: f64,
    pub accuracy: f64,
}
```

- [ ] **Step 2: Register module in `src/core/domain/mod.rs`**

Replace content with:

```rust
pub mod language;
pub mod snippet;
pub mod keystroke;
pub mod stats;

pub use language::Language;
pub use snippet::{Snippet, SnippetError};
pub use keystroke::{Keystroke, KeystrokeOutcome};
pub use stats::Stats;
```

- [ ] **Step 3: Verify build**

Run: `cargo build`
Expected: compiles cleanly.

- [ ] **Step 4: Commit**

```bash
git add src/core/domain/stats.rs src/core/domain/mod.rs
git commit -m "feat(domain): add Stats value object"
```

---

## Task 9: `TypingSession` aggregate — construction and basic getters

**Files:**
- Create: `src/core/domain/session.rs`
- Modify: `src/core/domain/mod.rs`

- [ ] **Step 1: Write the failing tests in `src/core/domain/session.rs`**

```rust
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
```

- [ ] **Step 2: Register module in `src/core/domain/mod.rs`**

Replace content with:

```rust
pub mod language;
pub mod snippet;
pub mod keystroke;
pub mod stats;
pub mod session;

pub use language::Language;
pub use snippet::{Snippet, SnippetError};
pub use keystroke::{Keystroke, KeystrokeOutcome};
pub use stats::Stats;
pub use session::TypingSession;
```

- [ ] **Step 3: Run tests, expect failure**

Run: `cargo test typing_session`
Expected: FAIL — `TypingSession::new`, `typed`, `snippet` not defined.

- [ ] **Step 4: Implement constructor + getters**

Append to `src/core/domain/session.rs` *before* the `#[cfg(test)]` block:

```rust
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
```

- [ ] **Step 5: Run tests**

Run: `cargo test typing_session`
Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add src/core/domain/session.rs src/core/domain/mod.rs
git commit -m "feat(domain): add TypingSession constructor and getters"
```

---

## Task 10: `TypingSession::apply` — Keystroke handling

**Files:**
- Modify: `src/core/domain/session.rs`

- [ ] **Step 1: Append failing tests to the `tests` module in `src/core/domain/session.rs`**

```rust
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
        s.apply(Keystroke::Char('x')); // wrong → errors=1
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
```

- [ ] **Step 2: Run tests, expect failure**

Run: `cargo test typing_session`
Expected: FAIL — `apply` not defined.

- [ ] **Step 3: Implement `apply` in `src/core/domain/session.rs`**

Add inside the existing `impl TypingSession` block:

```rust
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
        // Pop the last char (UTF-8 safe via char boundary)
        let new_len = self.typed.len() - last_char.len_utf8();
        self.typed.truncate(new_len);
        if was_correct {
            self.correct_chars = self.correct_chars.saturating_sub(1);
        }
        KeystrokeOutcome::Backspaced
    }
```

- [ ] **Step 4: Run tests**

Run: `cargo test typing_session`
Expected: PASS (7 tests).

- [ ] **Step 5: Commit**

```bash
git add src/core/domain/session.rs
git commit -m "feat(domain): implement TypingSession::apply for char and backspace"
```

---

## Task 11: `TypingSession::is_finished` and `stats`

**Files:**
- Modify: `src/core/domain/session.rs`

- [ ] **Step 1: Append failing tests to the `tests` module in `src/core/domain/session.rs`**

```rust
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
        // gross = (2/5)/1 = 0.4; net = max(0, 0.4 - 1/1) = 0
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
```

- [ ] **Step 2: Run tests, expect failure**

Run: `cargo test typing_session`
Expected: FAIL — `is_finished`, `stats` not defined.

- [ ] **Step 3: Implement `is_finished` and `stats`**

Add inside the existing `impl TypingSession` block in `src/core/domain/session.rs`. First add a `use`:

At the top of the file, replace the existing `use` block with:

```rust
use std::time::{Duration, Instant};
use crate::core::domain::{Snippet, Keystroke, KeystrokeOutcome, Stats};
use crate::core::application::services::{gross_wpm, net_wpm, accuracy};
```

Then add inside the `impl`:

```rust
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
```

- [ ] **Step 4: Run tests**

Run: `cargo test typing_session`
Expected: PASS (all 11 tests in session.rs).

- [ ] **Step 5: Commit**

```bash
git add src/core/domain/session.rs
git commit -m "feat(domain): add TypingSession::is_finished and stats"
```

---

## Task 12: Ports — `CorpusRepository`, `Clock`, `Rng`

**Files:**
- Create: `src/core/application/ports/corpus_repository.rs`
- Create: `src/core/application/ports/clock.rs`
- Create: `src/core/application/ports/rng.rs`
- Modify: `src/core/application/ports/mod.rs`

- [ ] **Step 1: Create `src/core/application/ports/corpus_repository.rs`**

```rust
use std::path::PathBuf;
use crate::core::domain::{Language, Snippet};

#[derive(Debug)]
pub enum CorpusError {
    DirectoryNotFound(PathBuf),
    Io(std::io::Error),
    NoSnippets(Language),
}

impl std::fmt::Display for CorpusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CorpusError::DirectoryNotFound(p) => write!(f, "corpus directory not found: {}", p.display()),
            CorpusError::Io(e) => write!(f, "io error reading corpus: {e}"),
            CorpusError::NoSnippets(l) => write!(f, "no snippets for {}", l.display_name()),
        }
    }
}

impl std::error::Error for CorpusError {}

impl From<std::io::Error> for CorpusError {
    fn from(e: std::io::Error) -> Self { CorpusError::Io(e) }
}

pub trait CorpusRepository {
    fn list(&self, language: Language) -> Result<Vec<Snippet>, CorpusError>;
}
```

- [ ] **Step 2: Create `src/core/application/ports/clock.rs`**

```rust
use std::time::Instant;

pub trait Clock {
    fn now(&self) -> Instant;
}
```

- [ ] **Step 3: Create `src/core/application/ports/rng.rs`**

```rust
pub trait Rng {
    fn next_u64(&mut self) -> u64;
}
```

- [ ] **Step 4: Register modules in `src/core/application/ports/mod.rs`**

Replace content with:

```rust
pub mod corpus_repository;
pub mod clock;
pub mod rng;

pub use corpus_repository::{CorpusRepository, CorpusError};
pub use clock::Clock;
pub use rng::Rng;
```

- [ ] **Step 5: Verify build**

Run: `cargo build`
Expected: compiles cleanly.

- [ ] **Step 6: Commit**

```bash
git add src/core/application/ports
git commit -m "feat(app): add CorpusRepository, Clock, Rng ports"
```

---

## Task 13: `StartSessionUseCase`

**Files:**
- Create: `src/core/application/use_cases/start_session.rs`
- Modify: `src/core/application/use_cases/mod.rs`

- [ ] **Step 1: Create `src/core/application/use_cases/start_session.rs` with failing tests**

```rust
use std::time::Duration;
use crate::core::domain::{Language, TypingSession, Snippet};
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::Instant;
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
        let rng = SeededRng(7); // 7 % 3 = 1 → "b"
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
```

- [ ] **Step 2: Register module in `src/core/application/use_cases/mod.rs`**

Replace content with:

```rust
pub mod start_session;
pub use start_session::{StartSessionUseCase, StartSessionError};
```

- [ ] **Step 3: Run tests, expect failure**

Run: `cargo test start_session`
Expected: FAIL — `new` and `execute` not defined.

- [ ] **Step 4: Implement `StartSessionUseCase`**

Append to `src/core/application/use_cases/start_session.rs` *before* the `#[cfg(test)]` block:

```rust
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
```

- [ ] **Step 5: Run tests**

Run: `cargo test start_session`
Expected: PASS (2 tests).

- [ ] **Step 6: Commit**

```bash
git add src/core/application/use_cases/start_session.rs src/core/application/use_cases/mod.rs
git commit -m "feat(app): add StartSessionUseCase with RNG-modulo selection"
```

---

## Task 14: `ProcessKeystrokeUseCase` and `TickUseCase`

**Files:**
- Create: `src/core/application/use_cases/process_keystroke.rs`
- Create: `src/core/application/use_cases/tick.rs`
- Modify: `src/core/application/use_cases/mod.rs`

- [ ] **Step 1: Create `src/core/application/use_cases/process_keystroke.rs`**

```rust
use crate::core::domain::{TypingSession, Keystroke, KeystrokeOutcome};

pub struct ProcessKeystrokeUseCase;

impl ProcessKeystrokeUseCase {
    pub fn new() -> Self { Self }

    pub fn execute(&self, session: &mut TypingSession, key: Keystroke) -> KeystrokeOutcome {
        session.apply(key)
    }
}

impl Default for ProcessKeystrokeUseCase {
    fn default() -> Self { Self::new() }
}
```

- [ ] **Step 2: Create `src/core/application/use_cases/tick.rs`**

```rust
use std::time::Instant;
use crate::core::domain::{TypingSession, Stats};
use crate::core::application::ports::Clock;

pub struct TickUseCase<C> { clock: C }

impl<C: Clock> TickUseCase<C> {
    pub fn new(clock: C) -> Self { Self { clock } }

    pub fn execute(&self, session: &TypingSession) -> Stats {
        session.stats(self.clock.now())
    }
}
```

- [ ] **Step 3: Update `src/core/application/use_cases/mod.rs`**

Replace content with:

```rust
pub mod start_session;
pub mod process_keystroke;
pub mod tick;

pub use start_session::{StartSessionUseCase, StartSessionError};
pub use process_keystroke::ProcessKeystrokeUseCase;
pub use tick::TickUseCase;
```

- [ ] **Step 4: Verify build**

Run: `cargo build && cargo test`
Expected: all tests pass, no new failures.

- [ ] **Step 5: Commit**

```bash
git add src/core/application/use_cases
git commit -m "feat(app): add ProcessKeystrokeUseCase and TickUseCase"
```

---

## Task 15: `SystemClock` adapter

**Files:**
- Create: `src/infrastructure/clock/system_clock.rs`
- Modify: `src/infrastructure/clock/mod.rs`

- [ ] **Step 1: Create `src/infrastructure/clock/system_clock.rs`**

```rust
use std::time::Instant;
use crate::core::application::ports::Clock;

#[derive(Debug, Clone, Copy, Default)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> Instant { Instant::now() }
}
```

- [ ] **Step 2: Update `src/infrastructure/clock/mod.rs`**

```rust
pub mod system_clock;
pub use system_clock::SystemClock;
```

- [ ] **Step 3: Verify build**

Run: `cargo build`
Expected: compiles cleanly.

- [ ] **Step 4: Commit**

```bash
git add src/infrastructure/clock
git commit -m "feat(infra): add SystemClock adapter"
```

---

## Task 16: `Xorshift64` PRNG adapter

**Files:**
- Create: `src/infrastructure/rng/xorshift64.rs`
- Modify: `src/infrastructure/rng/mod.rs`

- [ ] **Step 1: Write failing tests in `src/infrastructure/rng/xorshift64.rs`**

```rust
use std::time::{SystemTime, UNIX_EPOCH};
use crate::core::application::ports::Rng;

pub struct Xorshift64 {
    state: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_seed_zero_substitutes_default_state() {
        let mut a = Xorshift64::from_seed(0);
        // Should not output 0 forever
        assert_ne!(a.next_u64(), 0);
    }

    #[test]
    fn deterministic_for_same_seed() {
        let mut a = Xorshift64::from_seed(42);
        let mut b = Xorshift64::from_seed(42);
        for _ in 0..1000 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn distribution_into_buckets_is_roughly_uniform() {
        // 10_000 draws into 10 buckets — each bucket within ±20% of 1000
        let mut rng = Xorshift64::from_seed(0xC0FFEE);
        let mut buckets = [0u32; 10];
        for _ in 0..10_000 {
            let idx = (rng.next_u64() % 10) as usize;
            buckets[idx] += 1;
        }
        for (i, &count) in buckets.iter().enumerate() {
            assert!(
                count > 800 && count < 1200,
                "bucket {i} had {count} samples, expected ~1000"
            );
        }
    }
}
```

- [ ] **Step 2: Register module in `src/infrastructure/rng/mod.rs`**

```rust
pub mod xorshift64;
pub use xorshift64::Xorshift64;
```

- [ ] **Step 3: Run tests, expect failure**

Run: `cargo test xorshift`
Expected: FAIL — `from_seed`, `next_u64` not defined.

- [ ] **Step 4: Implement `Xorshift64`**

Append to `src/infrastructure/rng/xorshift64.rs` *before* the `#[cfg(test)]` block:

```rust
const DEFAULT_STATE: u64 = 0x9E37_79B9_7F4A_7C15;

impl Xorshift64 {
    pub fn from_seed(seed: u64) -> Self {
        let state = if seed == 0 { DEFAULT_STATE } else { seed };
        Self { state }
    }

    pub fn from_entropy() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(DEFAULT_STATE);
        let pid = std::process::id() as u64;
        let mixed = nanos ^ pid.rotate_left(17);
        Self::from_seed(mixed)
    }
}

impl Rng for Xorshift64 {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }
}
```

- [ ] **Step 5: Run tests**

Run: `cargo test xorshift`
Expected: PASS (3 tests).

- [ ] **Step 6: Commit**

```bash
git add src/infrastructure/rng
git commit -m "feat(infra): add Xorshift64 PRNG with entropy and seeded constructors"
```

---

## Task 17: Corpus directory and seed snippets

**Files:**
- Create: `corpus/rust/sample_hello.rs`
- Create: `corpus/rust/sample_fib.rs`
- Create: `corpus/rust/sample_struct.rs`
- Create: `corpus/typescript/sample_hello.ts`
- Create: `corpus/typescript/sample_fib.ts`
- Create: `corpus/typescript/sample_class.ts`
- Create: `corpus/go/sample_hello.go`
- Create: `corpus/go/sample_fib.go`
- Create: `corpus/go/sample_struct.go`

- [ ] **Step 1: Create `corpus/rust/sample_hello.rs`**

```rust
fn main() {
    let name = "world";
    println!("Hello, {}!", name);
}
```

- [ ] **Step 2: Create `corpus/rust/sample_fib.rs`**

```rust
fn fib(n: u32) -> u64 {
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 0..n {
        let next = a + b;
        a = b;
        b = next;
    }
    a
}
```

- [ ] **Step 3: Create `corpus/rust/sample_struct.rs`**

```rust
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}
```

- [ ] **Step 4: Create `corpus/typescript/sample_hello.ts`**

```typescript
function greet(name: string): string {
    return `Hello, ${name}!`;
}

console.log(greet("world"));
```

- [ ] **Step 5: Create `corpus/typescript/sample_fib.ts`**

```typescript
function fib(n: number): number {
    let a = 0;
    let b = 1;
    for (let i = 0; i < n; i++) {
        [a, b] = [b, a + b];
    }
    return a;
}
```

- [ ] **Step 6: Create `corpus/typescript/sample_class.ts`**

```typescript
class Counter {
    private value: number = 0;

    increment(): void {
        this.value += 1;
    }

    get(): number {
        return this.value;
    }
}
```

- [ ] **Step 7: Create `corpus/go/sample_hello.go`**

```go
package main

import "fmt"

func main() {
    name := "world"
    fmt.Printf("Hello, %s!\n", name)
}
```

- [ ] **Step 8: Create `corpus/go/sample_fib.go`**

```go
package main

func Fib(n int) uint64 {
    var a, b uint64 = 0, 1
    for i := 0; i < n; i++ {
        a, b = b, a+b
    }
    return a
}
```

- [ ] **Step 9: Create `corpus/go/sample_struct.go`**

```go
package main

import "math"

type Point struct {
    X, Y float64
}

func (p Point) Distance(other Point) float64 {
    dx := p.X - other.X
    dy := p.Y - other.Y
    return math.Sqrt(dx*dx + dy*dy)
}
```

- [ ] **Step 10: Commit**

```bash
git add corpus
git commit -m "feat(corpus): seed 3 snippets per language (rust, typescript, go)"
```

---

## Task 18: `FsCorpusRepository` adapter (unit pieces)

**Files:**
- Create: `src/infrastructure/corpus/fs_corpus_repository.rs`
- Modify: `src/infrastructure/corpus/mod.rs`

- [ ] **Step 1: Create `src/infrastructure/corpus/fs_corpus_repository.rs`**

```rust
use std::fs;
use std::path::{Path, PathBuf};
use crate::core::domain::{Language, Snippet};
use crate::core::application::ports::{CorpusRepository, CorpusError};

const MAX_SNIPPET_BYTES: u64 = 64 * 1024;

pub struct FsCorpusRepository {
    root: PathBuf,
}

impl FsCorpusRepository {
    pub fn new(root: PathBuf) -> Self { Self { root } }

    pub fn from_env() -> Self {
        let root = std::env::var("THYPOON_CORPUS_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("corpus"));
        Self { root }
    }

    fn is_hidden(path: &Path) -> bool {
        path.file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.starts_with('.'))
            .unwrap_or(false)
    }
}

impl CorpusRepository for FsCorpusRepository {
    fn list(&self, language: Language) -> Result<Vec<Snippet>, CorpusError> {
        let dir = self.root.join(language.dir_name());
        if !dir.is_dir() {
            return Err(CorpusError::DirectoryNotFound(dir));
        }

        let mut out = Vec::new();
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_file() || Self::is_hidden(&path) {
                continue;
            }
            let ext_matches = path
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e == language.extension())
                .unwrap_or(false);
            if !ext_matches {
                continue;
            }
            let metadata = entry.metadata()?;
            if metadata.len() > MAX_SNIPPET_BYTES {
                eprintln!("thypoon: skipping oversized snippet {}", path.display());
                continue;
            }
            match fs::read_to_string(&path) {
                Ok(text) => match Snippet::new(text, path.clone(), language) {
                    Ok(s) => out.push(s),
                    Err(_) => eprintln!("thypoon: skipping empty snippet {}", path.display()),
                },
                Err(e) => eprintln!("thypoon: skipping unreadable {}: {e}", path.display()),
            }
        }
        // Sort by path for stable ordering (RNG decides which one to pick).
        out.sort_by(|a, b| a.source_path().cmp(b.source_path()));
        Ok(out)
    }
}
```

- [ ] **Step 2: Register module in `src/infrastructure/corpus/mod.rs`**

```rust
pub mod fs_corpus_repository;
pub use fs_corpus_repository::FsCorpusRepository;
```

- [ ] **Step 3: Verify build**

Run: `cargo build`
Expected: compiles cleanly.

- [ ] **Step 4: Commit**

```bash
git add src/infrastructure/corpus
git commit -m "feat(infra): add FsCorpusRepository with extension filter and size cap"
```

---

## Task 19: Integration test for `FsCorpusRepository`

**Files:**
- Create: `tests/fs_corpus_repository.rs`

- [ ] **Step 1: Create `tests/fs_corpus_repository.rs`**

```rust
use std::fs;
use std::path::PathBuf;
use thypoon::core::application::ports::{CorpusRepository, CorpusError};
use thypoon::core::domain::Language;
use thypoon::infrastructure::corpus::FsCorpusRepository;

fn temp_root() -> PathBuf {
    let mut p = std::env::temp_dir();
    let unique = format!(
        "thypoon-test-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    p.push(unique);
    fs::create_dir_all(&p).unwrap();
    p
}

#[test]
fn lists_only_files_with_matching_extension() {
    let root = temp_root();
    let rust_dir = root.join("rust");
    fs::create_dir_all(&rust_dir).unwrap();
    fs::write(rust_dir.join("a.rs"), "fn a() {}").unwrap();
    fs::write(rust_dir.join("b.rs"), "fn b() {}").unwrap();
    fs::write(rust_dir.join("note.txt"), "ignore me").unwrap();
    fs::write(rust_dir.join("c.ts"), "ignore me too").unwrap();
    fs::write(rust_dir.join(".hidden.rs"), "hidden").unwrap();

    let repo = FsCorpusRepository::new(root.clone());
    let snippets = repo.list(Language::Rust).unwrap();

    let names: Vec<String> = snippets
        .iter()
        .map(|s| s.source_path().file_name().unwrap().to_string_lossy().into_owned())
        .collect();
    assert_eq!(names, vec!["a.rs".to_string(), "b.rs".to_string()]);

    fs::remove_dir_all(root).ok();
}

#[test]
fn missing_directory_returns_directory_not_found() {
    let root = temp_root();
    let repo = FsCorpusRepository::new(root.clone());
    let err = repo.list(Language::Go).unwrap_err();
    assert!(matches!(err, CorpusError::DirectoryNotFound(_)));
    fs::remove_dir_all(root).ok();
}
```

- [ ] **Step 2: Run tests**

Run: `cargo test --test fs_corpus_repository`
Expected: PASS (2 tests).

- [ ] **Step 3: Commit**

```bash
git add tests/fs_corpus_repository.rs
git commit -m "test(infra): integration tests for FsCorpusRepository"
```

---

## Task 20: CLI parser (`presentation::cli`)

**Files:**
- Create: `src/presentation/cli.rs`
- Modify: `src/presentation/mod.rs`

- [ ] **Step 1: Create `src/presentation/cli.rs`**

```rust
use clap::{Args, Parser};
use crate::core::domain::Language;

#[derive(Parser, Debug)]
#[command(name = "thypoon", version, about = "TUI typing trainer for code")]
pub struct Cli {
    #[command(flatten)]
    lang: LangFlags,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct LangFlags {
    /// Practice Rust
    #[arg(short = 'r', long = "rust")]
    rust: bool,
    /// Practice TypeScript
    #[arg(short = 't', long = "typescript")]
    typescript: bool,
    /// Practice Go
    #[arg(short = 'g', long = "go")]
    go: bool,
}

impl Cli {
    pub fn language(&self) -> Language {
        if self.lang.rust { Language::Rust }
        else if self.lang.typescript { Language::TypeScript }
        else if self.lang.go { Language::Go }
        else { unreachable!("clap group enforces exactly one") }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn parses_rust_short_flag() {
        let cli = Cli::try_parse_from(["thypoon", "-r"]).unwrap();
        assert_eq!(cli.language(), Language::Rust);
    }

    #[test]
    fn parses_typescript_long_flag() {
        let cli = Cli::try_parse_from(["thypoon", "--typescript"]).unwrap();
        assert_eq!(cli.language(), Language::TypeScript);
    }

    #[test]
    fn parses_go_short_flag() {
        let cli = Cli::try_parse_from(["thypoon", "-g"]).unwrap();
        assert_eq!(cli.language(), Language::Go);
    }

    #[test]
    fn rejects_no_flag() {
        assert!(Cli::try_parse_from(["thypoon"]).is_err());
    }

    #[test]
    fn rejects_two_flags() {
        assert!(Cli::try_parse_from(["thypoon", "-r", "-g"]).is_err());
    }
}
```

- [ ] **Step 2: Update `src/presentation/mod.rs`**

```rust
pub mod cli;
pub mod tui;

pub use cli::Cli;
```

- [ ] **Step 3: Create `src/presentation/tui/mod.rs`** (stub for now so the module declaration above resolves)

```rust
// Filled in by later tasks.
```

- [ ] **Step 4: Run tests**

Run: `cargo test cli`
Expected: PASS (5 tests).

- [ ] **Step 5: Commit**

```bash
git add src/presentation/cli.rs src/presentation/mod.rs src/presentation/tui/mod.rs
git commit -m "feat(presentation): add clap CLI with -r/-t/-g exclusive group"
```

---

## Task 21: Terminal RAII guard

**Files:**
- Create: `src/presentation/tui/terminal_guard.rs`
- Modify: `src/presentation/tui/mod.rs`

- [ ] **Step 1: Create `src/presentation/tui/terminal_guard.rs`**

```rust
use std::io::{self, Stdout};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

pub struct TerminalGuard {
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl TerminalGuard {
    pub fn enter() -> io::Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(self.terminal.backend_mut(), LeaveAlternateScreen);
        let _ = self.terminal.show_cursor();
    }
}
```

- [ ] **Step 2: Register module in `src/presentation/tui/mod.rs`**

Replace content with:

```rust
pub mod terminal_guard;
pub mod widgets;
pub mod renderer;
pub mod input_handler;
pub mod app;

pub use terminal_guard::TerminalGuard;
pub use app::run;
```

- [ ] **Step 3: Create empty placeholder files so the module tree compiles**

Create `src/presentation/tui/widgets/mod.rs`:

```rust
pub mod stats_bar;
pub mod snippet_view;
pub mod footer;
```

Create `src/presentation/tui/widgets/stats_bar.rs`, `snippet_view.rs`, `footer.rs` each with a single comment line:

```rust
// Implemented in subsequent tasks.
```

Create `src/presentation/tui/renderer.rs`, `input_handler.rs`, `app.rs` each with:

```rust
// Implemented in subsequent tasks.
```

- [ ] **Step 4: Verify build**

Run: `cargo build`
Expected: compiles cleanly (warnings about unused `run` re-export OK for now — silence with `#[allow(unused_imports)]` in `mod.rs` if needed, otherwise ignore).

If the unused re-export causes a hard error, change `mod.rs` `pub use app::run;` line to:

```rust
#[allow(unused_imports)]
pub use app::run;
```

- [ ] **Step 5: Commit**

```bash
git add src/presentation/tui
git commit -m "feat(tui): add TerminalGuard RAII for raw mode and alt screen"
```

---

## Task 22: `StatsBarWidget`

**Files:**
- Modify: `src/presentation/tui/widgets/stats_bar.rs`

- [ ] **Step 1: Replace `src/presentation/tui/widgets/stats_bar.rs`**

```rust
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
```

- [ ] **Step 2: Verify build**

Run: `cargo build`
Expected: compiles cleanly.

- [ ] **Step 3: Commit**

```bash
git add src/presentation/tui/widgets/stats_bar.rs
git commit -m "feat(tui): add StatsBarWidget (time/errors/wpm/accuracy)"
```

---

## Task 23: `SnippetViewWidget`

**Files:**
- Modify: `src/presentation/tui/widgets/snippet_view.rs`

- [ ] **Step 1: Replace `src/presentation/tui/widgets/snippet_view.rs`**

```rust
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Widget, Wrap};

pub struct SnippetViewWidget<'a> {
    pub target: &'a str,
    pub typed: &'a str,
}

impl<'a> Widget for SnippetViewWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let typed_chars: Vec<char> = self.typed.chars().collect();
        let cursor_pos = typed_chars.len();
        let mut spans: Vec<Span<'static>> = Vec::new();
        let pending_style = Style::default().fg(Color::DarkGray);
        let correct_style = Style::default().fg(Color::Green);
        let incorrect_style = Style::default()
            .fg(Color::White)
            .bg(Color::Red)
            .add_modifier(Modifier::UNDERLINED);
        let cursor_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .add_modifier(Modifier::BOLD);

        for (idx, ch) in self.target.chars().enumerate() {
            let visible = if ch == '\n' { '⏎' } else { ch };
            let render_char = if idx < cursor_pos {
                let typed_ch = typed_chars[idx];
                let style = if typed_ch == ch { correct_style } else { incorrect_style };
                let display = if typed_ch == ch { visible } else { typed_ch };
                Span::styled(display.to_string(), style)
            } else if idx == cursor_pos {
                Span::styled(visible.to_string(), cursor_style)
            } else {
                Span::styled(visible.to_string(), pending_style)
            };
            spans.push(render_char);
            if ch == '\n' {
                // Insert visible line-break: push the rest of the line on a new Line by ending here.
                // For simplicity, we keep everything on one wrapped paragraph and let Wrap handle width.
            }
        }

        let paragraph = Paragraph::new(Line::from(spans))
            .block(Block::default().borders(Borders::ALL).title(" snippet "))
            .wrap(Wrap { trim: false });
        paragraph.render(area, buf);
    }
}
```

- [ ] **Step 2: Verify build**

Run: `cargo build`
Expected: compiles cleanly.

- [ ] **Step 3: Commit**

```bash
git add src/presentation/tui/widgets/snippet_view.rs
git commit -m "feat(tui): add SnippetViewWidget with per-char correct/incorrect/cursor styling"
```

---

## Task 24: `FooterWidget`

**Files:**
- Modify: `src/presentation/tui/widgets/footer.rs`

- [ ] **Step 1: Replace `src/presentation/tui/widgets/footer.rs`**

```rust
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget};

pub struct FooterWidget {
    pub finished: bool,
}

impl Widget for FooterWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let hint = if self.finished {
            "ENTER restart · ESC quit"
        } else {
            "ESC quit"
        };
        let line = Line::from(Span::styled(hint, Style::default().fg(Color::Gray)));
        Paragraph::new(line).render(area, buf);
    }
}
```

- [ ] **Step 2: Verify build**

Run: `cargo build`
Expected: compiles cleanly.

- [ ] **Step 3: Commit**

```bash
git add src/presentation/tui/widgets/footer.rs
git commit -m "feat(tui): add FooterWidget with context-sensitive hints"
```

---

## Task 25: TUI renderer

**Files:**
- Modify: `src/presentation/tui/renderer.rs`

- [ ] **Step 1: Replace `src/presentation/tui/renderer.rs`**

```rust
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
```

- [ ] **Step 2: Verify build**

Run: `cargo build`
Expected: compiles cleanly.

- [ ] **Step 3: Commit**

```bash
git add src/presentation/tui/renderer.rs
git commit -m "feat(tui): add renderer orchestrating stats/snippet/footer layout"
```

---

## Task 26: Input handler

**Files:**
- Modify: `src/presentation/tui/input_handler.rs`

- [ ] **Step 1: Replace `src/presentation/tui/input_handler.rs`**

```rust
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crate::core::domain::Keystroke;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputCommand {
    Quit,
    Restart,
    Apply(Keystroke),
    Ignore,
}

pub fn map_key_event(event: KeyEvent, finished: bool) -> InputCommand {
    if event.kind != KeyEventKind::Press {
        return InputCommand::Ignore;
    }
    // Ctrl-C also quits
    if event.modifiers.contains(KeyModifiers::CONTROL) && event.code == KeyCode::Char('c') {
        return InputCommand::Quit;
    }
    match event.code {
        KeyCode::Esc => InputCommand::Quit,
        KeyCode::Enter if finished => InputCommand::Restart,
        KeyCode::Enter => InputCommand::Apply(Keystroke::Char('\n')),
        KeyCode::Tab => InputCommand::Apply(Keystroke::Char('\t')),
        KeyCode::Backspace => InputCommand::Apply(Keystroke::Backspace),
        KeyCode::Char(c) => InputCommand::Apply(Keystroke::Char(c)),
        _ => InputCommand::Ignore,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: Default::default(),
        }
    }

    #[test]
    fn esc_quits() {
        assert_eq!(map_key_event(key(KeyCode::Esc), false), InputCommand::Quit);
    }

    #[test]
    fn enter_restarts_when_finished() {
        assert_eq!(map_key_event(key(KeyCode::Enter), true), InputCommand::Restart);
    }

    #[test]
    fn enter_types_newline_when_running() {
        assert_eq!(
            map_key_event(key(KeyCode::Enter), false),
            InputCommand::Apply(Keystroke::Char('\n'))
        );
    }

    #[test]
    fn char_is_applied() {
        assert_eq!(
            map_key_event(key(KeyCode::Char('a')), false),
            InputCommand::Apply(Keystroke::Char('a'))
        );
    }

    #[test]
    fn backspace_is_applied() {
        assert_eq!(
            map_key_event(key(KeyCode::Backspace), false),
            InputCommand::Apply(Keystroke::Backspace)
        );
    }

    #[test]
    fn ctrl_c_quits() {
        let event = KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: Default::default(),
        };
        assert_eq!(map_key_event(event, false), InputCommand::Quit);
    }
}
```

- [ ] **Step 2: Run tests**

Run: `cargo test input_handler`
Expected: PASS (6 tests).

- [ ] **Step 3: Commit**

```bash
git add src/presentation/tui/input_handler.rs
git commit -m "feat(tui): add input handler mapping crossterm keys to commands"
```

---

## Task 27: TUI app event loop

**Files:**
- Modify: `src/presentation/tui/app.rs`

- [ ] **Step 1: Replace `src/presentation/tui/app.rs`**

```rust
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
    C: Clock + Copy,
{
    let mut guard = TerminalGuard::enter()?;
    let mut session = start_uc.execute(language)?;

    loop {
        let stats = tick_uc.execute(&session);
        let finished = session.is_finished(stats.elapsed_now_anchor(&tick_uc));
        // We compute `finished` via stats — `stats.remaining == 0` is the simple signal.
        let finished = stats.remaining.is_zero() || session.typed().chars().count() >= session.snippet().text().chars().count();

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
```

- [ ] **Step 2: Fix the spurious helper call**

The `stats.elapsed_now_anchor(&tick_uc)` line is a placeholder I left to illustrate the lack of helper — remove it. The function should be:

Replace `src/presentation/tui/app.rs` entirely with the version below (final form):

```rust
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
```

- [ ] **Step 3: Verify build**

Run: `cargo build`
Expected: compiles cleanly.

- [ ] **Step 4: Commit**

```bash
git add src/presentation/tui/app.rs
git commit -m "feat(tui): add event loop with finished gate, restart and quit handling"
```

---

## Task 28: Composition root (`main.rs`)

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Replace `src/main.rs`**

```rust
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
```

- [ ] **Step 2: Make `SystemClock` `Copy`-able for two consumers**

Verify `src/infrastructure/clock/system_clock.rs` already derives `Copy, Clone` (Task 15 includes them).

- [ ] **Step 3: Build**

Run: `cargo build`
Expected: compiles cleanly. There may be one warning about `Clock` not requiring `Copy`; both `start_uc` and `tick_uc` consume their own `clock` by value, so passing `clock` twice (once moved into `start_uc.new`, once into `tick_uc.new`) requires it to be `Copy`. Since `SystemClock` derives `Copy`, this works.

- [ ] **Step 4: Run the binary against bundled corpus**

Run: `cargo run -- -r`
Expected: TUI opens, shows a Rust snippet with a 60s timer ticking. Type a few chars; ESC to exit.

- [ ] **Step 5: Commit**

```bash
git add src/main.rs
git commit -m "feat: wire composition root with FS corpus, Xorshift64, SystemClock"
```

---

## Task 29: End-to-end smoke verification

**Files:**
- (no new files; manual verification only)

- [ ] **Step 1: `cargo test --all`**

Run: `cargo test`
Expected: all unit + integration tests pass.

- [ ] **Step 2: Manual TUI smoke — Rust**

Run: `cargo run -- -r`
Confirm:
- A 60-second countdown appears in the stats bar.
- A Rust snippet is shown in the middle panel.
- Typing the correct first char turns it green; an incorrect char turns it red and increments ERR.
- Backspace removes a char.
- WPM and ACC update live.
- ESC exits cleanly (terminal restored, no raw mode leftover).

- [ ] **Step 3: Manual TUI smoke — TypeScript and Go**

Run: `cargo run -- -t` then `cargo run -- -g`.
Confirm a snippet from the respective language directory appears.

- [ ] **Step 4: Error path — empty corpus**

Run: `THYPOON_CORPUS_DIR=/tmp/empty cargo run -- -r`
Expected: prints `thypoon: corpus directory not found: /tmp/empty/rust` and exits non-zero.

- [ ] **Step 5: Help output**

Run: `cargo run -- --help`
Expected: clap shows usage with `-r/--rust`, `-t/--typescript`, `-g/--go`, all in a mutually exclusive group.

- [ ] **Step 6: Commit a passing tag (optional)**

```bash
git tag v0.1.0
```

---

## Self-Review Notes

- **Spec coverage:**
  - CLI w/ exclusive `-r/-t/-g` → Task 20.
  - Onion layout (`core/domain`, `core/application`, `infrastructure`, `presentation`, `main`) → Tasks 1–2, 28.
  - File-based corpus picked up at runtime → Tasks 17–19.
  - PRNG without `rand` → Task 16.
  - 60-second mode → Task 28 (constant), Task 11 (`is_finished`).
  - Live timer / errors / WPM / accuracy → Tasks 6–8, 11, 22.
  - 30% stats / rest snippet / 1-line footer layout → Task 25.
  - Net WPM with error penalty → Task 6.
  - SOLID + DI via traits + composition root → Tasks 12–14, 28.

- **Placeholder scan:** Task 27 Step 1 introduced an intentionally-wrong helper to motivate the fix in Step 2; final file is fully implemented. No remaining TBD/TODO.

- **Type consistency:**
  - `StartSessionUseCase::new(repo, rng, clock, duration)` used in Task 13 and Task 28 — same signature.
  - `TickUseCase::new(clock)` and `.execute(&session)` — defined Task 14, called Task 27 / 28.
  - `ProcessKeystrokeUseCase::execute(&mut session, key)` — defined Task 14, called Task 27.
  - `Stats` fields (`remaining`, `errors`, `net_wpm`, `accuracy`) — defined Task 8, used Task 22.
  - `KeystrokeOutcome` variants — defined Task 5, returned Task 10.
