use std::env;
use std::path::Path;

use crate::theme::ThemeList;

pub fn parse_themes() -> Result<ThemeList, serde_json::Error> {
    const CONFIG_DIR: &str = &*(env::var("HOME").unwrap() + "/.config/eclipto/");

    if !Path::new(&(CONFIG_DIR.to_string() + "themes.json")).exists() {
        println!("No themes found, creating default themes.json");
        std::fs::write(CONFIG_DIR.to_string() + "themes.json", "define your themes here").expect("Unable to write file");
        // quit the program
        std::process::exit(0);
    }
    let contents = std::fs::read_to_string(CONFIG_DIR.to_string() + "themes.json")
        .expect("Something went wrong reading the file");

    let result: ThemeList = serde_json::from_str(&*contents)?;

    Ok(result)
}