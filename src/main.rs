use crate::config::reader::parse_themes;

mod theme_setter;
mod cli;
mod config;
mod theme;

fn main()
{
    let theme_name: String = cli::cli::command();
    println!("Theme: {}", theme_name);

    let theme_list = parse_themes().unwrap();
    println!("Themes: {:?}", theme_list.themes[&theme_name].clone());
    theme_setter::theme_setter::set_themes(&theme_list.themes[&theme_name].clone());
}