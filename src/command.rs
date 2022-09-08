use clap;
use std::{
    io::{BufRead, BufReader},
    process::{self, Stdio},
    thread,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandErr {
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

pub struct Args {
    pub commands: Vec<String>,
    pub names: Vec<String>,
}

impl Args {
    pub fn new() -> Result<Args, CommandErr> {
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
            .arg(
                clap::Arg::new("names")
                    .help("A comma separated values represent a name fore each running process")
                    .required(false),
            )
            .get_matches();

        let commands = args
            .get_many::<String>("commands")
            .ok_or(CommandErr::MissingCommandsArg)?
            .map(|a| a.to_string())
            .collect::<Vec<_>>();

        let names = match args.get_one::<String>("names") {
            Some(names) => names.split(",").map(|n| n.to_string()).collect::<Vec<_>>(),
            None => Args::get_programmes(&commands),
        };

        Ok(Args { commands, names })
    }

    fn get_programmes(commands: &Vec<String>) -> Vec<String> {
        commands
            .iter()
            .enumerate()
            .map(|(idx, command)| {
                if command.trim().is_empty() {
                    idx.to_string()
                } else {
                    match command.split_whitespace().next() {
                        Some(pro) => pro.into(),
                        None => idx.to_string(),
                    }
                }
            })
            .collect::<Vec<String>>()
    }
}

pub struct Commands {
    pub commands: Vec<process::Command>,
}

impl Commands {
    pub fn new<T>(raw_commands: &Vec<T>) -> Result<Commands, CommandErr>
    where
        T: ToString,
    {
        let mut commands = Vec::new();
        for command in raw_commands {
            commands.push(Commands::parse_command(&command.to_string())?);
        }

        Ok(Commands { commands })
    }

    pub fn spawn(self) {
        let mut handlers = vec![];
        for (idx, mut command) in self.commands.into_iter().enumerate() {
            handlers.push(thread::spawn(move || {
                let mut child = command
                    .spawn()
                    .map_err(|e| CommandErr::CommandErr(e))
                    .unwrap_or_else(|e| {
                        eprintln!("[{idx}] {} [{:?}]", e, command);
                        process::exit(1);
                    });

                let c = child.stdout.take().unwrap();
                let buf_reader = BufReader::new(c);

                buf_reader.lines().into_iter().for_each(|line| match line {
                    Ok(line) => println!("[{idx}] {line}"),
                    Err(e) => eprintln!("{}", CommandErr::CommandOutputErr(e.to_string())),
                })
            }));
        }

        for handler in handlers {
            handler.join().unwrap();
        }
    }

    fn parse_command(command: &str) -> Result<process::Command, CommandErr> {
        if command.trim().is_empty() {
            return Err(CommandErr::EmptyCommand);
        }

        let args = command.split_whitespace().collect::<Vec<_>>();
        let program = args
            .get(0)
            .ok_or(CommandErr::InvalidCommand(command.into()))?;

        let mut command = process::Command::new(*program);
        command.args(&args[1..]);
        command.stdout(Stdio::piped());

        Ok(command)
    }
}
