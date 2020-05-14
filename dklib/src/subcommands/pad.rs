use crate::args::PadArgs;
use crate::chart::{Chart, Stitch};
use crate::subcommands::pipe_chart;
use anyhow::Error;
use fehler::throws;

#[throws]
pub fn pad(args: PadArgs) {
    pipe_chart(args.pipe, pad_chart)?;
}

#[throws]
fn pad_chart(chart: &Chart) -> Chart {
    let mut padded = Chart::new(chart.cols() + 2, chart.rows() + 2);

    // Inefficient, but easy.
    for row in padded.rows() {
        for col in padded.cols() {
            padded.set_stitch(row, col, Stitch::new('.', None))?;
        }
    }

    for row in chart.rows() {
        for col in chart.cols() {
            padded.set_stitch(row + 1, col + 1, chart.stitch(row, col)?.clone())?;
        }
    }

    padded
}

#[rustfmt::skip::macros(chart, chart_str)]
#[cfg(test)]
mod test {
    use super::*;

    #[throws]
    #[test]
    fn test_pad() {
        let chart = chart!(
            "******",
            "*....*",
            "*.**.*",
            "*.**.*",
            "*....*",
            "******"
        )?;

        let padded = pad_chart(&chart)?;

        let padded_str = chart_str!(
            "........",
            ".******.",
            ".*....*.",
            ".*.**.*.",
            ".*.**.*.",
            ".*....*.",
            ".******.",
            "........"
        );

        assert_eq!(padded.write_to_string()?, padded_str);
    }
}
