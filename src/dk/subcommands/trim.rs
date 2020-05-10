use crate::dk::subcommands::pipe_chart;
use crate::dk::{
    args::TrimArgs,
    chart::{Chart, Stitch},
};
use anyhow::{anyhow, Error, Result};
use fehler::throws;
use crate::dk::units::{Rows, Cols};
use std::iter::Iterator;

#[throws]
pub fn trim(args: TrimArgs) {
    pipe_chart(args.in_file_name, args.out_file_name, trim_chart)?;
}

#[throws]
fn trim_chart(chart: &Chart) -> Chart {
    let top = find_top(&chart)?;
    let bottom = find_bottom(&chart)?;
    let left = find_left(&chart)?;
    let right = find_right(&chart)?;

    let mut trimmed = Chart::new(right - left + 1, bottom - top + 1);
    for row in bottom - top + 1 {
        for col in right - left + 1 {
            let stitch = chart.stitch(row + top, col + left)?;
            trimmed.set_stitch(row, col, stitch)?;
        }

    }

    // for row in top..(bottom + 1) {
    //     for (trimmed_col, col) in (left..=right).enumerate() {
    //         let stitch = chart.stitch(row, col)?;
    //         trimmed.set_stitch(Rows::from(trimmed_row), Cols::from(trimmed_col), stitch)?;
    //     }
    // }

    trimmed
}

fn find_top(chart: &Chart) -> Result<Rows> {
    for row in chart.rows() {
        for col in chart.cols() {
            match chart.stitch(row, col)? {
                // Knit and Empty get trimmed
                Stitch::Knit | Stitch::Empty => {
                    // no-op
                }
                _ => {
                    // We found a real character, so this is the top row.
                    return Ok(row);
                }
            }
        }
    }

    Err(anyhow!("Cannot trim an empty chart!"))
}

fn find_bottom(chart: &Chart) -> Result<Rows> {
    for row in chart.rows().into_iter().rev() {
        for col in chart.cols() {
            match chart.stitch(row, col)? {
                // Knit and Empty get trimmed
                Stitch::Knit | Stitch::Empty => {
                    // no-op
                }
                _ => {
                    // We found a real character, so this is the bottom row.
                    return Ok(row);
                }
            }
        }
    }

    Err(anyhow!("Cannot trim an empty chart."))
}

fn find_left(chart: &Chart) -> Result<Cols> {
    for col in chart.cols() {
        for row in chart.rows() {
            match chart.stitch(row, col)? {
                // Knit and Empty get trimmed
                Stitch::Knit | Stitch::Empty => {
                    // no-op
                }
                _ => {
                    // We found a real character, so this is the bottom row.
                    return Ok(col);
                }
            }
        }
    }

    Err(anyhow!("Cannot trim an empty chart"))
}

fn find_right(chart: &Chart) -> Result<Cols> {
    for col in chart.cols().into_iter().rev() {
        for row in chart.rows() {
            match chart.stitch(row, col)? {
                // Knit and Empty get trimmed
                Stitch::Knit | Stitch::Empty => {
                    // no-op
                }
                _ => {
                    // We found a real character, so this is the bottom row.
                    return Ok(col);
                }
            }
        }
    }

    Err(anyhow!("Cannot trim an empty chart"))
}
