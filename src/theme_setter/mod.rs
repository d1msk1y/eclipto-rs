// Declare module that will set gruvbox theme
pub mod theme_setter {
    use std::fs;
    use std::process::Command;

    use crate::theme::ThemeParams;

    pub fn apply(params: &ThemeParams) {
        match params.kitty {
            Some(ref path) => {
                let kitty_theme = fs::read_to_string(path).unwrap();
                fs::write("/home/dmytro/.config/kitty/current-theme.conf", kitty_theme).unwrap();
            }
            None => {
                println!("No kitty theme, doing nothing");
            }
        }

        match params.wallpaper {
            Some(ref path) => {
                set_wallpaper(path);
            }
            None => {
                println!("No wallpaper, doing nothing");
            }
        }

        match params.polybar {
            Some(ref theme_name) => {
                fs::write("/home/dmytro/polybar-collection/theme.sh", theme_name).unwrap();
            }
            None => {
                println!("No polybar theme, doing nothing");
            }
        }

        match params.rofi {
            Some(ref path) => {
                fs::write("/home/dmytro/.config/rofi/config.rasi", path).unwrap();
            }
            None => {
                println!("No rofi theme, doing nothing");
            }
        }

        // Restarting i3
        Command::new("i3-msg")
            .arg("restart")
            .status()
            .unwrap();

        match params.commands {
            Some(ref commands) => {
                for command in commands {
                    let output = Command::new("bash")
                        .arg("-c")
                        .arg(command)
                        .output()
                        .expect("failed to execute process");
                    println!("Command output: {}", String::from_utf8_lossy(&output.stdout));
                }
            }
            None => {
                println!("No custom commands, doing nothing");
            }
        }
    }

    fn set_wallpaper(path: &str) {
        let monitors = Command::new("bash")
            .arg("-c")
            .arg("xrandr | grep ' connected' | cut -d ' ' -f1")
            .output()
            .expect("failed to execute process");

        let monitors = String::from_utf8_lossy(&monitors.stdout);
        let monitors: Vec<&str> = monitors.split_whitespace().collect();
        for monitor in monitors {
            Command::new("nitrogen")
                .arg("--set-zoom-fill")
                .arg(path)
                .arg("--head")
                .arg(monitor)
                .output()
                .expect("failed to execute process");
        }
    }
}