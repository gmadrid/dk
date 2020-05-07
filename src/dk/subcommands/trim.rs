use crate::dk::{
    args::TrimArgs,
    chart::{Chart, Stitch},
};
use anyhow::{anyhow, Error};
use fehler::{throw, throws};
use std::path::PathBuf;

#[throws]
pub fn trim(args: TrimArgs) {
    for filename in args.filenames {
        let chart = Chart::read_from_file(&filename)?;
        dbg!(chart.rows());
        dbg!(chart.cols());

        let top = dbg!(find_top(&chart)?);
        let bottom = dbg!(find_bottom(&chart)?);
        let left = dbg!(find_left(&chart)?);
        let right = dbg!(find_right(&chart)?);

        let mut trimmed = Chart::new(right - left + 1, bottom - top + 1);
        dbg!(trimmed.rows());
        dbg!(trimmed.cols());
        let mut trimmed_row = 0;
        for row in top..=bottom {
            let mut trimmed_col = 0;
            for col in left..=right {
                let stitch = chart.stitch(row, col)?;
                trimmed.set_stitch(trimmed_row, trimmed_col, stitch)?;
                trimmed_col += 1;
            }
            trimmed_row += 1;
        }

        let mut new_name = filename.file_stem().unwrap().to_owned();
        new_name.push("-trimmed");
        let mut path = PathBuf::from(new_name);
        path.set_extension("knit");

        trimmed.write_to_file(&path)?;
    }
}

#[throws]
fn find_top(chart: &Chart) -> u16 {
    for row in 0..chart.rows() {
        for col in 0..chart.cols() {
            match chart.stitch(row, col)? {
                // Knit and Empty get trimmed
                Stitch::Knit | Stitch::Empty => {
                    // no-op
                }
                _ => {
                    // We found a real character, so this is the top row.
                    return row;
                }
            }
        }
    }

    throw!(anyhow!("Cannot trim an empty chart!"));
}

#[throws]
fn find_bottom(chart: &Chart) -> u16 {
    for row in (0..chart.rows()).rev() {
        for col in 0..chart.cols() {
            match chart.stitch(row, col)? {
                // Knit and Empty get trimmed
                Stitch::Knit | Stitch::Empty => {
                    // no-op
                }
                _ => {
                    // We found a real character, so this is the bottom row.
                    return row;
                }
            }
        }
    }

    throw!(anyhow!("Cannot trim an empty chart."));
}

#[throws]
fn find_left(chart: &Chart) -> u16 {
    for col in 0..chart.cols() {
        for row in 0..chart.rows() {
            match chart.stitch(row, col)? {
                // Knit and Empty get trimmed
                Stitch::Knit | Stitch::Empty => {
                    // no-op
                }
                _ => {
                    // We found a real character, so this is the bottom row.
                    return col;
                }
            }
        }
    }

    throw!(anyhow!("Cannot trim an empty chart"));
}

#[throws]
fn find_right(chart: &Chart) -> u16 {
    for col in (0..chart.cols()).rev() {
        for row in 0..chart.rows() {
            match chart.stitch(row, col)? {
                // Knit and Empty get trimmed
                Stitch::Knit | Stitch::Empty => {
                    // no-op
                }
                _ => {
                    // We found a real character, so this is the bottom row.
                    return col;
                }
            }
        }
    }

    throw!(anyhow!("Cannot trim an empty chart"));
}
