use glam::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ElementState {
    Normal,
    Focused,
    Disabled,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MenuAction {
    Nothing,
    StartGame,
    OpenSubmenu(String),
    BackToParent,
    ToggleSetting(String),
    SetValue(String, String),
}

pub trait MenuElement {
    fn position(&self) -> Vec2;
    fn set_position(&mut self, pos: Vec2);
    fn dimensions(&self) -> Vec2;
    fn update(&mut self, state: ElementState);
    fn render(&self, frame: &mut [u8], width: u32, height: u32);
    fn action(&self) -> MenuAction;
}

#[derive(Debug, Clone, PartialEq)]
pub struct MenuItem {
    position: Vec2,
    dimensions: Vec2,
    text: String,
    state: ElementState,
    action: MenuAction,
}

impl MenuItem {
    pub fn new(text: impl Into<String>, action: MenuAction) -> Self {
        Self {
            position: Vec2::ZERO,
            dimensions: Vec2::new(200.0, 40.0),
            text: text.into(),
            state: ElementState::Normal,
            action,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

impl MenuElement for MenuItem {
    fn position(&self) -> Vec2 {
        self.position
    }

    fn set_position(&mut self, pos: Vec2) {
        self.position = pos;
    }

    fn dimensions(&self) -> Vec2 {
        self.dimensions
    }

    fn update(&mut self, state: ElementState) {
        self.state = state;
    }

    fn render(&self, frame: &mut [u8], width: u32, height: u32) {
        let color = match self.state {
            ElementState::Normal => [100, 100, 100, 255],
            ElementState::Focused => [200, 200, 200, 255],
            ElementState::Disabled => [50, 50, 50, 255],
        };

        let x = self.position.x as u32;
        let y = self.position.y as u32;
        let w = self.dimensions.x as u32;
        let h = self.dimensions.y as u32;

        // Draw menu item background
        for py in y..y + h {
            for px in x..x + w {
                if px < width && py < height {
                    let idx = ((py * width + px) * 4) as usize;
                    frame[idx..idx + 4].copy_from_slice(&color);
                }
            }
        }
    }

    fn action(&self) -> MenuAction {
        self.action.clone()
    }
}
