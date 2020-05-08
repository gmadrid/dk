use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "dk", about = "A tool for making double-knitting patterns.")]
pub enum Dk {
    /// Convert an image to a knit chart based on color values.
    ImageConvert {
        #[structopt(flatten)]
        args: ImageConvertArgs,
    },
    Knitchart {
        #[structopt(flatten)]
        args: KnitchartArgs,
    },
    /// Cut a chart in half and output the left side.
    Left {
        #[structopt(flatten)]
        args: LeftArgs,
    },
    /// Cut a chart in half and output the right side.
    Right {
        #[structopt(flatten)]
        args: RightArgs,
    },
    /// Cut a chart in half and output two new charts.
    Split {
        #[structopt(flatten)]
        args: SplitArgs,
    },
    /// Trim all of the blanks and knit stitches off the outside of a chart.
    Trim {
        #[structopt(flatten)]
        args: TrimArgs,
    },
    /// Generate the mirror image of a chart.
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

    #[structopt(long, short, parse(from_os_str))]
    pub out_file_name: Option<PathBuf>,

    #[structopt(parse(from_os_str))]
    pub image_name: PathBuf,
}

#[derive(Debug, StructOpt)]
pub struct LeftArgs {
    #[structopt(long, short, parse(from_os_str))]
    pub out_file_name: Option<PathBuf>,

    #[structopt(parse(from_os_str))]
    pub in_file_name: Option<PathBuf>,
}

#[derive(Debug, StructOpt)]
pub struct RightArgs {
    #[structopt(long, short, parse(from_os_str))]
    pub out_file_name: Option<PathBuf>,

    #[structopt(parse(from_os_str))]
    pub in_file_name: Option<PathBuf>,
}

#[derive(Debug, StructOpt)]
pub struct KnitchartArgs {
    #[structopt(parse(from_os_str))]
    pub filenames: Vec<PathBuf>,
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
    #[structopt(long, short, parse(from_os_str))]
    pub out_file_name: Option<PathBuf>,

    #[structopt(parse(from_os_str))]
    pub in_file_name: Option<PathBuf>,
}

#[derive(Debug, StructOpt)]
pub struct ReflectArgs {
    #[structopt(long)]
    pub right_to_left: bool,

    #[structopt(long, short, parse(from_os_str))]
    pub out_file_name: Option<PathBuf>,

    #[structopt(parse(from_os_str))]
    pub in_file_name: Option<PathBuf>,
}

#[derive(Debug, StructOpt)]
pub struct ZipArgs {
    #[structopt(short, long)]
    pub output: Option<PathBuf>,

    pub left: PathBuf,
    pub right: PathBuf,
}
