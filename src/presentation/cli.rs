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
