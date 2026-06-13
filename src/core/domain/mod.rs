// Pure domain. No deps outside `std`.

pub mod language;
pub mod snippet;
pub mod keystroke;

pub use language::Language;
pub use snippet::{Snippet, SnippetError};
pub use keystroke::{Keystroke, KeystrokeOutcome};
