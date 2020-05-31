use crate::chart::{Chart, Stitch};
use crate::Error;
use anyhow::{anyhow, Error};
use css_color_parser::Color;
use fehler::{throw, throws};
use std::str::FromStr;

// Merge two charts, `left` and `right`.
//
// * All stitches in `right` chart will be represented as Purls in the final chart.
// * Colors will be mapped according to:
//   * Left Knit -> Color 1
//   * Left Purl -> Color 2
//   * Right Knit -> Color 2
//   * Right Purl -> Color 1

#[throws]
pub fn merge_charts(left: &Chart, right: &Chart) -> Chart {
    //ensure!
    if left.rows() != right.rows() {
        throw!(Error::IncompatibleMerge {
            dimen: "rows",
            rows1: left.rows().into(),
            rows2: right.rows().into()
        });
    }
    //ensure!
    if left.cols() != right.cols() {
        throw!(Error::IncompatibleMerge {
            dimen: "columns",
            rows1: left.cols().into(),
            rows2: right.cols().into()
        });
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
                let color = if left_stitch.symbol() == '*' {
                    &color_two
                } else {
                    &color_one
                };
                let merged_stitch = Stitch::new('*', Some(*color));
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
                let merged_stitch = Stitch::new(' ', Some(*color));
                merged.set_stitch(row, col, merged_stitch)?;
            }
        }
    }

    merged
}
