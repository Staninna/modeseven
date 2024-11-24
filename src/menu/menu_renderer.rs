/// TODO: Add docs
use super::MenuState;
use crate::assets::AssetManager;

pub struct MenuRenderer {
    selected_item: usize,
}

impl MenuRenderer {
    pub fn new() -> Self {
        Self { selected_item: 0 }
    }

    pub fn render(
        &self,
        frame: &mut [u8],
        menu_state: MenuState,
        assets: &AssetManager,
    ) -> anyhow::Result<()> {
        // Clear screen with dark background
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[30, 30, 30, 255]);
        }

        // TODO: Render menu items based on menu_state
        // For now just clearing the screen is enough

        Ok(())
    }

    pub fn move_selection(&mut self, delta: isize, max_items: usize) {
        self.selected_item =
            (self.selected_item as isize + delta).rem_euclid(max_items as isize) as usize;
    }

    pub fn selected_item(&self) -> usize {
        self.selected_item
    }
}
