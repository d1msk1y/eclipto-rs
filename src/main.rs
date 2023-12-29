use crate::config::reader::parse_themes;

mod themes;
mod theme_setter;
mod cli;
mod config;
mod theme;

fn main()
{
    let themes = parse_themes();
    println!("{:?}", themes.unwrap());
    let theme: String = cli::cli::command();
    // execute themes::theme::apply();
    match theme.as_str() {
        // match all the module names in themes directory
        "gruvbox" => themes::gruvbox::apply(),
        "gruvbox_light" => themes::gruvbox_light::apply(),
        _ => println!("No such theme"),
    }
}