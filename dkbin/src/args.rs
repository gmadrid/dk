mod common;

use std::path::PathBuf;
use structopt::StructOpt;

pub use common::{
    chart_in, chart_out, chart_path_in, chart_path_out, pipe_chart, ChartFileOut, Pipeable,
};

/// The command line arguments for all of the subcommands.
/// To ensure common argument names in the subcommands, use the above structs
/// and "flatten" them.
pub mod commandargs {
    use super::*;
    use crate::args::common::ChartFileIn;

    #[derive(Debug, StructOpt)]
    pub struct ImageConvertArgs {
        #[structopt(long, short, help = "height in stitches of the final pattern")]
        pub height: Option<u16>,

        #[structopt(long, short, help = "width in stitches of the final pattern")]
        pub width: Option<u16>,

        #[structopt(flatten)]
        pub outfile: ChartFileOut,

        #[structopt(parse(from_os_str))]
        pub image_name: PathBuf,
    }

    #[derive(Debug, StructOpt)]
    pub struct KnitchartArgs {
        #[structopt(flatten)]
        pub infile: ChartFileIn,

        #[structopt(parse(from_os_str))]
        pub image_name: Option<PathBuf>,
    }

    #[derive(Debug, StructOpt)]
    pub struct LeftArgs {
        #[structopt(flatten)]
        pub pipe: common::Pipeable,
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
        pub pipe: common::Pipeable,
    }

    #[derive(Debug, StructOpt)]
    pub struct ReflectArgs {
        #[structopt(long)]
        pub right_to_left: bool,

        #[structopt(flatten)]
        pub pipe: common::Pipeable,
    }

    #[derive(Debug, StructOpt)]
    pub struct RightArgs {
        #[structopt(flatten)]
        pub pipe: common::Pipeable,
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
        pub pipe: common::Pipeable,
    }

    #[derive(Debug, StructOpt)]
    pub struct ZipArgs {
        #[structopt(short, long)]
        pub out_file_name: Option<PathBuf>,

        pub left_file_name: PathBuf,
        pub right_file_name: PathBuf,
    }
}
