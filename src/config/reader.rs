use std::env;
use std::path::Path;

use crate::theme::ThemeList;

pub fn parse_themes() -> Result<ThemeList, serde_json::Error> {
    let config_file_dir: &str = &*(env::var("HOME").unwrap() + "/.config/eclipto/themes.json");
    println!("PATH: {}", config_file_dir);
    if !Path::new(&(config_file_dir.to_string())).exists() {
        println!("No themes found in ~/.config/eclipto/themes.json. Create the file and define your themes there.");
        println!("{}", config_file_dir);
        // std::process::exit(0);
    }
    println!("Ready");
    let contents = std::fs::read_to_string(config_file_dir.to_string())
        .expect("Something went wrong reading the file");

    let result: ThemeList = serde_json::from_str(&*contents)?;

    Ok(result)
}