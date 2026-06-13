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

#[test]
fn oversized_files_are_skipped() {
    let root = temp_root();
    let rust_dir = root.join("rust");
    fs::create_dir_all(&rust_dir).unwrap();
    fs::write(rust_dir.join("small.rs"), "fn a() {}").unwrap();
    // 64 KiB is the cap inside FsCorpusRepository — anything strictly larger is skipped.
    let oversized = "x".repeat(64 * 1024 + 1);
    fs::write(rust_dir.join("huge.rs"), &oversized).unwrap();

    let repo = FsCorpusRepository::new(root.clone());
    let snippets = repo.list(Language::Rust).unwrap();

    let names: Vec<String> = snippets
        .iter()
        .map(|s| s.source_path().file_name().unwrap().to_string_lossy().into_owned())
        .collect();
    assert_eq!(names, vec!["small.rs".to_string()]);

    fs::remove_dir_all(root).ok();
}

#[test]
fn empty_directory_returns_empty_list() {
    let root = temp_root();
    let go_dir = root.join("go");
    fs::create_dir_all(&go_dir).unwrap();

    let repo = FsCorpusRepository::new(root.clone());
    let snippets = repo.list(Language::Go).unwrap();
    assert!(snippets.is_empty());

    fs::remove_dir_all(root).ok();
}
