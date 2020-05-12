use crate::dk::{
    args::ZipArgs,
    chart::{Chart, Stitch},
    subcommands::chart_out,
    util::make_knit_pathbuf,
};
use anyhow::Error;
use fehler::throws;
use std::cmp::max;

#[throws]
pub fn zip(args: ZipArgs) {
    let left_chart = Chart::read_from_file(args.left_file_name)?;
    let right_chart = Chart::read_from_file(args.right_file_name)?;

    let zipped = zip_charts(&left_chart, &right_chart)?;

    let out_file_name = args
        .out_file_name
        .map(|pb| make_knit_pathbuf(pb, None))
        .transpose()?;
    chart_out(&out_file_name, &zipped)?;
}

#[throws]
fn zip_charts(left_chart: &Chart, right_chart: &Chart) -> Chart {
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
