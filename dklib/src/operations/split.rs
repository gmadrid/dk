use crate::Error;
use crate::chart::Chart;
use fehler::throws;

#[throws]
pub fn split_chart(chart: &Chart) -> (Chart, Chart) {
    // TODO: deal with odd widths.
    let split_point = chart.cols() / 2;

    // TODO: deal with charts with variable number of columns per row.
    //       perhaps pad the rows after reading in the file?
    let mut left_chart = Chart::new(split_point, chart.rows());
    let mut right_chart = Chart::new(chart.cols() - split_point, chart.rows());

    for row in chart.rows() {
        for col in split_point {
            left_chart.set_stitch(row, col, chart.stitch(row, col)?.clone())?;
        }

        for col in chart.cols() - split_point {
            right_chart.set_stitch(row, col, chart.stitch(row, col + split_point)?.clone())?;
        }
    }

    (left_chart, right_chart)
}

#[rustfmt::skip::macros(chart, chart_str)]
#[cfg(test)]
mod test {
    use super::*;

    #[throws]
    #[test]
    fn test_split_even() {
        let chart = chart!(
            "..**..",
            ".****.",
            "******",
            ".****.",
            ".**..."
        )?;

        let (left, right) = split_chart(&chart)?;
        let chart_left_str = chart_str!(
            "..*",
            ".**",
            "***",
            ".**",
            ".**"
        );
        let chart_right_str = chart_str!(
            "*..",
            "**.",
            "***",
            "**.",
            "..."
        );

        assert_eq!(left.write_to_string()?, chart_left_str);
        assert_eq!(right.write_to_string()?, chart_right_str);
    }

    // TODO: test_split_odd (and left and right)
}
