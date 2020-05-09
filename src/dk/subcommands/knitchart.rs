use crate::dk::args::KnitchartArgs;
use crate::dk::chart::Chart;
use crate::dk::thing::the_thing;
use anyhow::Error;
use fehler::throws;

#[throws]
pub fn knitchart(args: KnitchartArgs) {
    for filename in &args.filenames {
        let chart = Chart::read_from_file(filename)?;

        let mut outfile = filename.clone();
        outfile.set_extension("png");
        the_thing(&outfile, &chart)?;
    }
}