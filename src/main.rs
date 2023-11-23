mod themes;
mod theme_setter;
mod cli;

fn main()
{
    let theme: String = cli::cli::command();
    // execute themes::theme::apply();
    match theme.as_str() {
        // match all the module names in themes directory
        "gruvbox" => themes::gruvbox::apply(),
        "gruvbox_light" => themes::gruvbox_light::apply(),
        _ => println!("No such theme"),
    }
}
