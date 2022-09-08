mod command;

use command::{Args, ArgsErr};
use thiserror::Error;

#[derive(Debug, Error)]
enum AppErr {
    #[error("Args error: `{0}`")]
    ArgsErr(#[from] ArgsErr),
}

fn main() -> Result<(), AppErr> {
    let args = Args::new()?;

    // println!("{:?}", args);
    // let commands = vec![
    //     "ping -n 1000 example.com",
    //     "ping -n 1000 news.ycombinator.com",
    // ];

    // let mut c = Command::new("ping")
    //     .args(["-n", "1000", "example.com"])
    //     .stdin(Stdio::piped())
    //     .stderr(Stdio::piped())
    //     .stdout(Stdio::piped())
    //     .spawn()?;

    // let stdout = c.stdout.take().unwrap();
    // let buf_reader = BufReader::new(stdout);

    // buf_reader.lines().for_each(|line| {
    //     println!("[1] {}", line.unwrap());
    // });

    Ok(())
}
