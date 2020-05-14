use anyhow::{Error, Result};
use fehler::throws;
use std::{
    io::{BufReader, Read, Write},
    path::PathBuf,
};

mod image_convert;
mod knitchart;
mod merge;
mod pad;
mod reflect;
mod split;
mod trim;
mod zip;

use crate::args::Pipeable;
use crate::chart::Chart;
pub use image_convert::image_convert;
pub use knitchart::knitchart;
pub use merge::merge;
pub use pad::pad;
pub use reflect::reflect;
pub use split::{left, right, split};
use std::path::Path;
pub use trim::trim;
pub use zip::zip;

#[throws]
fn pipe_in<P>(path: &Option<P>) -> Box<dyn Read>
where
    P: AsRef<Path>,
{
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

fn chart_in<P>(in_path: &Option<P>) -> Result<Chart>
where
    P: AsRef<Path>,
{
    let rdr = pipe_in(in_path)?;
    let chart = Chart::read(&mut BufReader::new(rdr))?;
    Ok(chart)
}

fn chart_out(out_path: &Option<PathBuf>, chart: &Chart) -> Result<()> {
    let mut wtr = pipe_out(out_path)?;
    chart.write(&mut wtr)?;
    Ok(())
}
