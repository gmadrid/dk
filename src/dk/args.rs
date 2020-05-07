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
    },
    Trim {
        #[structopt(flatten)]
        args: TrimArgs,
    },
    Reflect {
        #[structopt(flatten)]
        args: ReflectArgs,
    },
    Zip {
        #[structopt(flatten)]
        args: ZipArgs,
    },
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

#[derive(Debug, StructOpt)]
pub struct TrimArgs {
    #[structopt(parse(from_os_str))]
    pub filenames: Vec<PathBuf>,
}

#[derive(Debug, StructOpt)]
pub struct ReflectArgs {
    #[structopt(long)]
    pub right_to_left: bool,

    #[structopt(parse(from_os_str))]
    pub filenames: Vec<PathBuf>,
}

#[derive(Debug, StructOpt)]
pub struct ZipArgs {
    #[structopt(short, long)]
    pub output: Option<PathBuf>,

    pub left: PathBuf,
    pub right: PathBuf,
}
