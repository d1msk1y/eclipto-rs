use crate::theme::ThemeParams;
use crate::theme_setter;

pub fn apply() {
    let params = ThemeParams {
        kitty: Option::from(String::from("/home/dmytro/.config/kitty/themes/Gruvbox Dark Soft.conf")),
        wallpaper: Option::from(String::from("/home/dmytro/documents/wallpapers/Gruvbox/minimalistic/ALLqk82.png")),
        rofi: Option::from(String::from("@theme \"/usr/share/rofi/themes/gruvbox-dark-soft.rasi\"")),
        polybar: Option::from(String::from("gruvbox")),
    };

    theme_setter::theme_setter::set_themes(params);
}
