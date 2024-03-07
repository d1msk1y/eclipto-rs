// Declare module that will set gruvbox theme
pub mod theme_setter {
    use std::{env, fs};
    use std::process::Command;

    use crate::theme::ThemeParams;

    pub fn apply(params: &ThemeParams) {
        match params.kitty {
            Some(ref path) => {
                let kitty_theme = fs::read_to_string(path).unwrap();
                fs::write(env::var("HOME").unwrap() + "/.config/kitty/current-theme.conf", kitty_theme).unwrap();
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
                fs::write(env::var("HOME").unwrap() + "/polybar-collection/theme.sh", theme_name).unwrap();
            }
            None => {
                println!("No polybar theme, doing nothing");
            }
        }

        match params.rofi {
            Some(ref path) => {
                fs::write(env::var("HOME").unwrap() + "/.config/rofi/config.rasi", path).unwrap();
            }
            None => {
                println!("No rofi theme, doing nothing");
            }
        }

        match params.i3 {
            Some(ref path) => {
                let override_file = fs::read_to_string(path).unwrap();
                fs::write(env::var("HOME").unwrap() + "/.config/i3/override", override_file).unwrap();
            }
            None => {
                println!("No i3 theme, doing nothing");
            }
        }

        match params.gtk3 {
            Some(ref path) => {
                // read file on the path
                let theme = fs::read_to_string(path).unwrap();
                fs::write(env::var("HOME").unwrap() + "/.config/gtk-3.0/settings.ini", theme).unwrap();
            }
            None => {
                println!("No GTK3 Theme, doin nothing")
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
        let output = Command::new("bash")
            .arg("-c")
            .arg("xrandr --listmonitors | grep 'Monitors' | cut -d ' ' -f2")
            .output()
            .expect("failed to execute process");

        println!("Output monitors: {}", String::from_utf8_lossy(&output.stdout));

        let monitor_count = String::from_utf8_lossy(&output.stdout).trim().parse::<usize>().unwrap_or(0);

        for index in 0..monitor_count {
            println!("Setting wallpaper for monitor {} with path: {}", index, path);
            let output = Command::new("nitrogen")
                .arg("--set-zoom-fill")
                .arg(path)
                .arg("--head=".to_owned() + &index.to_string())
                // .arg("0") // Convert index to String
                .output()
                .expect("Failed to execute process");

            println!("Nitrogen output: {}", String::from_utf8_lossy(&output.stdout));
        }
    }
}