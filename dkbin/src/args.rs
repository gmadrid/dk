use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Pipeable {
    #[structopt(long, short, parse(from_os_str))]
    pub out_file_name: Option<PathBuf>,

    #[structopt(parse(from_os_str))]
    pub in_file_name: Option<PathBuf>,
}

////////
pub mod commandargs {
    use super::*;

    #[derive(Debug, StructOpt)]
    pub struct ImageConvertArgs {
        #[structopt(long, short, help = "height in stitches of the final pattern")]
        pub height: Option<u16>,

        #[structopt(long, short, help = "width in stitches of the final pattern")]
        pub width: Option<u16>,

        #[structopt(long, short, parse(from_os_str))]
        pub out_file_name: Option<PathBuf>,

        #[structopt(parse(from_os_str))]
        pub image_name: PathBuf,
    }

    #[derive(Debug, StructOpt)]
    pub struct KnitchartArgs {
        #[structopt(long, short, parse(from_os_str))]
        pub out_file_name: Option<PathBuf>,

        #[structopt(parse(from_os_str))]
        pub in_file_name: Option<PathBuf>,
    }

    #[derive(Debug, StructOpt)]
    pub struct LeftArgs {
        #[structopt(flatten)]
        pub pipe: Pipeable,
    }

    #[derive(Debug, StructOpt)]
    pub struct MergeArgs {
        #[structopt(parse(from_os_str))]
        pub left: PathBuf,

        #[structopt(parse(from_os_str))]
        pub right: PathBuf,

        #[structopt(long, short, parse(from_os_str))]
        pub out_file_name: Option<PathBuf>,
    }

    #[derive(Debug, StructOpt)]
    pub struct PadArgs {
        #[structopt(flatten)]
        pub pipe: Pipeable,
    }

    #[derive(Debug, StructOpt)]
    pub struct ReflectArgs {
        #[structopt(long)]
        pub right_to_left: bool,

        #[structopt(flatten)]
        pub pipe: Pipeable,
    }

    #[derive(Debug, StructOpt)]
    pub struct RepeatArgs {
        #[structopt(long, short, parse(from_os_str))]
        pub in_file_name: Option<PathBuf>,

        #[structopt(long, short, parse(from_os_str))]
        pub out_file_name: Option<PathBuf>,

        #[structopt(long, short, default_value = "1")]
        pub horiz: u8,

        #[structopt(long, short, default_value("1"))]
        pub vert: u8,
    }

    #[derive(Debug, StructOpt)]
    pub struct RightArgs {
        #[structopt(flatten)]
        pub pipe: Pipeable,
    }

    #[derive(Debug, StructOpt)]
    pub struct SplitArgs {
        #[structopt(parse(from_os_str))]
        pub in_file_name: Option<PathBuf>,

        #[structopt(long = "output_stem", short = "o", parse(from_os_str))]
        pub out_file_stem: Option<PathBuf>,
    }

    #[derive(Debug, StructOpt)]
    pub struct TrimArgs {
        #[structopt(flatten)]
        pub pipe: Pipeable,
    }

    #[derive(Debug, StructOpt)]
    pub struct ZipArgs {
        #[structopt(short, long)]
        pub out_file_name: Option<PathBuf>,

        pub left_file_name: PathBuf,
        pub right_file_name: PathBuf,
    }
}
