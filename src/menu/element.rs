use glam::Vec2;
use rusttype::{point, Font, Scale};

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
    fn render(&self, frame: &mut [u8], width: u32, height: u32, font: &Font);
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

    fn render(&self, frame: &mut [u8], width: u32, height: u32, font: &Font) {
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

        // Draw text

        let text = self.text();
        let height = font.v_metrics(Scale::uniform(16.0)).ascent;

        let x = self.position.x - self.dimensions.x / 1.5;
        let y = self.position.y - self.dimensions.y / 1.5;
        // Step 3: Render text
        let glyphs: Vec<_> = font
            .layout(text, Scale::uniform(20.0), point(0.0, height))
            .collect();
        for (i, glyph) in glyphs.iter().enumerate() {
            let x = x + (self.dimensions.x as f32)
                - (glyph.unpositioned().h_metrics().advance_width)
                + (i as f32) * height;
            let y = y + (self.dimensions.y as f32) - height + glyph.position().y;
            let color = match self.state {
                ElementState::Normal => [255, 255, 255, 255],
                ElementState::Focused => [200, 200, 200, 255],
                ElementState::Disabled => [50, 50, 50, 255],
            };
            glyph.draw(|gx, gy, v| {
                // Calculate position relative to the menu item box
                let px = x as u32 + gx;
                let py = y as u32 + gy;
                let idx = ((py * width as u32 + px) * 4) as usize;
                // Blend the color with alpha from the glyph
                let alpha = (v * 255.0) as u8;
                frame[idx..idx + 4].copy_from_slice(&[color[0], color[1], color[2], alpha]);
            });
        }
        // Step 4: Draw border
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
                if px < width && py < height as u32 {
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
