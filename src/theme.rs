use std::clone::Clone;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ThemeParams {
    pub kitty: Option<String>,
    pub wallpaper: Option<String>,
    pub rofi: Option<String>,
    pub polybar: Option<String>,
    pub custom_commands: Option<Vec<String>>,
}

impl Clone for ThemeParams {
    fn clone(&self) -> Self {
        ThemeParams {
            kitty: self.kitty.clone(),
            wallpaper: self.wallpaper.clone(),
            rofi: self.rofi.clone(),
            polybar: self.polybar.clone(),
            custom_commands: self.custom_commands.clone(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ThemeList {
    pub themes: HashMap<String, ThemeParams>,
}

impl Clone for ThemeList {
    fn clone(&self) -> Self {
        ThemeList {
            themes: self.themes.clone(),
        }
    }
}