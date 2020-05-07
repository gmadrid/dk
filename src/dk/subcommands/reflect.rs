use crate::dk::args::ReflectArgs;
use anyhow::Error;
use fehler::throws;
use crate::dk::chart::Chart;
use std::path::PathBuf;

#[throws]
pub fn reflect(args: ReflectArgs) {
    for filename in args.filenames {
        let chart = Chart::read_from_file(&filename)?;

        let mut reflected = Chart::new(chart.cols(), chart.rows());

        for row in 0..chart.rows() {
            for col in 0..chart.cols() {
                let stitch = chart.stitch(row, col)?;

                reflected.set_stitch(row, reflected.cols() - col - 1, stitch)?;
            }
        }

        let mut new_name = filename.file_stem().unwrap().to_owned();
        new_name.push("-reflected");
        let mut path = PathBuf::from(new_name);
        path.set_extension("knit");

        reflected.write_to_file(&path)?;
    }
}
