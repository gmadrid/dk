use crate::dk::args::{LeftArgs, RightArgs, SplitArgs};
use crate::dk::chart::Chart;
use crate::dk::subcommands::{chart_in, pipe_chart};
use crate::dk::util::make_knit_pathbuf;
use anyhow::{anyhow, Error};
use fehler::throws;
use std::path::{Path, PathBuf};

#[throws]
pub fn left(args: LeftArgs) {
    pipe_chart(args.in_file_name, args.out_file_name, |chart| {
        Ok(split_chart(chart)?.0)
    })?;
}

#[throws]
pub fn right(args: RightArgs) {
    pipe_chart(args.in_file_name, args.out_file_name, |chart| {
        Ok(split_chart(chart)?.1)
    })?;
}

#[throws]
pub fn split(args: SplitArgs) {
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
pub fn split_chart(chart: &Chart) -> (Chart, Chart) {
    // TODO: deal with odd widths.
    let split_point = chart.cols() / 2;

    // TODO: deal with charts with variable number of columns per row.
    //       perhaps pad the rows after reading in the file?
    let mut left_chart = Chart::new(split_point, chart.rows());
    let mut right_chart = Chart::new(chart.cols() - split_point, chart.rows());

    for row in 0..chart.rows() {
        for col in 0..split_point {
            left_chart.set_stitch(row, col, chart.stitch(row, col)?)?;
        }

        for col in split_point..chart.cols() {
            right_chart.set_stitch(row, col - split_point, chart.stitch(row, col)?)?;
        }
    }

    (left_chart, right_chart)
}
