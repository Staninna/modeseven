/// TODO: Add docs
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Menu(MenuState),
    Playing,
    Paused,
}

impl GameState {
    pub fn is_playing(&self) -> bool {
        matches!(self, GameState::Playing)
    }

    pub fn is_paused(&self) -> bool {
        matches!(self, GameState::Paused)
    }

    pub fn is_menu(&self) -> bool {
        matches!(self, GameState::Menu(_))
    }

    pub const fn main() -> Self {
        Self::Menu(MenuState::Main)
    }
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
