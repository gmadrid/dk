use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "dk", about = "A tool for making double-knitting patterns.")]
pub enum Dk {
    ImageConvert {
        #[structopt(flatten)]
        args: ImageConvertArgs,
    },
    Split {
        #[structopt(flatten)]
        args: SplitArgs,
    }
}

#[derive(Debug, StructOpt)]
pub struct ImageConvertArgs {
    #[structopt(long, short, help = "height in stitches of the final pattern")]
    pub height: Option<u16>,

    #[structopt(long, short, help = "width in stitches of the final pattern")]
    pub width: Option<u16>,

    #[structopt(parse(from_os_str))]
    pub filenames: Vec<PathBuf>,
}

#[derive(Debug, StructOpt)]
pub struct SplitArgs {
    #[structopt(parse(from_os_str))]
    pub filenames: Vec<PathBuf>,
}