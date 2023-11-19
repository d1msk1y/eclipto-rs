// themes/mod.rs

// Declare the submodule. This tells Rust to look for a file named `gruvbox.rs` in the `themes` directory.
pub mod gruvbox;

// Future theme modules can be declared in the same way:
// pub mod some_other_theme;

// Re-export the submodule if desired. This line is optional and depends on how you want to structure your access patterns.
pub use gruvbox::apply_gruvbox_theme;
