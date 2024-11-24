/// TODO: Add docs
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Menu(MenuState),
    Playing,
    Paused,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuState {
    Main,
    Settings,
    Credits,
}

impl fmt::Display for MenuState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MenuState::Main => write!(f, "Main Menu"),
            MenuState::Settings => write!(f, "Settings"),
            MenuState::Credits => write!(f, "Credits"),
        }
    }
}
