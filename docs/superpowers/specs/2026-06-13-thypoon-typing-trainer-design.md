# Thypoon — TUI Typing Trainer (Design Spec)

**Date:** 2026-06-13
**Status:** Approved (pending written-spec review)
**Author:** Ignacio Toledo

## 1. Purpose

A terminal-based typing trainer for programming languages. The user runs `thypoon` with a language flag, a random code snippet for that language is loaded, and the user types it during a 60-second session. The TUI shows live timer, error count, and WPM.

## 2. Goals & Non-Goals

### Goals
- Single binary CLI: `thypoon -r | -t | -g`.
- 60-second fixed mode.
- Live stats: timer, errors, gross/net WPM, accuracy.
- Three languages: Rust, TypeScript, Go.
- Corpus is file-based: dropping a new snippet file under `corpus/<lang>/` makes it eligible at next run.
- Random snippet selection without the `rand` crate.
- Clean Onion architecture; SOLID; testable domain core.

### Non-Goals (v1)
- Multiple session lengths (15 / 30 / 120 s).
- Persistent score history / leaderboard.
- Network features.
- Custom corpus formats (JSON / Markdown). Only raw `.rs` / `.ts` / `.go`.
- Themes / color customization.

## 3. CLI

```
thypoon (-r|--rust) | (-t|--typescript) | (-g|--go)
```

- Exactly one language flag required. Mutual exclusion enforced by `clap` argument group (`required = true, multiple = false`).
- `--help` and `--version` come for free via `clap`.
- Exit codes: `0` success, `1` corpus error (empty / unreadable), `2` CLI parse error (handled by clap).

## 4. Architecture — Clean Onion

### 4.1 Layers and dependency rule

```
┌─────────────────────────────────────────────────────────────┐
│  Composition Root (main.rs)  — wires concrete adapters      │
├─────────────────────────────────────────────────────────────┤
│  Presentation (TUI, CLI parser)                             │
├─────────────────────────────────────────────────────────────┤
│  Infrastructure (FS, Clock, PRNG)                           │
├─────────────────────────────────────────────────────────────┤
│  Application (Use cases + Ports/Traits)                     │
├─────────────────────────────────────────────────────────────┤
│  Domain (pure entities, value objects, services)            │
└─────────────────────────────────────────────────────────────┘
```

Dependency rule: arrows point inward only.

- `domain` depends on: `std` only.
- `application` depends on: `domain`.
- `infrastructure` depends on: `application` (implements its ports) + `domain`.
- `presentation` depends on: `application` + `domain`.
- `main.rs` (composition root) depends on: everything (the only allowed place).

### 4.2 Folder layout

```
src/
  main.rs
  core/
    mod.rs
    domain/
      mod.rs
      language.rs
      snippet.rs
      keystroke.rs
      stats.rs
      session.rs
    application/
      mod.rs
      ports/
        mod.rs
        corpus_repository.rs
        clock.rs
        rng.rs
      services/
        mod.rs
        wpm_calculator.rs
        accuracy_calculator.rs
      use_cases/
        mod.rs
        start_session.rs
        process_keystroke.rs
        tick.rs
  infrastructure/
    mod.rs
    corpus/
      mod.rs
      fs_corpus_repository.rs
    clock/
      mod.rs
      system_clock.rs
    rng/
      mod.rs
      xorshift64.rs
  presentation/
    mod.rs
    cli.rs
    tui/
      mod.rs
      app.rs
      renderer.rs
      input_handler.rs
      widgets/
        mod.rs
        stats_bar.rs
        snippet_view.rs
        footer.rs
corpus/
  rust/
    sample1.rs
    sample2.rs
    sample3.rs
  typescript/
    sample1.ts
    sample2.ts
    sample3.ts
  go/
    sample1.go
    sample2.go
    sample3.go
```

The `corpus/` directory lives at the crate root (sibling of `src/`) and is discovered at runtime via `env!("CARGO_MANIFEST_DIR")`. An optional `THYPOON_CORPUS_DIR` environment variable overrides the path for installed binaries.

## 5. Domain Model

### 5.1 `Language`

```rust
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Language { Rust, TypeScript, Go }

impl Language {
    pub fn dir_name(&self) -> &'static str { /* "rust" | "typescript" | "go" */ }
    pub fn extension(&self) -> &'static str { /* "rs" | "ts" | "go" */ }
    pub fn display_name(&self) -> &'static str { /* "Rust" | "TypeScript" | "Go" */ }
}
```

### 5.2 `Snippet`

```rust
pub struct Snippet {
    text: String,
    source_path: PathBuf,
    language: Language,
}
```

Immutable value object. Constructor validates non-empty text.

### 5.3 `Keystroke`

```rust
pub enum Keystroke {
    Char(char),
    Backspace,
}
```

### 5.4 `Stats`

```rust
pub struct Stats {
    pub elapsed: Duration,
    pub remaining: Duration,
    pub typed_chars: u32,
    pub correct_chars: u32,
    pub errors: u32,
    pub gross_wpm: f64,
    pub net_wpm: f64,
    pub accuracy: f64,  // 0.0 .. 1.0
}
```

### 5.5 `TypingSession` (aggregate)

```rust
pub struct TypingSession {
    snippet: Snippet,
    typed: String,
    started_at: Instant,
    duration_target: Duration,
    errors: u32,
}

impl TypingSession {
    pub fn new(snippet: Snippet, started_at: Instant, duration_target: Duration) -> Self;
    pub fn apply(&mut self, key: Keystroke) -> KeystrokeOutcome;
    pub fn is_finished(&self, now: Instant) -> bool;
    pub fn stats(&self, now: Instant) -> Stats;
    pub fn snippet(&self) -> &Snippet;
    pub fn typed(&self) -> &str;
}
```

Invariants:
- `typed.len() <= snippet.text().len()`.
- `errors` is monotonically increasing.
- Backspace decrements `typed` length but never decreases `errors`.

`KeystrokeOutcome { Correct, Incorrect, IgnoredBeyondEnd, IgnoredBeforeStart }` — used by the presentation layer for visual feedback.

## 6. Application Layer

### 6.1 Ports (traits)

```rust
pub trait CorpusRepository {
    fn list(&self, language: Language) -> Result<Vec<Snippet>, CorpusError>;
}

pub trait Clock {
    fn now(&self) -> Instant;
}

pub trait Rng {
    fn next_u64(&mut self) -> u64;
}
```

### 6.2 Services

#### WpmCalculator

```rust
// 1 word = 5 chars (standard typing benchmark).
// gross_wpm = (chars_typed / 5) / minutes
// net_wpm   = max(0, gross_wpm - (errors / minutes))
pub fn gross_wpm(chars_typed: u32, elapsed: Duration) -> f64;
pub fn net_wpm(chars_typed: u32, errors: u32, elapsed: Duration) -> f64;
```

Edge case: `elapsed < 1ms` → return `0.0` (avoid div-by-zero blow-up at session start).

#### AccuracyCalculator

```rust
pub fn accuracy(correct: u32, total: u32) -> f64;
```

`total == 0 → 1.0` (no input yet, do not display 0% accuracy).

### 6.3 Use Cases

#### `StartSessionUseCase`

```rust
pub struct StartSessionUseCase<Repo, R, C>
where Repo: CorpusRepository, R: Rng, C: Clock {
    repo: Repo, rng: R, clock: C, duration: Duration,
}

impl ... {
    pub fn execute(&mut self, language: Language) -> Result<TypingSession, StartSessionError>;
}
```

Algorithm:
1. `snippets = repo.list(language)?`
2. If empty → `Err(NoSnippets)`.
3. `idx = rng.next_u64() as usize % snippets.len()`
4. `Ok(TypingSession::new(snippets.swap_remove(idx), clock.now(), self.duration))`.

#### `ProcessKeystrokeUseCase`

Thin wrapper: forwards `Keystroke` to `session.apply` and returns `KeystrokeOutcome`. Exists so presentation never imports the domain mutator directly (preserves dependency rule).

#### `TickUseCase`

Returns updated `Stats` for current `Instant`. Pure — no side effects.

## 7. Infrastructure Adapters

### 7.1 `FsCorpusRepository`

```rust
pub struct FsCorpusRepository { root: PathBuf }

impl FsCorpusRepository {
    pub fn from_env() -> Self {
        let root = std::env::var("THYPOON_CORPUS_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("corpus"));
        Self { root }
    }
}

impl CorpusRepository for FsCorpusRepository {
    fn list(&self, language: Language) -> Result<Vec<Snippet>, CorpusError> {
        let dir = self.root.join(language.dir_name());
        // read_dir, filter by extension == language.extension(),
        // read each file → Snippet
    }
}
```

Hidden files (`.`-prefixed) and files larger than 64 KiB are skipped. Read errors on individual files are logged but do not fail the whole listing.

### 7.2 `SystemClock`

```rust
pub struct SystemClock;
impl Clock for SystemClock {
    fn now(&self) -> Instant { Instant::now() }
}
```

### 7.3 `Xorshift64` PRNG

Marsaglia xorshift, period 2⁶⁴ − 1.

```rust
pub struct Xorshift64 { state: u64 }

impl Xorshift64 {
    pub fn from_entropy() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0xDEAD_BEEF_CAFE_BABE);
        let pid = std::process::id() as u64;
        let mut state = nanos ^ pid.rotate_left(17);
        if state == 0 { state = 0x9E37_79B9_7F4A_7C15; }
        Self { state }
    }

    pub fn from_seed(seed: u64) -> Self {
        let s = if seed == 0 { 0x9E37_79B9_7F4A_7C15 } else { seed };
        Self { state: s }
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

Reasoning: xorshift64 has uniform distribution good enough for picking 1 of ≤ ~1000 snippets. No external dependency. Deterministic when seeded → enables reproducible tests.

## 8. Presentation Layer

### 8.1 CLI parser

```rust
#[derive(Parser)]
#[command(name = "thypoon", version, about = "TUI typing trainer for code")]
pub struct Cli {
    #[command(flatten)]
    lang: LangFlags,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct LangFlags {
    #[arg(short = 'r', long = "rust")]       rust: bool,
    #[arg(short = 't', long = "typescript")] typescript: bool,
    #[arg(short = 'g', long = "go")]         go: bool,
}

impl Cli { pub fn language(&self) -> Language { /* map flags */ } }
```

### 8.2 TUI layout

```
┌──────────────────────────────────────────────────────────────┐
│  ⏱  47s    ✗ 3 errors    ⚡ 62 WPM (net)    ◎ 97.2%         │  ← 30%
├──────────────────────────────────────────────────────────────┤
│                                                              │
│   fn main() {                                                │
│       █  let x = 42;                                         │  ← rest
│       println!("{}", x);                                     │
│   }                                                          │
│                                                              │
├──────────────────────────────────────────────────────────────┤
│  ESC quit · Enter restart                                    │  ← 1 line
└──────────────────────────────────────────────────────────────┘
```

`Layout::vertical([Constraint::Percentage(30), Constraint::Min(0), Constraint::Length(1)])`.

#### Widgets
- `StatsBarWidget` — reads `Stats`, renders 4 cells with separators.
- `SnippetViewWidget` — char-by-char `Span` styling: green for correct typed chars, red+underline for incorrect, dim gray for pending, reverse-video block for cursor at `typed.len()`.
- `FooterWidget` — static hint line.

### 8.3 Event loop

```rust
pub fn run<R, C, Repo>(
    start_uc: &mut StartSessionUseCase<Repo, R, C>,
    language: Language,
    clock: &C,
) -> Result<(), AppError>
where R: Rng, C: Clock, Repo: CorpusRepository {
    let mut session = start_uc.execute(language)?;
    let mut terminal = init_terminal()?;
    loop {
        let now = clock.now();
        if session.is_finished(now) { /* draw results screen, wait for key */ }
        renderer::draw(&mut terminal, &session, now)?;
        if event::poll(Duration::from_millis(50))? {
            match event::read()? {
                Key(Esc, _) => break,
                Key(Enter, _) if session.is_finished(now)
                    => session = start_uc.execute(language)?,
                Key(Char(c), _) => { process_uc.execute(&mut session, Keystroke::Char(c)); }
                Key(Backspace, _) => { process_uc.execute(&mut session, Keystroke::Backspace); }
                _ => {}
            }
        }
    }
    restore_terminal()?;
    Ok(())
}
```

50 ms poll → ~20 fps, smooth enough for a typing-stats view and low CPU.

## 9. Composition Root (`main.rs`)

```rust
fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let language = cli.language();

    let clock = SystemClock;
    let rng = Xorshift64::from_entropy();
    let repo = FsCorpusRepository::from_env();

    let mut start_uc = StartSessionUseCase::new(
        repo, rng, clock, Duration::from_secs(60),
    );

    presentation::tui::app::run(&mut start_uc, language, &SystemClock)?;
    Ok(())
}
```

Only place where concrete adapters meet abstract use cases.

## 10. Dependencies

```toml
[dependencies]
ratatui   = "0.29"
crossterm = "0.28"
clap      = { version = "4", features = ["derive"] }
```

Explicitly excluded: `rand` (custom Xorshift), `tokio` (sync event loop is enough), `serde` (no JSON / config).

## 11. WPM Formula (reference)

- Standard: 1 word = 5 keystrokes.
- `gross_wpm = (chars_typed / 5) / minutes`
- `net_wpm   = max(0, gross_wpm - errors_per_minute)`
- `errors_per_minute = errors / minutes`
- `accuracy = correct_chars / typed_chars` (or 1.0 if no input).

Net WPM is what the StatsBar shows in large; gross is available for debugging.

## 12. Error Handling

| Error                   | Where                       | User-facing behavior                            |
| ----------------------- | --------------------------- | ----------------------------------------------- |
| `CorpusError::NoDir`    | `FsCorpusRepository`        | Print `corpus/<lang>/ not found`, exit 1.       |
| `CorpusError::Empty`    | `StartSessionUseCase`       | Print `no snippets for <lang>`, exit 1.         |
| `CorpusError::IoError`  | `FsCorpusRepository`        | Log on stderr per file, continue if others OK.  |
| Terminal init failure   | TUI app                     | Restore terminal, print error, exit 1.          |
| Clap parse error        | CLI                         | Clap prints help + exit 2.                      |

The TUI always restores the terminal on exit (RAII guard pattern: `struct TerminalGuard` on `Drop` calls `disable_raw_mode` + `LeaveAlternateScreen`).

## 13. Testing Strategy

### Domain (pure, fast)
- `wpm_calculator`: golden values — `(300 chars, 0 errors, 60s) → 60 net wpm`, etc.
- `accuracy_calculator`: `(0, 0) → 1.0`, `(95, 100) → 0.95`.
- `TypingSession::apply`: correct char advances `correct_chars`, wrong char increments `errors` and `typed_chars`, backspace shrinks `typed` without touching `errors`.
- `is_finished`: time-driven (60 s elapsed) AND snippet-complete-driven.

### Application
- `StartSessionUseCase` with in-memory `CorpusRepository` returning N snippets + seeded `Xorshift64` → asserts deterministic pick.
- Empty repo → `StartSessionError::NoSnippets`.

### Infrastructure
- `Xorshift64`: 10 000 draws into 10 buckets — assert each bucket count is within ±5 % of expected (smoke test of uniformity).
- `FsCorpusRepository`: temp dir with `sample.rs`, `sample.json`, hidden `.foo.rs`, and a `sample.ts` → asserts only `.rs` files are listed for `Language::Rust`.

### Presentation
- Skipped at unit level. Manual run is the test (per system guidance: TUI feature correctness verified by running the app).

## 14. SOLID Mapping

- **S** Single Responsibility: `WpmCalculator` does math; `FsCorpusRepository` does FS; `SnippetViewWidget` does rendering.
- **O** Open/Closed: adding a 4th language = `Language::Kotlin` variant + `corpus/kotlin/` dir + CLI flag. Use cases untouched.
- **L** Liskov: any `Rng` impl works for `StartSessionUseCase` (tested with a deterministic seeded one).
- **I** Interface Segregation: three small ports (`Clock`, `Rng`, `CorpusRepository`) instead of one fat `Services` trait.
- **D** Dependency Inversion: use cases parameterized over traits; concrete adapters injected in `main.rs`.

## 15. Open Questions / Future Work

- Add `-d/--duration` for 15 / 30 / 120 s modes.
- Add results screen with chart of WPM over time (could store ticks in `TypingSession`).
- Persist best-per-language to `~/.local/share/thypoon/scores.json`.
- Configurable theme (`THYPOON_THEME` env or config file).
- Pluggable corpus formats (JSON manifest) — would add a second `CorpusRepository` adapter without touching use cases.
