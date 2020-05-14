use crate::{args::ReflectArgs, chart::Chart, subcommands::pipe_chart};
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
            reflected.set_stitch(row, reflected.cols() - col - 1, stitch.clone())?;
        }
    }

    reflected
}

#[rustfmt::skip::macros(chart, chart_str)]
#[cfg(test)]
mod test {
    use super::*;
    use crate::dk::subcommands::reflect::reflect_chart;

    #[throws]
    #[test]
    fn test_reflect() {
        let chart = chart!(
            "*..",
            "**.",
            ".**")?;
        let reflected = reflect_chart(&chart)?;
        let reflected_str = chart_str!(
            "..*",
            ".**",
            "**.");
        assert_eq!(reflected.write_to_string()?, reflected_str);
    }
}
