use clap;
use std::{
    io::{BufRead, BufReader, Stdout},
    process::{self, Stdio},
    thread,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArgsErr {
    #[error("`{0}` is not a valid command")]
    InvalidCommand(String),
    #[error("Found an empty command")]
    EmptyCommand,
    #[error("At least on command should be passed")]
    MissingCommandsArg,
    #[error("Unable to start `{0}` command")]
    CommandErr(#[from] std::io::Error),
    #[error("Command output error: `{0}`")]
    CommandOutputErr(String),
}

/// Run multiple commands concurrently
pub struct Args {
    commands: Vec<process::Command>,
}

impl Args {
    pub fn new() -> Result<(), ArgsErr> {
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

        let commands_str = args
            .get_many::<String>("commands")
            .ok_or(ArgsErr::MissingCommandsArg)?;

        let mut commands = Vec::new();
        for command in commands_str {
            let command = Args::parse_command(command)?;
            commands.push(command);
        }

        Ok(())
    }

    fn parse_command(command: &str) -> Result<process::Command, ArgsErr> {
        if command.trim().is_empty() {
            return Err(ArgsErr::EmptyCommand);
        }

        let args = command.split_whitespace().collect::<Vec<_>>();
        let program = args.get(0).ok_or(ArgsErr::InvalidCommand(command.into()))?;

        let mut command = process::Command::new(*program);
        command.args(&args[1..]);
        command.stdout(Stdio::piped());

        Ok(command)
    }

    pub fn spawn(self) {
        let mut handlers = vec![];
        for mut command in self.commands {
            handlers.push(thread::spawn(move || {
                let mut child = command
                    .spawn()
                    .map_err(|e| ArgsErr::CommandErr(e))
                    .expect("Unable to start this command");

                let c = child.stdout.take().unwrap();
                let buf_reader = BufReader::new(c);

                buf_reader.lines().into_iter().for_each(|line| match line {
                    Ok(line) => println!("{line}"),
                    Err(e) => eprintln!("{}", ArgsErr::CommandOutputErr(e.to_string())),
                })
            }));
        }

        for handler in handlers {
            handler.join().unwrap();
        }
    }
}
