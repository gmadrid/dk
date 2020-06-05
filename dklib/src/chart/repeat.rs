use crate::chart::Chart;
use crate::Error;
use assure::assure;
use fehler::throws;

#[throws]
pub fn repeat_chart(chart: &Chart, h: u8, v: u8) -> Chart {
    assure!(h > 0, Error::ZeroNotLegal { argname: "h" });
    assure!(v > 0, Error::ZeroNotLegal { argname: "v" });

    let mut repeated = Chart::new(chart.cols() * h, chart.rows() * v);

    // TODO: reimplement this using "stamp"
    for h_repeat in 0..h {
        for v_repeat in 0..v {
            for inner_col in chart.cols() {
                for inner_row in chart.rows() {
                    let new_chart_row = chart.rows() * v_repeat + inner_row;
                    let new_chart_col = chart.cols() * h_repeat + inner_col;
                    repeated.set_stitch(
                        new_chart_row,
                        new_chart_col,
                        chart.stitch(inner_row, inner_col)?.clone(),
                    )?;
                }
            }
        }
    }

    repeated
}

#[cfg(test)]
mod test {
    use super::*;

    #[throws]
    #[test]
    fn test_repeat() {
        let chart = chart!("..*..", ".*.*.", ".***.", ".**..", ".*...")?;

        let horiz = repeat_chart(&chart, 2, 1)?;
        let horiz_str = chart_str!(
            "..*....*..",
            ".*.*..*.*.",
            ".***..***.",
            ".**...**..",
            ".*....*..."
        );
        assert_eq!(horiz.write_to_string()?, horiz_str);

        let vert = repeat_chart(&chart, 1, 3)?;
        let vert_str = chart_str!(
            "..*..", ".*.*.", ".***.", ".**..", ".*...", "..*..", ".*.*.", ".***.", ".**..",
            ".*...", "..*..", ".*.*.", ".***.", ".**..", ".*..."
        );
        assert_eq!(vert.write_to_string()?, vert_str);

        let both = repeat_chart(&chart, 2, 3)?;
        let both_str = chart_str!(
            "..*....*..",
            ".*.*..*.*.",
            ".***..***.",
            ".**...**..",
            ".*....*...",
            "..*....*..",
            ".*.*..*.*.",
            ".***..***.",
            ".**...**..",
            ".*....*...",
            "..*....*..",
            ".*.*..*.*.",
            ".***..***.",
            ".**...**..",
            ".*....*..."
        );
        assert_eq!(both.write_to_string()?, both_str);
    }
}
