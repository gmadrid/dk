use crate::dk::subcommands::pipe_chart;
use crate::dk::{
    args::TrimArgs,
    chart::{Chart, Stitch},
};
use anyhow::{anyhow, Error, Result};
use fehler::throws;

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
    for (trimmed_row, row) in (top..=bottom).enumerate() {
        for (trimmed_col, col) in (left..=right).enumerate() {
            let stitch = chart.stitch(row, col)?;
            trimmed.set_stitch(trimmed_row as u16, trimmed_col as u16, stitch)?;
        }
    }

    trimmed
}

fn find_top(chart: &Chart) -> Result<u16> {
    for row in 0..chart.rows() {
        for col in 0..chart.cols() {
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

fn find_bottom(chart: &Chart) -> Result<u16> {
    for row in (0..chart.rows()).rev() {
        for col in 0..chart.cols() {
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

fn find_left(chart: &Chart) -> Result<u16> {
    for col in 0..chart.cols() {
        for row in 0..chart.rows() {
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

fn find_right(chart: &Chart) -> Result<u16> {
    for col in (0..chart.cols()).rev() {
        for row in 0..chart.rows() {
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
