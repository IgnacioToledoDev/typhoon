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
