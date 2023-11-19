use crate::theme_setter;
use crate::theme_setter::ThemeParams;

pub fn apply_gruvbox_theme() {
    let params = ThemeParams {
        kitty: String::from("/home/dmytro/.config/kitty/themes/Gruvbox Dark Soft.conf"),
        wallpaper: String::from("/home/dmytro/documents/wallpapers/Gruvbox/minimalistic/ALLqk82.png"),
        rofi: String::from(""@theme "/usr/share/rofi/themes/gruvbox-dark-soft.rasi"""),
        polybar: String::from("gruvbox"),
    };

    theme_setter::theme_setter::set_themes(params);
}
