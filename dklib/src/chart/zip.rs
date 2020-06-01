use crate::chart::{Chart, Stitch};
use crate::Error;
use fehler::throws;
use std::cmp::max;

#[throws]
pub fn zip_charts(left_chart: &Chart, right_chart: &Chart) -> Chart {
    let mut zipped = Chart::new(
        left_chart.cols() + right_chart.cols(),
        max(left_chart.rows(), right_chart.rows()),
    );

    for row in zipped.rows() {
        for col in zipped.cols() {
            let stitch = if col < left_chart.cols() {
                // We're in the left chart.
                if row >= left_chart.rows() {
                    // TODO: arbitrary stitches
                    Stitch::new('.', None)
                } else {
                    left_chart.stitch(row, col)?.clone()
                }
            } else if row >= right_chart.rows() {
                // TODO: arbitrary stitches
                Stitch::new('.', None)
            } else {
                right_chart.stitch(row, col - left_chart.cols())?.clone()
            };

            zipped.set_stitch(row, col, stitch.clone())?;
        }
    }

    zipped
}

#[rustfmt::skip::macros(chart, chart_str)]
#[cfg(test)]
mod test {
    use super::*;

    #[throws]
    #[test]
    fn test_zip() {
        let left = chart!(
            "*...*",
            ".*.*.",
            "..*..",
            ".*.*.",
            "*...*"
        )?;
        let right = chart!(
            "**..**",
            "**..**",
            ".****.",
            ".****.",
            "..**..",
            "..**.."
        )?;

        let zipped = zip_charts(&left, &right)?;

        let zipped_str = chart_str!(
            "*...***..**",
            ".*.*.**..**",
            "..*...****.",
            ".*.*..****.",
            "*...*..**..",
            ".......**.."
        );

        assert_eq!(zipped.write_to_string()?, zipped_str);
    }
}
