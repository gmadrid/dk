use crate::dk::subcommands::chart_out;
use crate::dk::util::make_knit_pathbuf;
use crate::dk::{
    args::ZipArgs,
    chart::{Chart, Stitch},
};
use anyhow::Error;
use fehler::throws;
use std::cmp::max;

#[throws]
pub fn zip(args: ZipArgs) {
    let left_chart = Chart::read_from_file(args.left_file_name)?;
    let right_chart = Chart::read_from_file(args.right_file_name)?;

    let mut zipped = Chart::new(
        left_chart.cols() + right_chart.cols(),
        max(left_chart.rows(), right_chart.rows()),
    );

    for row in zipped.rows() {
        for col in zipped.cols() {
            let stitch = if col < left_chart.cols() {
                // We're in the left chart.
                if row >= left_chart.rows() {
                    Stitch::Knit
                } else {
                    left_chart.stitch(row, col)?
                }
            } else if row >= right_chart.rows() {
                Stitch::Knit
            } else {
                right_chart.stitch(row, col - left_chart.cols())?
            };

            zipped.set_stitch(row, col, stitch)?;
        }
    }

    let out_file_name = args
        .out_file_name
        .map(|pb| make_knit_pathbuf(pb, None))
        .transpose()?;
    chart_out(&out_file_name, &zipped)?;
}
