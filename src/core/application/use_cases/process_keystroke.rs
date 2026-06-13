use crate::core::domain::{TypingSession, Keystroke, KeystrokeOutcome};

pub struct ProcessKeystrokeUseCase;

impl ProcessKeystrokeUseCase {
    pub fn new() -> Self { Self }

    pub fn execute(&self, session: &mut TypingSession, key: Keystroke) -> KeystrokeOutcome {
        session.apply(key)
    }
}

impl Default for ProcessKeystrokeUseCase {
    fn default() -> Self { Self::new() }
}
