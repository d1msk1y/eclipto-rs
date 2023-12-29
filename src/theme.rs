pub struct ThemeParams {
    pub kitty: Option<String>,
    pub wallpaper: Option<String>,
    pub rofi: Option<String>,
    pub polybar: Option<String>,
}

#[derive(Debug)]
pub struct ThemeList {
    pub themes: Vec<ThemeParams>,
}