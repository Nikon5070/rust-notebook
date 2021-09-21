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
        // loop {
        //     println!("{:?}", events)
        //     // let args = ''
        //     //
        //     // events[args]
        // }
    }

    pub fn command(&self, command: &String) {
        println!("{:}", command)
        // loop {
        //     println!("{:?}", events)
        //     // let args = ''
        //     //
        //     // events[args]
        // }
    }

    fn run_command(&self, command: &String) {
        let commands_all: HashMap<&str, Fn() -> None> = HashMap::new();
        commands_all.insert("add", Box::new(|| {
                println!("add");
        }))

    }
}
