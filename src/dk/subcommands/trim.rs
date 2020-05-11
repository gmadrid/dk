use crate::dk::subcommands::pipe_chart;
use crate::dk::units::{Cols, Rows};
use crate::dk::{args::TrimArgs, chart::Chart};
use anyhow::{anyhow, Error, Result};
use fehler::throws;
use std::iter::Iterator;

#[throws]
pub fn trim(args: TrimArgs) {
    pipe_chart(args.pipe, trim_chart)?;
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
            let stitch = chart.stitch(row + top, col + left)?.clone();
            trimmed.set_stitch(row, col, stitch)?;
        }
    }

    trimmed
}

fn find_top(chart: &Chart) -> Result<Rows> {
    for row in chart.rows() {
        for col in chart.cols() {
            if !chart.stitch(row, col)?.is_empty() {
                return Ok(row);
            }
        }
    }

    Err(anyhow!("Cannot trim an empty chart!"))
}

fn find_bottom(chart: &Chart) -> Result<Rows> {
    for row in chart.rows().into_iter().rev() {
        for col in chart.cols() {
            if !chart.stitch(row, col)?.is_empty() {
                return Ok(row);
            }
        }
    }

    Err(anyhow!("Cannot trim an empty chart."))
}

fn find_left(chart: &Chart) -> Result<Cols> {
    for col in chart.cols() {
        for row in chart.rows() {
            if !chart.stitch(row, col)?.is_empty() {
                return Ok(col);
            }
        }
    }

    Err(anyhow!("Cannot trim an empty chart"))
}

fn find_right(chart: &Chart) -> Result<Cols> {
    for col in chart.cols().into_iter().rev() {
        for row in chart.rows() {
            if !chart.stitch(row, col)?.is_empty() {
                return Ok(col);
            }
        }
    }

    Err(anyhow!("Cannot trim an empty chart"))
}
