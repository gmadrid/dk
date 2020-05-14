//use anyhow::Error;
//use fehler::throws;
use structopt::StructOpt;
//use dklib;

mod args;
mod subcommands;

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

fn main() -> Result<(), u8> {
    dispatch!(
        ImageConvert --> image_convert,
        Knitchart    --> knitchart,
        Left         --> left,
        Merge        --> merge,
        Pad          --> pad,
        Reflect      --> reflect,
        Right        --> right,
        Split        --> split,
        Trim         --> trim,
        Zip          --> zip
    );
}
