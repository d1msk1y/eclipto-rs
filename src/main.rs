use crate::config::reader::parse_themes;

mod themes;
mod theme_setter;
mod cli;
mod config;
mod theme;

fn main()
{
    let theme: String = cli::cli::command();
    println!("Theme: {}", theme);

    let themes = parse_themes().unwrap();
    println!("Themes: {:?}", themes.themes[&theme].clone());
    theme_setter::theme_setter::set_themes(&themes.themes[&theme].clone());
}