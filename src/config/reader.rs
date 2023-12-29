use serde_json::Result;

use crate::theme::ThemeList;

pub fn parse_themes() -> Result<ThemeList> {
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
        "gruvbox_light": {
            // similar structure
        }
    }
}
"#;
    let p: ThemeList = serde_json::from_str(data)?;

    Ok(p)
}