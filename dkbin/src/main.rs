use anyhow::Error;
use fehler::throws;
use subcommands::UboatCaptain;

mod args;
mod subcommands;

#[throws]
fn main() {
    subcommands::SubCommands::dispatch()?;
}
