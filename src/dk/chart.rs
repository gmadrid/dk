use crate::dk::units::{Cols, Height, Rows, Width};
use anyhow::{anyhow, Error};
use fehler::{throw, throws};
use std::cmp::max;
use std::convert::{TryFrom, TryInto};
use std::fmt::{self, Debug, Display, Formatter};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use css_color_parser::{Color, NAMED_COLORS};

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

    pub fn color(&self) -> Option<Color> {
        self.color
    }
}

impl Default for Stitch {
    fn default() -> Self {
        Stitch { symbol: ' ', color: None }
    }
}

impl Display for Stitch {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

//
//
// #[derive(Copy, Clone)]
// pub enum Stitch {
//     Knit,
//     Purl,
//     Empty,
// }
//
// impl Default for Stitch {
//     fn default() -> Self {
//         Stitch::Empty
//     }
// }
//
// impl Debug for Stitch {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let ch = match self {
//             Stitch::Knit => ".",
//             Stitch::Purl => "*",
//             Stitch::Empty => "#",
//         };
//
//         write!(f, "{}", ch)
//     }
// }
//
// impl Display for Stitch {
//     fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
//         // Display and Debug are the same.
//         write!(f, "{:?}", self)
//     }
// }

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

            }
            stitches.push(current_row);
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
    pub fn stitch(&self, row: Rows, col: Cols) -> Stitch {
        self.range_check(row, col)?;
        self.stitches[usize::from(row)][usize::from(col)]
    }

    #[throws]
    pub fn set_stitch(&mut self, row: Rows, col: Cols, stitch: Stitch) {
        // ensure!
        self.range_check(row, col)?;
        self.stitches[usize::from(row)][usize::from(col)] = stitch;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stitch() {
        let def = Stitch::default();
        assert_eq!(def.symbol(), ' ');
        assert_eq!(def.color(), None);
    }
}