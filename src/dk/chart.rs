use crate::dk::units::{Cols, Height, Rows, Width};
use anyhow::{anyhow, Error};
use css_color_parser::Color;
use fehler::{throw, throws};
use std::{
    cmp::max,
    convert::{TryFrom, TryInto},
    fmt::{self, Debug, Display, Formatter},
    io::{BufRead, BufReader, Write},
    path::Path,
};

#[derive(Clone, Debug)]
pub struct Stitch {
    symbol: char,
    color: Option<Color>,
}

impl Stitch {
    pub fn new(symbol: char, color: Option<Color>) -> Stitch {
        Stitch { symbol, color }
    }

    pub fn symbol(&self) -> char {
        self.symbol
    }

    #[cfg(test)]
    pub fn color(&self) -> Option<Color> {
        self.color
    }

    // TODO: this only works without arbitrary stitches
    pub fn is_empty(&self) -> bool {
        self.symbol == ' ' || self.symbol == '.'
    }
}

impl Default for Stitch {
    fn default() -> Self {
        Stitch {
            symbol: ' ',
            color: None,
        }
    }
}

impl Display for Stitch {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

pub struct Chart {
    stitches: Vec<Vec<Stitch>>,

    rows: Rows,
    cols: Cols,
}

impl Chart {
    // TODO: use the type system so you can't swap these.
    pub fn new<W, H>(w: W, h: H) -> Chart
    where
        W: Into<Width>,
        H: Into<Height>,
    {
        let width = w.into();
        let height = h.into();
        let mut stitches: Vec<Vec<Stitch>> = Vec::default();
        stitches.resize_with(usize::from(height), Default::default);
        for i in height {
            stitches[usize::from(i)].resize_with(usize::from(width), Default::default);
        }

        Chart {
            stitches,
            rows: height.into(),
            cols: width.into(),
        }
    }

    pub fn rows(&self) -> Rows {
        self.rows
    }

    pub fn cols(&self) -> Cols {
        self.cols
    }

    #[throws]
    pub fn write_to_file(&self, path: impl AsRef<Path>) {
        let mut writer = std::fs::File::create(path)?;
        self.write(&mut writer)?;
    }

    #[throws]
    pub fn write(&self, w: &mut dyn Write) {
        self.write_header(w)?;
        self.write_stitches(w)?;
        self.write_footer(w)?;
    }

    #[throws]
    fn write_header(&self, w: &mut dyn Write) {
        writeln!(w, "CHART")?
    }

    #[throws]
    fn write_stitches(&self, w: &mut dyn Write) {
        for row in &self.stitches {
            for stitch in row {
                write!(w, "{}", stitch)?;
            }
            writeln!(w)?;
        }
    }

    #[throws]
    fn write_footer(&self, _w: &mut dyn Write) {
        // currently a no-op.
    }

    #[throws]
    pub fn read_from_file(path: impl AsRef<Path>) -> Chart {
        let file = std::fs::File::open(path)?;
        let mut rdr = BufReader::new(file);
        Chart::read(&mut rdr)?
    }

    #[throws]
    pub fn read(rdr: &mut impl BufRead) -> Chart {
        Chart::read_header(rdr)?;
        let chart = Chart::read_stitches(rdr)?;
        Chart::read_footer(rdr)?;
        chart
    }

    #[throws]
    fn read_header(rdr: &mut impl BufRead) {
        // Read until we find a line containing the word 'CHART'
        let mut line = String::new();

        loop {
            line.clear();
            let size = rdr.read_line(&mut line)?;
            if size == 0 {
                // Ran out of file before finding the header.
                throw!(anyhow!("Missing header: 'CHART' not found."));
            }
            if line.starts_with("CHART") {
                break;
            }
        }
    }

    #[throws]
    fn read_stitches(rdr: &mut impl BufRead) -> Chart {
        let mut max_cols = 0;

        let mut line = String::new();
        let mut stitches = Vec::new();
        loop {
            line.clear();
            let size = rdr.read_line(&mut line)?;
            if size == 0 {
                // File's done!
                break;
            }

            let stitch_str = line.trim_end_matches('\n');
            max_cols = max(stitch_str.len(), max_cols);

            let mut current_row = Vec::new();
            for ch in stitch_str.chars() {
                let stitch = Stitch::new(ch, None);
                current_row.push(stitch);
            }
            stitches.push(current_row);
        }

        for line in &mut stitches {
            // TODO: we are defaulting to '.' here, but that's wrong.
            //       we *should* default to something knowable, but on the chart.
            //       Perhaps "stitches: .,*" where the first stitch is the default.
            line.resize_with(max_cols, || Stitch::new('.', None));
        }

        let rows = Rows::try_from(stitches.len())?;
        Chart {
            stitches,
            rows,
            cols: max_cols.try_into()?,
        }
    }

    #[throws]
    fn read_footer(_r: &mut impl BufRead) {
        // currently a no-op
    }

    #[throws]
    fn range_check(&self, row: Rows, col: Cols) {
        if row >= self.rows {
            throw!(anyhow!("Row {} should be less than {}", row, self.rows))
        }
        if col >= self.cols {
            throw!(anyhow!("Col {} should be less than {}", col, self.cols))
        }
    }

    #[throws]
    pub fn stitch(&self, row: Rows, col: Cols) -> &Stitch {
        self.range_check(row, col)?;
        &self.stitches[usize::from(row)][usize::from(col)]
    }

    #[throws]
    pub fn set_stitch(&mut self, row: Rows, col: Cols, stitch: Stitch) {
        // ensure!
        self.range_check(row, col)?;
        self.stitches[usize::from(row)][usize::from(col)] = stitch;
    }
}

#[rustfmt::skip::macros(chart, chart_str)]
#[cfg(test)]
mod test {
    use super::*;

    macro_rules! chart_str {
        ($($line:expr),*) => {{
            let mut s = String::default();
            s.push_str("CHART\n");
            $(
                s.push_str($line);
                s.push_str("\n");
            )*
            s
        }}
    }

    macro_rules! chart {
        ($($line:expr),*) => {{
            let mut s = String::default();
            s.push_str("CHART\n");
            $(
                s.push_str($line);
                s.push_str("\n");
            )*
            Chart::read(&mut BufReader::new(s.as_bytes()))
        }}
    }

    #[test]
    fn test_stitch() {
        let def = Stitch::default();
        assert_eq!(def.symbol(), ' ');
        assert_eq!(def.color(), None);
        assert_eq!(format!("STITCH: {}", def), "STITCH:  ");

        let star = Stitch::new('*', None);
        assert_eq!(star.symbol(), '*');
        assert_eq!(star.color(), None);
        assert_eq!(format!("STITCH: {}", star), "STITCH: *");

        // TODO: test the 'color' field
    }

    #[throws]
    #[test]
    fn test_read_write() {
        let chart_in = chart_str!(
            "...*...",
            "..***..",
            "*.****.",
            "*******",
            ".*****.",
            "..***..",
            "...*...",
            ".......");
        let chart = Chart::read(&mut BufReader::new(chart_in.as_bytes()))?;
        assert_eq!(Cols::from(7u8), chart.cols());
        assert_eq!(Rows::from(8u8), chart.rows());

        let mut vec_out = Vec::new();
        chart.write(&mut vec_out)?;
        let chart_out = String::from_utf8(vec_out)?;

        assert_eq!(chart_out, chart_in);
    }

    #[throws]
    #[test]
    fn test_missing_stitches() {
        let chart = chart!(
            "......",
            "*******")?;
        assert_eq!(Cols::from(7u8), chart.cols());
        assert_eq!(Rows::from(2u8), chart.rows());

        let mut vec_out = Vec::new();
        chart.write(&mut vec_out)?;
        let chart_out = String::from_utf8(vec_out)?;

        let fixed_chart = chart_str!(
            ".......",
            "*******"
        );
        //dbg!(fixed_chart);
        assert_eq!(chart_out, fixed_chart);
    }

    #[throws]
    #[test]
    fn test_stitch_get_set() {
        let mut chart = chart!(
            "@.....",
            "..**..",
            "*....^")?;

        assert_eq!('@', chart.stitch(0u8.into(), 0u8.into())?.symbol());
        assert_eq!('*', chart.stitch(1u8.into(), 2u8.into())?.symbol());
        assert_eq!(
            '^',
            chart.stitch(chart.rows() - 1, chart.cols() - 1)?.symbol()
        );

        chart.set_stitch(1u8.into(), 2u8.into(), Stitch::new('#', None))?;
        assert_eq!('#', chart.stitch(1u8.into(), 2u8.into())?.symbol());
    }

    #[throws]
    #[test]
    fn test_range_check() {
        let mut chart = chart!(
            "@.....",
            "..**..",
            "*....^")?;

        assert!(chart.stitch(3u8.into(), 0u8.into()).is_err());
        assert!(chart.stitch(0u8.into(), 6u8.into()).is_err());
        assert!(chart
            .set_stitch(3u8.into(), 0u8.into(), Stitch::default())
            .is_err());
        assert!(chart
            .set_stitch(0u8.into(), 6u8.into(), Stitch::default())
            .is_err());
    }
}
