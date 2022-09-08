use clap::{Arg, Command};

/// Run multiple commands concurrently
pub struct Args {
    commands: Vec<String>,
}

impl Args {
    pub fn new() {
        let args = Command::new("concurrently")
            .author("Ahmed Ibrahim")
            .version("1.0.0")
            .about("Run multiple commands concurrently")
            .arg(
                Arg::new("commands")
                    .help("Set multiple commands to concurrently")
                    .multiple_values(true)
                    .required(true),
            )
            .get_matches();

        match args.get_many::<String>("commands") {
            Some(m) => {
                println!("{:?}", m.collect::<Vec<_>>())
            }
            None => {
                println!("Not found")
            }
        }
    }
}

pub struct CommandBuilder;
