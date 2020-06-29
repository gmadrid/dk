use crate::span::{Location, Span};
use crate::spanning_reader::{SpanningRead, SpanningReader};
use crate::Error;
use fehler::{throw, throws};

trait ParseNodeBase {
    type ValueType;
    fn span(&self) -> &Span;
    fn value(&self) -> &Self::ValueType;
}

trait ParseNode: ParseNodeBase {
    fn in_first_set(ch: char) -> bool;
}

macro_rules! parse_node_base {
    ( $pnb:ty, $vt:ty ) => {
        impl ParseNodeBase for $pnb {
            type ValueType = $vt;
            fn span(&self) -> &Span {
                &self.span
            }
            fn value(&self) -> &Self::ValueType {
                &self.value
            }
        }
    };
}

#[derive(Debug)]
struct Bool {
    value: bool,
    span: Span,
}
parse_node_base!(Bool, bool);

impl ParseNode for Bool {
    fn in_first_set(ch: char) -> bool {
        ch == 't' || ch == 'f'
    }
}

#[derive(Debug)]
struct Ident {
    value: String,
    span: Span,
}
parse_node_base!(Ident, String);

impl ParseNode for Ident {
    fn in_first_set(ch: char) -> bool {
        ch == '_' || ch.is_ascii_alphabetic()
    }
}

#[derive(Debug)]
struct NumberConstant {
    value: i32,
    span: Span,
}
parse_node_base!(NumberConstant, i32);

impl ParseNode for NumberConstant {
    fn in_first_set(ch: char) -> bool {
        ch == '-' || ch.is_ascii_digit()
    }
}

#[derive(Debug)]
struct StringConstant {
    value: String,
    span: Span,
}
parse_node_base!(StringConstant, String);

impl ParseNode for StringConstant {
    fn in_first_set(ch: char) -> bool {
        ch == '"'
    }
}

pub struct Parser<SR>
where
    SR: SpanningRead,
{
    sr: SR,
}

impl<SR> Parser<SR>
where
    SR: SpanningRead,
{
    pub fn new(sr: SR) -> Parser<SR> {
        Parser { sr }
    }

    fn skip_white(&mut self) {
        while let Some(ch) = self.sr.peek_char() {
            if !ch.is_whitespace() {
                break;
            }

            self.sr.eat_char();
        }
    }

    #[throws]
    fn expect_char(&self, expected: char) {
        if let Some(ch) = self.sr.peek_char() {
            if expected == ch {
                return;
            }
        }
        throw!(Error::ParseError {
            msg: format!("Expected '{}'.", expected),
            location: self.sr.location(),
        });
    }

    #[throws]
    fn parse_string_constant(&mut self) -> StringConstant {
        let start = self.sr.location();

        self.expect_char('"')?;
        self.sr.eat_char(); // skip the quote

        let mut value = String::new();
        while let Some(ch) = self.sr.peek_char() {
            if ch == '"' {
                break;
            }
            value.push(ch);
            self.sr.eat_char();
        }

        self.expect_char('"')?;
        self.sr.next_char();

        let end = self.sr.location();
        StringConstant {
            value,
            span: Span::new(start, end)?,
        }
    }

    #[throws]
    fn parse_number_constant(&mut self) -> NumberConstant {
        let start = self.sr.location();

        let mut number_str = String::new();
        if let Some(first) = self.sr.peek_char() {
            if !NumberConstant::in_first_set(first) {
                throw!(Error::ParseError {
                    msg: format!("Expected 0-9"),
                    location: start,
                });
            }
            self.sr.eat_char();
            number_str.push(first);
        } else {
            throw!(Error::ParseError {
                msg: format!("Unexpected EOF"),
                location: start,
            });
        }

        while let Some(ch) = self.sr.peek_char() {
            if !ch.is_ascii_digit() {
                break;
            }
            number_str.push(ch);
            self.sr.eat_char();
        }

        // TODO: Wrap this error in a ParseError.
        let value: i32 = number_str.parse()?;

        let end = self.sr.location();
        let span = Span::new(start, end)?;

        NumberConstant { value, span }
    }

    #[throws]
    fn parse_ident_or_bool(&mut self) -> (Option<Ident>, Option<Bool>) {
        let start = self.sr.location();

        let mut value = String::new();
        if let Some(first) = self.sr.peek_char() {
            if !Ident::in_first_set(first) {
                throw!(Error::ParseError {
                    msg: format!("Expected '_' or alphabetic."),
                    location: start,
                });
            }
            self.sr.eat_char();
            value.push(first);
        } else {
            throw!(Error::ParseError {
                msg: format!("Unexpected EOF"),
                location: start,
            })
        }

        while let Some(ch) = self.sr.peek_char() {
            if ch != '_' && !ch.is_ascii_alphanumeric() {
                break;
            }
            value.push(ch);
            self.sr.eat_char();
        }

        let end = self.sr.location();
        let span = Span::new(start, end)?;

        if &value == "true" || &value == "false" {
            return (
                None,
                Some(Bool {
                    value: &value == "true",
                    span,
                }),
            );
        }

        (Some(Ident { value, span }), None)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn with_str(s: &str) -> Parser<SpanningReader> {
        let sr = SpanningReader::new(s.as_bytes()).unwrap();
        Parser::new(sr)
    }

    #[test]
    fn test_string_parse() {
        let mut p = with_str("\"foo\"");
        let string_constant = p.parse_string_constant().unwrap();
        assert_eq!(string_constant.value, "foo");
    }

    #[test]
    fn test_string_no_quotes() {
        // Missing lead quote.
        let mut p = with_str("foo\"");
        assert!(p.parse_string_constant().is_err());
        // TODO: check Location
        // TODO: check Error type.

        // Missing end quote
        let mut p = with_str("\"foo");
        assert!(p.parse_string_constant().is_err());
        // TODO: check Location
        // TODO: check Error type.
    }

    #[test]
    fn test_ident() {
        let tests = vec![
            "foo", "_bar", "fooBar", "FooBar", "_123", "_a321", "b456", "bar", "bamf", "UPPER",
            "_UPPER", "tru", "fals", "truee", "falsee", "true1", "false1",
        ];

        for test in tests {
            let mut p = with_str(test);
            let (ident, bool_constant) = p.parse_ident_or_bool().unwrap();
            assert_eq!(ident.unwrap().value, test);
            assert!(bool_constant.is_none());
        }
    }

    #[test]
    fn test_leading_ident() {
        let tests = vec!["foo", "bar", "baz", "quux"];

        let mut p = with_str(&tests.join(" "));
        for test in tests {
            let (ident, bool_constant) = p.parse_ident_or_bool().unwrap();
            assert_eq!(ident.unwrap().value, test);
            assert!(bool_constant.is_none());
            // TODO: test span

            // Skip the space.
            p.skip_white();
        }
    }

    #[test]
    fn test_bad_ident() {
        let tests = vec![" foo", "123"];

        for test in tests {
            let mut p = with_str(test);
            let token = p.parse_ident_or_bool();
            assert!(token.is_err());
        }
    }

    #[test]
    fn test_bool_ident() {
        let tests = vec![("true", true), ("false", false)];
        for test in tests {
            let mut p = with_str(test.0);
            let (ident, bool_constant) = p.parse_ident_or_bool().unwrap();
            assert!(ident.is_none());
            assert_eq!(bool_constant.unwrap().value, test.1);
            // TODO: test span
        }
    }

    #[test]
    fn test_number() {
        let tests = vec![
            ("123", 123_i32),
            ("1", 1),
            ("-323", -323),
            ("444for", 444),
            ("456 ", 456),
        ];
        for test in tests {
            let mut p = with_str(test.0);
            let number = p.parse_number_constant().unwrap();
            assert_eq!(number.value, test.1);
            // TODO: test span
        }
    }

    #[test]
    fn test_bad_number() {
        let tests = vec!["abc", " abc", " 123"];
        for test in tests {
            let mut p = with_str(test);
            let number = p.parse_number_constant();
            assert!(number.is_err())
        }
    }

    #[test]
    fn test_skip_white() {
        let tests = vec![
            ("abc", Some('a')),
            (" abc", Some('a')),
            ("\tabc", Some('a')),
            ("\nabc", Some('a')),
            (" \t \n  abc", Some('a')),
            ("    ", None),
            ("", None),
        ];

        for test in tests {
            let mut p = with_str(test.0);
            p.skip_white();
            assert_eq!(p.sr.peek_char(), test.1);
        }
    }
}
