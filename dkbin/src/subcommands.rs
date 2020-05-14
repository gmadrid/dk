use crate::args::commandargs;
use structopt::StructOpt;
use std::path::PathBuf;

mod image_convert;

pub use image_convert::image_convert;

#[derive(Debug, StructOpt)]
#[structopt(name = "dk", about = "A tool for making double-knitting patterns.")]
pub enum SubCommands {
    /// Convert an image to a knit chart based on color values.
    ImageConvert {
        #[structopt(flatten)]
        args: commandargs::ImageConvertArgs,
    },
    /// Outputs an chart image for a chart.
    Knitchart {
        #[structopt(flatten)]
        args: commandargs::KnitchartArgs,
    },
    /// Cut a chart in half and output the left side.
    Left {
        #[structopt(flatten)]
        args: commandargs::LeftArgs,
    },
    /// Merge two charts into a single double-knitting chart.
    Merge {
        #[structopt(flatten)]
        args: commandargs::MergeArgs,
    },
    /// Adds one knit around the entire chart.
    Pad {
        #[structopt(flatten)]
        args: commandargs::PadArgs,
    },
    /// Cut a chart in half and output the right side.
    Right {
        #[structopt(flatten)]
        args: commandargs::RightArgs,
    },
    /// Cut a chart in half and output two new charts.
    Split {
        #[structopt(flatten)]
        args: commandargs::SplitArgs,
    },
    /// Trim all of the blanks and knit stitches off the outside of a chart.
    Trim {
        #[structopt(flatten)]
        args: commandargs::TrimArgs,
    },
    /// Generate the mirror image of a chart.
    Reflect {
        #[structopt(flatten)]
        args: commandargs::ReflectArgs,
    },
    /// Zip two charts together side-by-side.
    Zip {
        #[structopt(flatten)]
        args: commandargs::ZipArgs,
    },
}

