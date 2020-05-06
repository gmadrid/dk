use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(name = "dk", about = "A tool for making double-knitting patterns.")]
pub enum Dk {
    ImageConvert {
        #[structopt(flatten)]
        args: ImageConvertArgs,
    },
}

#[derive(Debug, StructOpt)]
pub struct ImageConvertArgs{
    #[structopt(long, short, help = "height in stitches of the final pattern")]
    pub height: Option<u16>,

    #[structopt(long, short, help = "width in stitches of the final pattern")]
    pub width: Option<u16>,

    #[structopt(parse(from_os_str))]
    pub files: Vec<PathBuf>,
}
