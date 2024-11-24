use crate::assets::AssetManager;
use crate::consts::{PIXELS_HEIGHT, PIXELS_WIDTH};
use crate::menu::element::{ElementState, MenuAction, MenuElement, MenuItem};
use glam::Vec2;
use std::collections::HashMap;

pub struct Menu {
    items: Vec<MenuItem>,
    selected_item: usize,
}

impl Menu {
    fn new(items: Vec<MenuItem>) -> Self {
        let mut menu = Self {
            items,
            selected_item: 0,
        };
        menu.layout_items();
        menu
    }

    fn layout_items(&mut self) {
        let menu_height = self.items.len() as f32 * 50.0;
        let start_y = (PIXELS_HEIGHT as f32 - menu_height) / 2.0;

        for (i, item) in self.items.iter_mut().enumerate() {
            item.set_position(Vec2::new(
                (PIXELS_WIDTH as f32 - item.dimensions().x) / 2.0,
                start_y + i as f32 * 50.0,
            ));
        }
    }

    fn item_count(&self) -> usize {
        self.items.len()
    }

    fn selected_action(&self) -> MenuAction {
        self.items[self.selected_item].action()
    }

    fn selected_item(&self) -> usize {
        self.selected_item
    }

    fn selected_text(&self) -> Option<&str> {
        self.items.get(self.selected_item).map(|item| item.text())
    }
}

pub struct MenuRenderer {
    menus: HashMap<String, Menu>,
    current_menu: String,
    menu_stack: Vec<String>, // Tracks menu navigation history
}

impl MenuRenderer {
    pub fn new() -> Self {
        let mut menus = HashMap::new();

        // Main Menu
        menus.insert(
            "main".to_string(),
            Menu::new(vec![
                MenuItem::new("Play", MenuAction::StartGame),
                MenuItem::new("Options", MenuAction::OpenSubmenu("options".to_string())),
                MenuItem::new("Graphics", MenuAction::OpenSubmenu("graphics".to_string())),
                MenuItem::new("Sound", MenuAction::OpenSubmenu("sound".to_string())),
                MenuItem::new("Controls", MenuAction::OpenSubmenu("controls".to_string())),
                MenuItem::new("Credits", MenuAction::OpenSubmenu("credits".to_string())),
                MenuItem::new("Quit", MenuAction::OpenSubmenu("quit".to_string())),
            ]),
        );

        // Options Menu
        menus.insert(
            "options".to_string(),
            Menu::new(vec![
                MenuItem::new(
                    "Difficulty: Normal",
                    MenuAction::ToggleSetting("difficulty".to_string()),
                ),
                MenuItem::new(
                    "Fullscreen: Off",
                    MenuAction::ToggleSetting("fullscreen".to_string()),
                ),
                MenuItem::new("Back", MenuAction::BackToParent),
            ]),
        );

        // Graphics Menu
        menus.insert(
            "graphics".to_string(),
            Menu::new(vec![
                MenuItem::new(
                    "Resolution: 1920x1080",
                    MenuAction::OpenSubmenu("resolution".to_string()),
                ),
                MenuItem::new(
                    "Quality: High",
                    MenuAction::ToggleSetting("quality".to_string()),
                ),
                MenuItem::new("VSync: On", MenuAction::ToggleSetting("vsync".to_string())),
                MenuItem::new("Back", MenuAction::BackToParent),
            ]),
        );

        // Sound Menu
        menus.insert(
            "sound".to_string(),
            Menu::new(vec![
                MenuItem::new(
                    "Master Volume: 100%",
                    MenuAction::SetValue("master_volume".to_string(), "100".to_string()),
                ),
                MenuItem::new(
                    "Music Volume: 80%",
                    MenuAction::SetValue("music_volume".to_string(), "80".to_string()),
                ),
                MenuItem::new(
                    "SFX Volume: 90%",
                    MenuAction::SetValue("sfx_volume".to_string(), "90".to_string()),
                ),
                MenuItem::new("Back", MenuAction::BackToParent),
            ]),
        );

        // Controls Menu
        menus.insert(
            "controls".to_string(),
            Menu::new(vec![
                MenuItem::new(
                    "Keyboard Settings",
                    MenuAction::OpenSubmenu("keyboard".to_string()),
                ),
                MenuItem::new(
                    "Gamepad Settings",
                    MenuAction::OpenSubmenu("gamepad".to_string()),
                ),
                MenuItem::new("Back", MenuAction::BackToParent),
            ]),
        );

        // Keyboard Settings Menu
        menus.insert(
            "keyboard".to_string(),
            Menu::new(vec![
                MenuItem::new("Key Bindings", MenuAction::Nothing), // TODO: Implement
                MenuItem::new("Back", MenuAction::BackToParent),
            ]),
        );

        // Gamepad Settings Menu
        menus.insert(
            "gamepad".to_string(),
            Menu::new(vec![
                MenuItem::new("Gamepad Bindings", MenuAction::Nothing), // TODO: Implement
                MenuItem::new("Back", MenuAction::BackToParent),
            ]),
        );

        // Credits Menu
        menus.insert(
            "credits".to_string(),
            Menu::new(vec![
                MenuItem::new("Created by You", MenuAction::Nothing),
                MenuItem::new("Graphics: You", MenuAction::Nothing),
                MenuItem::new("Music: You", MenuAction::Nothing),
                MenuItem::new("Back", MenuAction::BackToParent),
            ]),
        );

        // Quit Confirmation Menu
        menus.insert(
            "quit".to_string(),
            Menu::new(vec![
                MenuItem::new("Are you sure?", MenuAction::Nothing),
                MenuItem::new(
                    "Yes",
                    MenuAction::SetValue("quit".to_string(), "true".to_string()),
                ),
                MenuItem::new("No", MenuAction::BackToParent),
            ]),
        );

        Self {
            menus,
            current_menu: "main".to_string(),
            menu_stack: Vec::new(),
        }
    }

    pub fn render(&mut self, frame: &mut [u8], assets: &AssetManager) -> anyhow::Result<()> {
        let font = assets.get_font();

        // Clear screen with dark background
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[30, 30, 30, 255]);
        }

        if let Some(menu) = self.menus.get_mut(&self.current_menu) {
            for (i, item) in menu.items.iter_mut().enumerate() {
                let _ = item.update(if i == menu.selected_item {
                    ElementState::Focused
                } else {
                    ElementState::Normal
                });
                item.render(frame, PIXELS_WIDTH, PIXELS_HEIGHT, &font);
            }
        }

        Ok(())
    }

    pub fn move_selection(&mut self, delta: isize) {
        if let Some(menu) = self.menus.get_mut(&self.current_menu) {
            menu.selected_item = (menu.selected_item as isize + delta)
                .rem_euclid(menu.item_count() as isize) as usize;
        }
    }

    pub fn handle_input(&mut self) -> MenuAction {
        if let Some(menu) = self.menus.get(&self.current_menu) {
            let action = menu.selected_action();

            match &action {
                MenuAction::OpenSubmenu(submenu) => {
                    self.menu_stack.push(self.current_menu.clone());
                    self.current_menu = submenu.clone();
                }
                MenuAction::BackToParent => {
                    if let Some(parent) = self.menu_stack.pop() {
                        self.current_menu = parent;
                    }
                }
                _ => {}
            }

            action
        } else {
            MenuAction::Nothing
        }
    }

    pub fn current_menu(&self) -> &str {
        &self.current_menu
    }

    pub fn current_selected_text(&self) -> Option<String> {
        self.menus
            .get(&self.current_menu)
            .and_then(|menu| menu.selected_text().map(String::from))
    }
}
