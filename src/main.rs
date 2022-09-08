mod command;

use command::{Args, CommandErr, Commands};
use thiserror::Error;

#[derive(Debug, Error)]
enum AppErr {
    #[error("Args error: `{0}`")]
    CommandErr(#[from] CommandErr),
}

fn main() -> Result<(), AppErr> {
    let args = Args::new()?;
    let commands = Commands::new(&args.commands)?;
    commands.spawn();

    Ok(())
}
