use anyhow::Error;
use fehler::throws;
use structopt::StructOpt;

mod args;
mod subcommands;

/// A macro to avoid repetitive code to dispatch for all of the subcommands.
/// It will complain at compile time if any subcommands are not included.
macro_rules! dispatch {
    ($($command:ident --> $proc:ident),*) => {
        let subcommand = subcommands::SubCommands::from_args();
        match subcommand {
            $(
            subcommands::SubCommands::$command { args } => { subcommands::$proc(args)?; }
            )*
        }
    }
}

#[throws]
fn main() {
    // Dispatch to all of the subcommands.
    dispatch!(
        ImageConvert --> image_convert,
        Knitchart    --> knitchart,
        Left         --> left,
        Merge        --> merge,
        Pad          --> pad,
        Reflect      --> reflect,
        Repeat       --> repeat,
        Right        --> right,
        Split        --> split,
        Stamp        --> stamp,
        Trim         --> trim,
        Zip          --> zip
    );
}
