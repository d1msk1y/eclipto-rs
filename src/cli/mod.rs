pub mod cli {
    pub fn command() -> String {
        let theme = std::env::args().nth(1).expect("no pattern given");
        return theme;
    }
}