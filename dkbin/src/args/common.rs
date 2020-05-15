use anyhow::{Error, Result};
use dklib::chart::Chart;
use fehler::throws;
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct ChartFileIn {
    #[structopt(
        name = "input_chart_name",
        long = "in",
        short = "i",
        parse(from_os_str)
    )]
    pub chart_file_in: Option<PathBuf>,
}

#[derive(Debug, StructOpt)]
pub struct ChartFileOut {
    #[structopt(
        name = "output_chart_name",
        long = "out",
        short = "o",
        parse(from_os_str)
    )]
    pub chart_file_out: Option<PathBuf>,
}

/// A common set of arguments for "pipeable" chart operations.
/// Pipeable operations ase those which take a chart as input and produce a chart as output.
/// Include this struct and "flatten" it to reuse the common code and to ensure that
/// the arguments names are the same for all subcommands.
#[derive(Debug, StructOpt)]
pub struct Pipeable {
    #[structopt(flatten)]
    pub infile: ChartFileIn,

    #[structopt(flatten)]
    pub outfile: ChartFileOut,
}

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
fn pipe_out<P>(path: &Option<P>) -> Box<dyn Write>
where
    P: AsRef<Path>,
{
    let pipe: Box<dyn Write> = if let Some(p) = path {
        Box::new(std::fs::File::create(p)?)
    } else {
        Box::new(std::io::stdout())
    };
    pipe
}

#[throws]
fn pipe_command(
    in_path: Option<PathBuf>,
    out_path: Option<PathBuf>,
    cmd: impl FnOnce(&mut dyn Read, &mut dyn Write) -> Result<()>,
) {
    let mut rdr = pipe_in(&in_path)?;
    let mut wtr = pipe_out(&out_path)?;
    cmd(rdr.as_mut(), wtr.as_mut())?;
}

#[throws]
pub fn chart_in(infile: &ChartFileIn) -> Chart {
    chart_path_in(&infile.chart_file_in)?
}

#[throws]
pub fn chart_path_in<P>(in_path: &Option<P>) -> Chart
where
    P: AsRef<Path>,
{
    let rdr = pipe_in(in_path)?;
    Chart::read(&mut BufReader::new(rdr))?
}

#[throws]
pub fn chart_out(outfile: &ChartFileOut, chart: &Chart) {
    // TODO: write in terms of chart_path_out
    chart_path_out(&outfile.chart_file_out, chart)?
}

#[throws]
pub fn chart_path_out<P>(out_path: &Option<P>, chart: &Chart)
where
    P: AsRef<Path>,
{
    let mut wtr = pipe_out(out_path)?;
    chart.write(&mut wtr)?;
}

#[throws]
pub fn pipe_chart(pipe: Pipeable, cmd: impl FnOnce(&Chart) -> Result<Chart>) {
    pipe_command(
        pipe.infile.chart_file_in,
        pipe.outfile.chart_file_out,
        |rdr, wtr| {
            let chart = Chart::read(&mut BufReader::new(rdr))?;
            let out_chart = cmd(&chart)?;
            out_chart.write(wtr)
        },
    )?;
}
