// At any point, request (row, col) from reader.
// Allow peeking.

use crate::span::Location;
use crate::Error;
use fehler::{throws};

pub trait SpanningRead {
    /// Returns the current location in the stream.
    /// NOTE: After the EOF, location() will be one past the EOF.
    ///       (We may revisit this later.)
    fn location(&self) -> Location;

    /// Returns the next char in the stream and advances the current char.
    /// Returns None if there are no more characters (EOF).
    fn next_char(&mut self) -> Option<char>;

    /// Returns the next char, but doesn't advance the current char.
    /// Returns None if there are no more characters (EOF).
    /// Equivalent to `peek_char(); eat_char()`.
    fn peek_char(&self) -> Option<char>;

    /// Advances the current char.
    /// At EOF, this is a no-op.
    fn eat_char(&mut self);

    /// Returns true iff there are no more characters to come.
    fn eof(&self) -> bool;
}

pub struct SpanningReader {
    // Ugh, this isn't great. This is probably 4x the size of the input.
    input_chars: Vec<char>,

    // Location of the next character.
    location: Location,

    // Index in `input_chars` of the next character.
    index: usize,
}

impl SpanningReader {
    #[throws]
    pub fn new<R: std::io::Read>(mut rdr: R) -> SpanningReader {
        let mut input_string = String::new();
        // TODO: get rid of unwrap!!!!!
        let _ = rdr.read_to_string(&mut input_string)?;

        // It's bad enough that we read in the entire file contents, but
        // now, we're basically doubling the size by collecting the chars.
        // However, this _greatly_ simplifies the code.
        // Maybe, one day we can fix this.
        let input_chars = input_string.chars().collect::<Vec<_>>();

        SpanningReader {
            input_chars,
            location: Location::new(1, 1),
            index: 0,
        }
    }
}

impl SpanningRead for SpanningReader {
    fn location(&self) -> Location {
        self.location
    }

    fn next_char(&mut self) -> Option<char> {
        let ch = self.peek_char();
        self.eat_char();
        ch
    }

    fn peek_char(&self) -> Option<char> {
        if self.eof() {
            None
        } else {
            Some(self.input_chars[self.index])
        }
    }

    fn eat_char(&mut self) {
        if !self.eof() {
            let ch = self.input_chars[self.index];
            self.index += 1;

            if ch == '\n' {
                self.location.next_row();
            } else {
                self.location.next_col();
            }
        }
    }

    fn eof(&self) -> bool {
        self.index >= self.input_chars.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn rdr_for_string(s: &str) -> SpanningReader {
        SpanningReader::new(s.as_bytes()).unwrap()
    }

    #[test]
    fn new_test() {
        let sr = rdr_for_string("test_one_line");

        assert_eq!(sr.location(), Location::new(1, 1));
        assert!(!sr.eof());
    }

    #[test]
    fn new_test_empty() {
        let sr = rdr_for_string("");

        assert_eq!(sr.location(), Location::new(1, 1));
        assert!(sr.eof());
    }

    #[test]
    fn test_eat_char() {
        let s = "foo\nbar\nbaz";
        let mut sr = rdr_for_string(s);
        for _ in 0..s.len() {
            dbg!(sr.location());
            assert!(!sr.eof());
            sr.eat_char();
        }
        assert!(sr.eof());
    }

    #[test]
    fn test_peek_char() {
        let s = "foo\nbarf";
        let mut sr = rdr_for_string(s);

        assert_eq!(sr.peek_char(), Some('f'));
        assert_eq!(sr.peek_char(), Some('f'));
        assert_eq!(sr.peek_char(), Some('f'));
        sr.eat_char();
        assert_eq!(sr.peek_char(), Some('o'));
        assert_eq!(sr.peek_char(), Some('o'));

        for _ in sr.index..s.len() {
            sr.eat_char();
        }

        assert_eq!(sr.peek_char(), None);
    }

    #[test]
    fn test_next_char() {
        let s = "foo\nbarf";
        let mut sr = rdr_for_string(s);

        assert_eq!(sr.next_char(), Some('f'));
        assert_eq!(sr.next_char(), Some('o'));
        assert_eq!(sr.next_char(), Some('o'));
        assert_eq!(sr.next_char(), Some('\n'));
        assert_eq!(sr.next_char(), Some('b'));
        assert_eq!(sr.next_char(), Some('a'));
        assert_eq!(sr.next_char(), Some('r'));
        assert_eq!(sr.next_char(), Some('f'));
        assert_eq!(sr.next_char(), None);
    }

    #[test]
    fn test_eof() {
        let mut sr = rdr_for_string("fo\nb");

        assert_eq!(sr.eof(), false);
        sr.eat_char();
        assert_eq!(sr.eof(), false);
        sr.eat_char();
        assert_eq!(sr.eof(), false);
        sr.eat_char();
        assert_eq!(sr.eof(), false);
        sr.eat_char();
        assert_eq!(sr.eof(), true);
    }

    #[test]
    fn test_location() {
        let mut sr = rdr_for_string("foo\nba\nquux");

        assert_eq!(sr.location(), Location::new(1, 1));
        sr.eat_char();
        assert_eq!(sr.location(), Location::new(1, 2));
        sr.eat_char();
        assert_eq!(sr.location(), Location::new(1, 3));
        sr.eat_char();
        assert_eq!(sr.location(), Location::new(1, 4));
        sr.eat_char();
        assert_eq!(sr.location(), Location::new(2, 1));
        sr.eat_char();
        assert_eq!(sr.location(), Location::new(2, 2));
        sr.eat_char();
        assert_eq!(sr.location(), Location::new(2, 3));
        sr.eat_char();
        assert_eq!(sr.location(), Location::new(3, 1));
        sr.eat_char();
        assert_eq!(sr.location(), Location::new(3, 2));
        sr.eat_char();
        assert_eq!(sr.location(), Location::new(3, 3));
        sr.eat_char();
        assert_eq!(sr.location(), Location::new(3, 4));
        sr.eat_char();

        // Eating chars past the end shouldn't change the location.
        assert_eq!(sr.location(), Location::new(3, 5));
        sr.eat_char();
        assert_eq!(sr.location(), Location::new(3, 5));
        sr.eat_char();
    }
}
