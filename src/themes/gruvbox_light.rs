use crate::theme::ThemeParams;
use crate::theme_setter;

pub fn apply() {
    let params = ThemeParams {
        kitty: Option::from(String::from("/home/dmytro/.config/kitty/themes/Gruvbox Material Light Medium.conf")),
        wallpaper: Option::from(String::from("/home/dmytro/documents/wallpapers/Gruvbox/light/cyber-girl-light.png")),
        rofi: Option::from(String::from("@theme \"/usr/share/rofi/themes/gruvbox-light-hard.rasi\"")),
        polybar: Option::from(String::from("gruvbox-light")),
    };

    theme_setter::theme_setter::set_themes(&params);
}
