use crate::Error;
use assure::assure;
use fehler::throws;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Location {
    row: u16,
    col: u16,
}

impl Location {
    pub fn new(row: u16, col: u16) -> Location {
        Location { row, col }
    }

    pub fn row(&self) -> u16 {
        self.row
    }

    pub fn col(&self) -> u16 {
        self.col
    }

    pub fn next_row(&mut self) {
        self.row += 1;
        self.col = 1;
    }

    pub fn next_col(&mut self) {
        self.col += 1;
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(r:{}, c:{})", self.row, self.col)
    }
}

#[derive(Debug, Clone)]
pub struct Span {
    start: Location,
    end: Location,
}

impl Span {
    #[throws]
    /// start will be the location of the first char.
    /// end will be the location _after_ the last char.
    /// NOTE: end may point past the end of the file.
    pub fn new(start: Location, end: Location) -> Span {
        assure!(
            start < end,
            Error::InvertedSpan {
                first: start,
                second: end
            }
        );
        Span { start, end }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_location() {
        let l = Location::new(5, 32);
        assert_eq!(l.row, 5);
        assert_eq!(l.col, 32);
    }

    #[test]
    fn new_span() {
        let start = Location::new(5, 12);
        let end = Location::new(6, 20);

        let s = Span::new(start, end).unwrap();
        assert_eq!(s.start.row, 5);
        assert_eq!(s.start.col, 12);
        assert_eq!(s.end.row, 6);
        assert_eq!(s.end.col, 20);
    }

    #[test]
    fn new_span_inverted() {
        let start5 = Location::new(5, 12);
        let end5 = Location::new(5, 15);
        let end6 = Location::new(6, 10);

        assert!(Span::new(end5, start5.clone()).is_err());
        assert!(Span::new(end6, start5).is_err());
    }

    #[test]
    fn next_location() {
        let mut location = Location::new(5, 5);

        location.next_col();
        assert_eq!(location.row, 5);
        assert_eq!(location.col, 6);

        location.next_col();
        assert_eq!(location.row, 5);
        assert_eq!(location.col, 7);

        location.next_row();
        assert_eq!(location.row, 6);
        assert_eq!(location.col, 1);

        location.next_row();
        assert_eq!(location.row, 7);
        assert_eq!(location.col, 1);

        location.next_col();
        assert_eq!(location.row, 7);
        assert_eq!(location.col, 2);

        location.next_col();
        assert_eq!(location.row, 7);
        assert_eq!(location.col, 3);

        location.next_row();
        assert_eq!(location.row, 8);
        assert_eq!(location.col, 1);
    }
}
