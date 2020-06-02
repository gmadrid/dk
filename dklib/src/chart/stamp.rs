use crate::chart::Chart;
use crate::units::{Cols, Rows};
use crate::Error;
use fehler::throws;

#[throws]
pub fn stamp_chart(chart: &Chart, stamp: &Chart, h_offset: Cols, v_offset: Rows) -> Chart {
    let mut stamped = chart.clone();
    for stamp_row in stamp.rows {
        for stamp_col in stamp.cols {
            let chart_row = v_offset + stamp_row;
            let chart_col = h_offset + stamp_col;
            if chart_row < stamped.rows && chart_col < stamped.cols {
                stamped.set_stitch(
                    v_offset + stamp_row,
                    h_offset + stamp_col,
                    stamp.stitch(stamp_row, stamp_col)?.clone(),
                )?;
            }
        }
    }
    stamped
}

#[rustfmt::skip::macros(chart, chart_str)]
#[cfg(test)]
mod test {
    use super::*;

    #[throws]
    #[test]
    fn test_stamp() {
        let big_chart = chart!(
            "*......",
            ".......",
            "......*",
            ".......",
            ".......",
            "......*"
        )?;

        let small_chart = chart!(
            ".*.",
            ".**",
            "**.",
            "***"
        )?;

        let stamped = stamp_chart(&big_chart, &small_chart, 1_u8.into(), 2_u8.into())?;

        let stamped_str = chart_str!(
            "*......",
            ".......",
            "..*...*",
            "..**...",
            ".**....",
            ".***..*"
        );
        assert_eq!(stamped.write_to_string()?, stamped_str);
    }

    #[throws]
    #[test]
    fn test_stamp_out_of_bounds() {
        let big_chart = chart!(
            "*......",
            ".......",
            "......*",
            ".......",
            ".......",
            "......*"
        )?;

        let small_chart = chart!(
            ".*.",
            ".**",
            "**.",
            "***"
        )?;

        let stamped = stamp_chart(&big_chart, &small_chart, 5_u8.into(), 3_u8.into())?;

        let stamped_str = chart_str!(
            "*......",
            ".......",
            "......*",
            "......*",
            "......*",
            ".....**"
        );
        assert_eq!(stamped.write_to_string()?, stamped_str);
    }
}
