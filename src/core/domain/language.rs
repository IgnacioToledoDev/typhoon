#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Language {
    Rust,
    TypeScript,
    Go,
}

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
