pub const DB_FILE: &'static str = "db.json";

#[derive(Clone, Debug)]
pub struct Config {
    pub command: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let command: String = args[1].clone();

        Config { command }
    }
}
