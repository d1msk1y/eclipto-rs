use crate::theme::ThemeList;

pub fn parse_themes() -> Result<ThemeList, serde_json::Error> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
    "themes": {
        "gruvbox": {
            "kitty": "/path/to/kitty/theme",
            "wallpaper": "/path/to/wallpaper",
            "rofi": "/path/to/rofi/theme",
            "polybar": "theme_name"
        },
        "gruvbox-light": {
            "kitty": "/path/to/kitty/theme",
            "wallpaper": "/path/to/wallpaper",
            "rofi": "/path/to/rofi/theme",
            "polybar": "theme_name"
        }
    }
}
"#;
    let result: ThemeList = serde_json::from_str(data)?;

    Ok(result)
}