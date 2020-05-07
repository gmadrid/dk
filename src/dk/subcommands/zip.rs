use crate::dk::args::ZipArgs;
use crate::dk::chart::{Chart, Stitch};
use anyhow::Error;
use fehler::throws;
use std::path::PathBuf;
use std::cmp::max;

#[throws]
pub fn zip(args: ZipArgs) {
    let left_chart = Chart::read_from_file(args.left)?;
    let right_chart = Chart::read_from_file(args.right)?;

    let mut zipped = Chart::new(
        left_chart.cols() + right_chart.cols(),
        max(left_chart.rows(), right_chart.rows()),
    );

    for row in 0..zipped.rows() {
        for col in 0..zipped.cols() {
            let stitch = if col < left_chart.cols() {
                // We're in the left chart.
                if row >= left_chart.rows() {
                    Stitch::Knit
                } else {
                    left_chart.stitch(row, col)?
                }
            } else {
                if row >= right_chart.rows() {
                    Stitch::Knit
                } else {
                    right_chart.stitch(row, col - left_chart.cols())?
                }
            };

            zipped.set_stitch(row, col, stitch)?;
        }
    }

    zipped.write_to_file("zipped.knit")?;
}
