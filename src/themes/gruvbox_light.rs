use crate::theme_setter;
use crate::theme_setter::ThemeParams;

pub fn apply() {
    let params = ThemeParams {
        kitty: String::from("/home/dmytro/.config/kitty/themes/Gruvbox Material Light Medium.conf"),
        wallpaper: String::from("/home/dmytro/documents/wallpapers/Gruvbox/light/cyber-girl-light.png"),
        rofi: String::from("@theme \"/usr/share/rofi/themes/gruvbox-light-hard.rasi\""),
        polybar: String::from("gruvbox-light"),
    };

    theme_setter::theme_setter::set_themes(params);
}
