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
