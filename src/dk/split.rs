use anyhow::Error;
use fehler::throws;
use crate::dk::args::SplitArgs;
use crate::dk::chart::Chart;
use std::path::PathBuf;

#[throws]
pub fn split(args: SplitArgs) {
    for filename in args.filenames {
        let chart = Chart::read_from_file(&filename)?;

        // TODO: deal with odd widths.
        let split_point = chart.cols() / 2;

        // TODO: deal with charts with variable number of columns per row.
        //       perhaps pad the rows after reading in the file?
        let mut left_chart = Chart::new(split_point, chart.rows());
        let mut right_chart = Chart::new(chart.cols() - split_point, chart.rows());

        for row in 0..chart.rows() {
            for col in 0..split_point {
                left_chart.set_stitch(row, col, chart.stitch(row, col)?)?;
            }

            for col in split_point..chart.cols() {
                right_chart.set_stitch(row, col - split_point, chart.stitch(row, col)?)?;
            }
        }

        let mut left_name = filename.file_stem().unwrap().to_owned();
        left_name.push("-left");
        let mut left_path = PathBuf::from(left_name);
        left_path.set_extension("knit");
        let mut right_name = filename.file_stem().unwrap().to_owned();
        right_name.push("-right");
        let mut right_path = PathBuf::from(right_name);
        right_path.set_extension("knit");

        println!("Writing to \"{}\" and \"{}\".", left_path.to_string_lossy(), right_path.to_string_lossy());
        left_chart.write_to_file(left_path)?;
        right_chart.write_to_file(right_path)?;
    }
}
