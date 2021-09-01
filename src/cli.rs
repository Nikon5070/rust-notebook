use crate::config::Config;
use std::collections::HashMap;

type CliEvents = HashMap<String, String>;

pub struct Cli {
    pub config: Config,
    // pub events: CliEvents,
}

impl Cli {
    pub fn new(config: Config) -> Cli {
        Cli { config }
    }

    pub fn run(&self, events: CliEvents) {
        loop {
            println!("{:?}", events)
            // let args = ''
            //
            // events[args]
        }
    }
}
