use std::{env, fs, process::Command};

struct ThemeParams {
    kitty: String,
    wallpaper: String,
    rofi: String,
    polybar: String,
}

// create theme params struct to use it in set_themes function

fn set_themes(params: ThemeParams) {
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

fn set_wallpaper(wallpaper: &str) {
    let monitors_output = Command::new("xrandr")
        .arg("--listmonitors")
        .output()
        .unwrap();

    let monitors_str = String::from_utf8_lossy(&monitors_output.stdout);
    let monitors = monitors_str.split_whitespace();


    Command::new("nitrogen")
        .args(&["--set-zoom-fill", wallpaper])
        .status()
        .unwrap();

}

fn main() {
    println!("Hello World");
    let gruvbox = ThemeParams {
        kitty: "/home/dmytro/.config/kitty/themes/Gruvbox Dark Soft.conf".to_string(),
        wallpaper: "/home/dmytro/documents/wallpapers/Gruvbox/minimalistic/ALLqk82.png".to_string(),
        rofi: r#"@theme "/usr/share/rofi/themes/gruvbox-dark-soft.rasi""#.to_string(),
        polybar: "gruvbox".to_string(),
    };

    set_themes(gruvbox);

}
