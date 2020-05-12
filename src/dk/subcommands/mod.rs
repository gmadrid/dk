use anyhow::{Error, Result};
use fehler::throws;
use std::{
    io::{BufReader, Read, Write},
    path::PathBuf,
};

mod image_convert;
mod knitchart;
mod pad;
mod reflect;
mod split;
mod trim;
mod zip;

use crate::dk::args::Pipeable;
use crate::dk::chart::Chart;
pub use image_convert::image_convert; // image filename, outfile
pub use knitchart::knitchart; // infile, image filename
pub use pad::pad;
pub use reflect::reflect;
pub use split::{left, right, split};
pub use trim::trim;
pub use zip::zip; // (infile, infile), outfile
                  // pub use zip::mirror // infile, outfile [2 possibilities]

#[throws]
fn pipe_in(path: &Option<PathBuf>) -> Box<dyn Read> {
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

fn pipe_command(
    in_path: Option<PathBuf>,
    out_path: Option<PathBuf>,
    cmd: impl FnOnce(&mut dyn Read, &mut dyn Write) -> Result<()>,
) -> Result<()> {
    let mut rdr = pipe_in(&in_path)?;
    let mut wtr = pipe_out(&out_path)?;
    cmd(rdr.as_mut(), wtr.as_mut())
}

fn pipe_chart(pipe: Pipeable, cmd: impl FnOnce(&Chart) -> Result<Chart>) -> Result<()> {
    pipe_command(pipe.in_file_name, pipe.out_file_name, |rdr, wtr| {
        let chart = Chart::read(&mut BufReader::new(rdr))?;
        let out_chart = cmd(&chart)?;
        out_chart.write(wtr)
    })
}

fn chart_in(in_path: &Option<PathBuf>) -> Result<Chart> {
    let rdr = pipe_in(in_path)?;
    let chart = Chart::read(&mut BufReader::new(rdr))?;
    Ok(chart)
}

fn chart_out(out_path: &Option<PathBuf>, chart: &Chart) -> Result<()> {
    let mut wtr = pipe_out(out_path)?;
    chart.write(&mut wtr)?;
    Ok(())
}
