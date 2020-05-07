use anyhow::{anyhow, Error};
use fehler::{throw, throws};
use std::fmt::{self, Debug};

#[derive(Copy, Clone)]
pub enum Stitch {
    Knit,
    Purl,
    Empty,
}

impl Default for Stitch {
    fn default() -> Self {
        Stitch::Empty
    }
}

impl Debug for Stitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            Stitch::Knit => ".",
            Stitch::Purl => "*",
            Stitch::Empty => "#",
        };

        write!(f, "{}", ch)
    }
}

pub struct Chart {
    stitches: Vec<Vec<Stitch>>,

    rows: usize,
    cols: usize,
}

impl Chart {
    // TODO: use the type system so you can't swap these.
    pub fn new(width: u16, height: u16) -> Chart {
        let mut stitches: Vec<Vec<Stitch>> = Vec::default();
        stitches.resize_with(usize::from(height), Default::default);
        for i in 0..height {
            stitches[usize::from(i)].resize_with(usize::from(width), Default::default);
        }

        Chart {
            stitches,
            rows: usize::from(height),
            cols: usize::from(width),
        }
    }

    pub fn rows(&self) -> u16 {
        self.rows as u16
    }

    pub fn cols(&self) -> u16 {
        self.cols as u16
    }

    #[throws]
    fn range_check(&self, row: u16, col: u16) {
        if usize::from(row) >= self.rows {
            throw!(anyhow!("Row {} should be less than {}", row, self.rows))
        }
        if usize::from(col) >= self.cols {
            throw!(anyhow!("Col {} should be less than {}", col, self.cols))
        }
    }

    #[throws]
    pub fn stitch(&self, row: u16, col: u16) -> Stitch {
        self.range_check(row, col)?;
        self.stitches[usize::from(row)][usize::from(col)]
    }

    #[throws]
    pub fn set_stitch(&mut self, row: u16, col: u16, stitch: Stitch) {
        // ensure!
        self.range_check(row, col)?;
        self.stitches[usize::from(row)][usize::from(col)] = stitch;
    }
}