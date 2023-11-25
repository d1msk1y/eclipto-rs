// Declare module that will set gruvbox theme

pub struct ThemeParams {
    pub kitty: String,
    pub wallpaper: String,
    pub rofi: String,
    pub polybar: String,
}

pub mod theme_setter {
    use std::fs;
    use std::process::Command;

    use crate::theme_setter::ThemeParams;

    pub fn set_themes(params: ThemeParams) {
        fs::write("/home/dmytro/polybar-collection/theme.sh", params.polybar).unwrap();

        let kitty_theme = fs::read_to_string(params.kitty).unwrap();
        fs::write("/home/dmytro/.config/kitty/current-theme.conf", kitty_theme).unwrap();

        fs::write("/home/dmytro/.config/rofi/config.rasi", params.rofi).unwrap();

        set_wallpaper(params.wallpaper.as_str());

        // Restarting i3
        Command::new("i3-msg")
            .arg("restart")
            .status()
            .unwrap();
    }

    fn set_wallpaper(path: &str) {
        //set nitrogen wallpaper
        let output = Command::new("nitrogen")
            .arg("--set-zoom-fill")
            .arg(path)
            .output()
            .expect("failed to execute process");
    }
}