use crate::dk::thing::the_thing;
use crate::dk::{
    args::MergeArgs,
    chart::{Chart, Stitch},
    subcommands::{chart_in, chart_out},
};
use anyhow::{anyhow, Error};
use css_color_parser::Color;
use fehler::{throw, throws};
use std::str::FromStr;

#[throws]
pub fn merge(args: MergeArgs) {
    let left = chart_in(&Some(&args.left))?;
    let right = chart_in(&Some(&args.right))?;

    let merged = merge_charts(&left, &right)?;
    chart_out(&args.out_file_name, &merged)?;

    the_thing("merged.png", &merged)?;
}

// Merge two charts, `left` and `right`.
//
// * All stitches in `right` chart will be represented as Purls in the final chart.
// * Colors will be mapped according to:
//   * Left Knit -> Color 1
//   * Left Purl -> Color 2
//   * Right Knit -> Color 2
//   * Right Purl -> Color 1

#[throws]
fn merge_charts(left: &Chart, right: &Chart) -> Chart {
    //ensure!
    if left.rows() != right.rows() {
        throw!(anyhow!(
            "Charts must have the same number of rows. ({} != {})",
            left.rows(),
            right.rows()
        ));
    }
    //ensure!
    if left.cols() != right.cols() {
        throw!(anyhow!(
            "Charts must have the same number of rows. ({} != {})",
            left.cols(),
            right.cols()
        ));
    }

    let color_one = Color::from_str("lightblue")?;
    let color_two = Color::from_str("goldenrod")?;
    let mut merged = Chart::new(left.cols() + right.cols(), left.rows());

    for row in merged.rows() {
        for col in merged.cols() {
            if u32::from(col) % 2 == 0 {
                // From left chart.
                let left_col = col / 2u16;
                let left_stitch = left.stitch(row, left_col)?;
                let color = if left_stitch.symbol() == '.' {
                    &color_two
                } else {
                    &color_one
                };
                let merged_stitch = Stitch::new('*', Some(color.clone()));
                merged.set_stitch(row, col, merged_stitch)?;
            } else {
                // From right chart.
                let right_col = col / 2u16;
                let right_stitch = right.stitch(row, right_col)?;
                let color = if right_stitch.symbol() == '*' {
                    &color_one
                } else {
                    &color_two
                };
                let merged_stitch = Stitch::new(' ', Some(color.clone()));
                merged.set_stitch(row, col, merged_stitch)?;
            }
        }
    }

    merged
}
