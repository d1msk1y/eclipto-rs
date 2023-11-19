// Declare module that will set gruvbox theme

pub struct ThemeParams {
    pub kitty: String,
    pub wallpaper: String,
    pub rofi: String,
    pub polybar: String,
}

pub mod theme_setter {
    // create theme params struct to use it in set_themes function

    use std::fs;
    use std::process::Command;
    use crate::theme_setter::ThemeParams;

    pub fn set_themes(params: ThemeParams) {
        fs::write("/home/dmytro/polybar-collection/theme.sh", params.polybar).unwrap();

        let kitty_theme = fs::read_to_string(params.kitty).unwrap();
        fs::write("/home/dmytro/.config/kitty/current-theme.conf", kitty_theme).unwrap();

        // Writing to another file
        fs::write("/home/dmytro/.config/rofi/config.rasi", params.rofi).unwrap();

        // Function to set wallpaper
        set_wallpaper(params.wallpaper.as_str());

        // Restarting i3
        Command::new("i3-msg")
            .arg("restart")
            .status()
            .unwrap();
    }

    pub fn set_wallpaper(wallpaper: &str) {
        let monitors_output = Command::new("xrandr")
            .arg("--listmonitors")
            .output()
            .unwrap();

        let monitors_str = String::from_utf8_lossy(&monitors_output.stdout);
        let monitors = monitors_str.split_whitespace();
    }
}