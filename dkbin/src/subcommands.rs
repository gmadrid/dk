use crate::args::commandargs;
use anyhow::Error;
use fehler::throws;
use image;
use std::path::PathBuf;
use structopt::StructOpt;
use dklib::operations::convert_image_to_chart;

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

#[throws]
pub fn image_convert(args: commandargs::ImageConvertArgs) {
    let original_image = image::open(args.image_name)?;

    let chart = convert_image_to_chart(&original_image, args.height, args.width)?;

    let out_file_name = args
        .out_file_name
        .map(|pb| make_knit_pathbuf(pb, None))
        .transpose()?;
    chart_out(&out_file_name, &chart)?;
}

#[throws]
pub fn knitchart(args: commandargs::KnitchartArgs) {
    let chart = chart_in(&args.in_file_name)?;

    // TODO: use infilename if available and not provided.
    let mut out_file = args.out_file_name.unwrap_or_else(|| "chart.png".into());
    out_file.set_extension("png");
    the_thing(&out_file, &chart)?;
}
