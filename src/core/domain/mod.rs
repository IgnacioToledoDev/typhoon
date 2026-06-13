// Pure domain. No deps outside `std`.

pub mod language;
pub mod snippet;

pub use language::Language;
pub use snippet::{Snippet, SnippetError};
