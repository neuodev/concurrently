use clap;
use std::process;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArgsErr {
    #[error("Invalid command")]
    InvalidCommand(String),
    #[error("Found an empty command")]
    EmptyCommand,
}

/// Run multiple commands concurrently
pub struct Args {
    commands: Vec<String>,
}

impl Args {
    pub fn new() {
        let args = clap::Command::new("concurrently")
            .author("Ahmed Ibrahim")
            .version("1.0.0")
            .about("Run multiple commands concurrently")
            .arg(
                clap::Arg::new("commands")
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

    fn parse_command(command: &str) -> Result<(String, Vec<String>), ArgsErr> {
        if command.trim().is_empty() {
            return Err(ArgsErr::EmptyCommand);
        }

        let args = command.split_whitespace().collect::<Vec<_>>();
        let program = args.get(0).ok_or(ArgsErr::InvalidCommand(command.into()))?;
        Ok((
            program.to_string(),
            args[1..].iter().map(|a| a.to_string()).collect::<Vec<_>>(),
        ))
    }
}

pub struct CommandBuilder;
