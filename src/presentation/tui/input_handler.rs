use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crate::core::domain::Keystroke;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputCommand {
    Quit,
    Restart,
    Apply(Keystroke),
    Ignore,
}

pub fn map_key_event(event: KeyEvent, finished: bool) -> InputCommand {
    if event.kind != KeyEventKind::Press {
        return InputCommand::Ignore;
    }
    if event.modifiers.contains(KeyModifiers::CONTROL) && event.code == KeyCode::Char('c') {
        return InputCommand::Quit;
    }
    match event.code {
        KeyCode::Esc => InputCommand::Quit,
        KeyCode::Enter if finished => InputCommand::Restart,
        KeyCode::Enter => InputCommand::Apply(Keystroke::Char('\n')),
        KeyCode::Tab => InputCommand::Apply(Keystroke::Char('\t')),
        KeyCode::Backspace => InputCommand::Apply(Keystroke::Backspace),
        KeyCode::Char(c) => InputCommand::Apply(Keystroke::Char(c)),
        _ => InputCommand::Ignore,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }
    }

    #[test]
    fn esc_quits() {
        assert_eq!(map_key_event(key(KeyCode::Esc), false), InputCommand::Quit);
    }

    #[test]
    fn enter_restarts_when_finished() {
        assert_eq!(map_key_event(key(KeyCode::Enter), true), InputCommand::Restart);
    }

    #[test]
    fn enter_types_newline_when_running() {
        assert_eq!(
            map_key_event(key(KeyCode::Enter), false),
            InputCommand::Apply(Keystroke::Char('\n'))
        );
    }

    #[test]
    fn char_is_applied() {
        assert_eq!(
            map_key_event(key(KeyCode::Char('a')), false),
            InputCommand::Apply(Keystroke::Char('a'))
        );
    }

    #[test]
    fn backspace_is_applied() {
        assert_eq!(
            map_key_event(key(KeyCode::Backspace), false),
            InputCommand::Apply(Keystroke::Backspace)
        );
    }

    #[test]
    fn ctrl_c_quits() {
        let event = KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        };
        assert_eq!(map_key_event(event, false), InputCommand::Quit);
    }
}
