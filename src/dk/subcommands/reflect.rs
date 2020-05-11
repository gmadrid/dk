use crate::dk::args::ReflectArgs;
use crate::dk::chart::Chart;
use crate::dk::subcommands::pipe_chart;
use anyhow::Error;
use fehler::throws;

#[throws]
pub fn reflect(args: ReflectArgs) {
    pipe_chart(args.pipe, reflect_chart)?;
}

#[throws]
pub fn reflect_chart(chart: &Chart) -> Chart {
    let mut reflected = Chart::new(chart.cols(), chart.rows());

    for row in chart.rows() {
        for col in chart.cols() {
            let stitch = chart.stitch(row, col)?;

            reflected.set_stitch(row, reflected.cols() - col - 1, stitch)?;
        }
    }

    reflected
}
