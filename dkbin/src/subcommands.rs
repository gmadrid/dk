use crate::args::{commandargs, Pipeable};
use anyhow::{anyhow, Error};
use dklib::{
    chart::Chart,
    operations::{
        convert_image_to_chart, merge_charts, pad_chart, reflect_chart, split_chart, trim_chart,
        zip_charts,
    },
    the_thing,
};
use fehler::throws;
use image;
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

/// Makes a pathbuf from `path` but with the `.knit` extension.
/// If `suffix` is provided, then append it to the file stem also.
#[throws]
pub fn make_knit_pathbuf(path: impl AsRef<Path>, suffix: Option<&str>) -> PathBuf {
    let name = path
        .as_ref()
        .file_stem()
        .ok_or_else(|| anyhow!("Pathbuf has no filename part: {}", path.as_ref().display()))?;
    let mut owned = name.to_owned();
    if let Some(suffix) = suffix {
        owned.push(suffix);
    }
    let mut result = PathBuf::from(owned);
    result.set_extension("knit");
    result
}

#[throws]
fn pipe_in<P>(path: &Option<P>) -> Box<dyn Read>
where
    P: AsRef<Path>,
{
    let pipe: Box<dyn Read> = if let Some(p) = path {
        Box::new(std::fs::File::open(p)?)
    } else {
        Box::new(std::io::stdin())
    };
    pipe
}

#[throws]
fn pipe_out(path: &Option<PathBuf>) -> Box<dyn Write> {
    let pipe: Box<dyn Write> = if let Some(p) = path {
        Box::new(std::fs::File::create(p)?)
    } else {
        Box::new(std::io::stdout())
    };
    pipe
}

#[throws]
fn pipe_command(
    in_path: Option<PathBuf>,
    out_path: Option<PathBuf>,
    cmd: impl FnOnce(&mut dyn Read, &mut dyn Write) -> dklib::Result<()>,
) {
    let mut rdr = pipe_in(&in_path)?;
    let mut wtr = pipe_out(&out_path)?;
    cmd(rdr.as_mut(), wtr.as_mut())?;
}

#[throws]
fn pipe_chart(pipe: Pipeable, cmd: impl FnOnce(&Chart) -> dklib::Result<Chart>) {
    pipe_command(pipe.in_file_name, pipe.out_file_name, |rdr, wtr| {
        let chart = Chart::read(&mut BufReader::new(rdr))?;
        let out_chart = cmd(&chart)?;
        out_chart.write(wtr)
    })?;
}

#[throws]
fn chart_in<P>(in_path: &Option<P>) -> Chart
where
    P: AsRef<Path>,
{
    let rdr = pipe_in(in_path)?;
    Chart::read(&mut BufReader::new(rdr))?
}

#[throws]
fn chart_out(out_path: &Option<PathBuf>, chart: &Chart) {
    let mut wtr = pipe_out(out_path)?;
    chart.write(&mut wtr)?;
}

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
    /// Generate the mirror image of a chart.
    Reflect {
        #[structopt(flatten)]
        args: commandargs::ReflectArgs,
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

#[throws]
pub fn left(args: commandargs::LeftArgs) {
    pipe_chart(args.pipe, |chart| Ok(split_chart(chart)?.0))?;
}

#[throws]
pub fn merge(args: commandargs::MergeArgs) {
    let left = chart_in(&Some(&args.left))?;
    let right = chart_in(&Some(&args.right))?;

    let merged = merge_charts(&left, &right)?;
    chart_out(&args.out_file_name, &merged)?;

    // TODO: find a more disciplined way to do this.
    //       it probably means adding color to the charts.
    the_thing("merged.png", &merged)?;
}

#[throws]
pub fn pad(args: commandargs::PadArgs) {
    pipe_chart(args.pipe, pad_chart)?;
}

#[throws]
pub fn reflect(args: commandargs::ReflectArgs) {
    pipe_chart(args.pipe, reflect_chart)?;
}

#[throws]
pub fn right(args: commandargs::RightArgs) {
    pipe_chart(args.pipe, |chart| Ok(split_chart(chart)?.1))?;
}

#[throws]
pub fn split(args: commandargs::SplitArgs) {
    let chart = chart_in(&args.in_file_name)?;

    // If the out stem is provided, use it. Fallback on the input file name.
    // If that's not present (we read from stdin), then just pick "split".
    let stem = args
        .out_file_stem
        .as_ref()
        .or_else(|| args.in_file_name.as_ref())
        .map_or_else(|| PathBuf::from("split"), |p| p.to_owned());

    // TODO: check for existing filenames.

    let left_file_name = make_knit_pathbuf(&stem, Some("-left"))?;
    let right_file_name = make_knit_pathbuf(&stem, Some("-right"))?;

    let (left_chart, right_chart) = split_chart(&chart)?;
    left_chart.write_to_file(left_file_name)?;
    right_chart.write_to_file(right_file_name)?;
}

#[throws]
pub fn trim(args: commandargs::TrimArgs) {
    pipe_chart(args.pipe, trim_chart)?;
}

#[throws]
pub fn zip(args: commandargs::ZipArgs) {
    let left_chart = Chart::read_from_file(args.left_file_name)?;
    let right_chart = Chart::read_from_file(args.right_file_name)?;

    let zipped = zip_charts(&left_chart, &right_chart)?;

    let out_file_name = args
        .out_file_name
        .map(|pb| make_knit_pathbuf(pb, None))
        .transpose()?;
    chart_out(&out_file_name, &zipped)?;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[throws]
    fn test_knit_pathbuf() {
        let no_ext = PathBuf::from("/foo/bar");
        let with_ext = PathBuf::from("/foo/bar.png");
        let no_file_stem = PathBuf::from("/");

        assert_eq!(make_knit_pathbuf(&no_ext, None)?, PathBuf::from("bar.knit"));
        assert_eq!(
            make_knit_pathbuf(&with_ext, None)?,
            PathBuf::from("bar.knit")
        );

        assert_eq!(
            make_knit_pathbuf(no_ext, Some("-foo"))?,
            PathBuf::from("bar-foo.knit")
        );
        assert_eq!(
            make_knit_pathbuf(with_ext, Some("-foo"))?,
            PathBuf::from("bar-foo.knit")
        );

        assert!(make_knit_pathbuf(no_file_stem, None).is_err());
    }
}
