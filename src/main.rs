// main.rs
mod foo;
mod themes;
mod theme_setter;


// use the themes module

fn main() {
    // Using gruvbox module directly if re-exported
    themes::apply_gruvbox_theme();

    // Or if not re-exported, access it through its submodule path

}
