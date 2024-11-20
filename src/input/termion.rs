use termion::event::{Event, Key};

use super::Input;

impl From<&Event> for Input {
    /// Convert termion [`Event`](https://docs.rs/termion/latest/termion/event/enum.Event.html) to [`Input`].
    ///
    /// **Note:** This implementation is only available when the `termion` feature is enabled.
    fn from(value: &Event) -> Self {
        match value {
            Event::Key(key) => match key {
                Key::Char('j') | Key::Down => Input::Down,
                Key::Char('k') | Key::Up => Input::Up,
                Key::Char('h') | Key::Left | Key::Backspace => Input::Left,
                Key::Char('l') | Key::Right | Key::Char('\n') => Input::Right,
                Key::Char(' ') => Input::Space,
                _ => Input::None,
            },
            _ => Input::None,
        }
    }
}
