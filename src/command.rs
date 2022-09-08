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
    #[error("Size mismatch: `{0}`")]
    SizeMismatch(String),
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
                    .short('s')
                    .takes_value(true),
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

        if names.len() != commands.len() {
            Err(CommandErr::SizeMismatch(format!("Size mismatch between names and their processes, found {} name(s) and {} process(es)", names.len(), commands.len())))
        } else {
            Ok(Args { commands, names })
        }
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
    commands: Vec<process::Command>,
    names: Vec<String>,
}

impl Commands {
    pub fn new(args: &Args) -> Result<Commands, CommandErr> {
        let mut commands = Vec::new();
        for command in &args.commands {
            commands.push(Commands::parse_command(command)?);
        }

        Ok(Commands {
            commands,
            names: args.names.clone(),
        })
    }

    pub fn spawn(self) {
        let mut handlers = vec![];
        for (mut command, name) in self.commands.into_iter().zip(self.names) {
            handlers.push(thread::spawn(move || {
                let mut child = command
                    .spawn()
                    .map_err(|e| CommandErr::CommandErr(e))
                    .unwrap_or_else(|e| {
                        eprintln!("[{name}] {} [{:?}]", e, command);
                        process::exit(1);
                    });

                let c = child.stdout.take().unwrap();
                let buf_reader = BufReader::new(c);

                buf_reader.lines().into_iter().for_each(|line| match line {
                    Ok(line) => println!("[{name}] {line}"),
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
