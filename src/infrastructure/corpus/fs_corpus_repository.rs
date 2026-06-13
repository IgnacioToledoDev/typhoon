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
        out.sort_by(|a, b| a.source_path().cmp(b.source_path()));
        Ok(out)
    }
}
