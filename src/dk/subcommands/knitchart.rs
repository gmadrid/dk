use crate::dk::args::{KnitchartArgs};
use crate::dk::chart::Chart;
use anyhow::Error;
use fehler::throws;
use std::path::PathBuf;
use crate::dk::thing::the_thing;

#[throws]
pub fn knitchart(args: KnitchartArgs) {
    for filename in &args.filenames {
        let chart = Chart::read_from_file(filename)?;

        let mut outfile = filename.clone();
        outfile.set_extension("png");
        the_thing(&outfile, &chart)?;
    }
}
