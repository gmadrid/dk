use crate::span::Span;
use crate::spanning_reader::{SpanningRead, SpanningReader};
use crate::Error;
use fehler::{throw, throws};

#[derive(Debug)]
struct BoolTerminal {
    value: bool,
    span: Span,
}

#[derive(Debug)]
struct IdentTerminal {
    value: String,
    span: Span,
}

#[derive(Debug)]
struct NumberTerminal {
    value: i32,
    span: Span,
}

#[derive(Debug)]
struct StringTerminal {
    value: String,
    span: Span,
}

#[derive(Debug)]
enum Token {
    Bool(BoolTerminal),
    Ident(IdentTerminal),
    Number(NumberTerminal),
    String(StringTerminal),
}

pub struct Parser<SR>
where
    SR: SpanningRead,
{
    sr: SR,
}

fn in_first_ident(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphabetic()
}

fn in_first_number(ch: char) -> bool {
    ch == '-' || ch.is_ascii_digit()
}

fn in_first_string(ch: char) -> bool {
    ch == '"'
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
    fn parse_string_terminal(&mut self) -> Token {
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
        Token::String(StringTerminal {
            value,
            span: Span::new(start, end)?,
        })
    }

    #[throws]
    fn parse_number_terminal(&mut self) -> Token {
        let start = self.sr.location();

        let mut number_str = String::new();
        if let Some(first) = self.sr.peek_char() {
            if !in_first_number(first) {
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

        Token::Number(NumberTerminal { value, span })
    }

    #[throws]
    fn parse_ident_terminal(&mut self) -> Token {
        let start = self.sr.location();

        let mut value = String::new();
        if let Some(first) = self.sr.peek_char() {
            if !in_first_ident(first) {
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
            return Token::Bool(BoolTerminal {
                value: &value == "true",
                span,
            });
        }

        Token::Ident(IdentTerminal { value, span })
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
        let token = p.parse_string_terminal().unwrap();
        if let Token::String(string_terminal) = token {
            assert_eq!(string_terminal.value, "foo");
        // TODO: test span
        } else {
            panic!("Wrong token type: {:?}", token);
        }
    }

    #[test]
    fn test_string_no_quotes() {
        // Missing lead quote.
        let mut p = with_str("foo\"");
        assert!(p.parse_string_terminal().is_err());
        // TODO: check Location
        // TODO: check Error type.

        // Missing end quote
        let mut p = with_str("\"foo");
        assert!(p.parse_string_terminal().is_err());
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
            let token = p.parse_ident_terminal().unwrap();
            if let Token::Ident(ident_terminal) = token {
                assert_eq!(ident_terminal.value, test);
            // TODO: test span
            } else {
                panic!("Wrong token type: {:?}", token);
            }
        }
    }

    #[test]
    fn test_leading_ident() {
        let tests = vec!["foo", "bar", "baz", "quux"];

        let mut p = with_str(&tests.join(" "));
        for test in tests {
            let token = p.parse_ident_terminal().unwrap();
            if let Token::Ident(ident_terminal) = token {
                assert_eq!(ident_terminal.value, test);
            // TODO: test span
            } else {
                panic!("Wrong token type: {:?}", token);
            }

            // Skip the space.
            p.sr.eat_char();
        }
    }

    #[test]
    fn test_bad_ident() {
        let tests = vec![" foo", "123"];

        for test in tests {
            let mut p = with_str(test);
            let token = p.parse_ident_terminal();
            assert!(token.is_err());
        }
    }

    #[test]
    fn test_bool_ident() {
        let tests = vec![("true", true), ("false", false)];
        for test in tests {
            let mut p = with_str(test.0);
            let token = p.parse_ident_terminal().unwrap();
            if let Token::Bool(bool_terminal) = token {
                assert_eq!(bool_terminal.value, test.1);
            // TODO: test span
            } else {
                panic!("Wrong token type: {:?}", token);
            }
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
            let token = p.parse_number_terminal().unwrap();
            if let Token::Number(number_terminal) = token {
                assert_eq!(number_terminal.value, test.1);
            // TODO: test span
            } else {
                panic!("Wrong token type: {:?}", token);
            }
        }
    }

    #[test]
    fn test_bad_number() {
        let tests = vec!["abc", " abc", " 123"];
        for test in tests {
            let mut p = with_str(test);
            let token = p.parse_number_terminal();
            assert!(token.is_err())
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
