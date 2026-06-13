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
