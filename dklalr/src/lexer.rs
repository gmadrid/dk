use std::iter::Peekable;
use std::str::CharIndices;

pub type Spanned<Token, Loc, Error> = std::result::Result<(Loc, Token, Loc), Error>;

#[derive(Debug, Clone)]
pub enum LexicalError {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    // Lexical tokens
    Comma,
    Eq,
    LParen,
    RParen,

    // Tokens with values
    Ident(String),
    Number(i32),
    String(String),

    // Keywords
    Chart,
    True,
    False,
}

type LexerItem = Spanned<Token, usize, LexicalError>;

pub struct Lexer<'input> {
    chars: Peekable<CharIndices<'input>>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            chars: input.char_indices().peekable(),
        }
    }

    fn single(&mut self, token: Token) -> LexerItem {
        if let Some((i, _)) = self.chars.next() {
            Ok((i, token, i + 1))
        } else {
            todo!()
        }
    }

    fn ident(&mut self) -> LexerItem {
        let mut ident = String::new();

        let mut start = None;
        let mut end = 0_usize;
        while let Some((i, ch)) = self.chars.peek() {
            if start.is_none() {
                // First char
                start = Some(*i);
                if !ch.is_alphabetic() && *ch != '_' {
                    break;
                }
            } else {
                if !ch.is_alphanumeric() && *ch != '_' {
                    break;
                }
            }

            ident.push(*ch);
            end = i + 1;
            self.chars.next();
        }

        if let Some(start) = start {
            let token = if ident == "true" {
                Token::True
            } else if ident == "false" {
                Token::False
            } else {
                Token::Ident(ident)
            };
            return Ok((start, token, end));
        } else {
            panic!("Invalid input for ident.");
        }
    }

    fn string(&mut self) -> LexerItem {
        let mut string = String::new();

        if let Some((start, ch)) = self.chars.next() {
            if ch != '"' {
                panic!("expected '\"' at start of String");
            }

            while let Some((i, ch)) = self.chars.next() {
                if ch == '"' {
                    return Ok((start, Token::String(string), i + 1));
                } else {
                    string.push(ch);
                }
            }
            panic!("unexpected end of input inside string");
        } else {
            panic!("unexpected end of input to start string.")
        }
    }

    fn number(&mut self) -> LexerItem {
        let mut number_str = String::new();
        if let Some((start, ch)) = self.chars.next() {
            if !ch.is_ascii_digit() && ch != '-' {
                panic!("Invalid first char for number.");
            }
            number_str.push(ch);

            let mut end = start + 1;
            while let Some((i, ch)) = self.chars.peek() {
                if !ch.is_ascii_digit() {
                    break;
                }
                number_str.push(*ch);
                end = *i + 1;
                self.chars.next();
            }

            if let Ok(number) = number_str.parse() {
                return Ok((start, Token::Number(number), end));
            } else {
                todo!("Put an error here.")
            }
        } else {
            todo!("Some error return goes here.");
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = LexerItem;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.peek() {
                Some((i, ',')) => return Some(self.single(Token::Comma)),
                Some((i, '=')) => return Some(self.single(Token::Eq)),
                Some((i, '(')) => return Some(self.single(Token::LParen)),
                Some((i, ')')) => return Some(self.single(Token::RParen)),

                Some((i, '"')) => return Some(self.string()),

                Some((i, ch)) if ch.is_whitespace() => { self.chars.next(); },
                Some((i, ch)) if ch.is_ascii_digit() || *ch == '-' => return Some(self.number()),
                Some((i, ch)) if ch.is_alphabetic() || *ch == '_' => return Some(self.ident()),

                None => return None,
                _ => { panic!("unexpected input"); }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::{Lexer, Token};

    #[test]
    fn test_singles() {
        let singles = ",=()";
        let mut lexer = Lexer::new(singles);

        assert_eq!(
            lexer.next().unwrap().unwrap(),
            (0_usize, Token::Comma, 1_usize)
        );
        assert_eq!(
            lexer.next().unwrap().unwrap(),
            (1_usize, Token::Eq, 2_usize)
        );
        assert_eq!(
            lexer.next().unwrap().unwrap(),
            (2_usize, Token::LParen, 3_usize)
        );
        assert_eq!(
            lexer.next().unwrap().unwrap(),
            (3_usize, Token::RParen, 4_usize)
        );
        assert!(lexer.next().is_none());
    }

    #[test]
    fn test_singles_with_spaces() {
        let singles = " , = ( ) ";
        let mut lexer = Lexer::new(singles);

        assert_eq!(
            lexer.next().unwrap().unwrap(),
            (1_usize, Token::Comma, 2_usize)
        );
        assert_eq!(
            lexer.next().unwrap().unwrap(),
            (3_usize, Token::Eq, 4_usize)
        );
        assert_eq!(
            lexer.next().unwrap().unwrap(),
            (5_usize, Token::LParen, 6_usize)
        );
        assert_eq!(
            lexer.next().unwrap().unwrap(),
            (7_usize, Token::RParen, 8_usize)
        );
    }

    #[test]
    fn test_numbers() {
        // TODO: range check the numbers.
        let number = "345 -765 0 1 8888 0088-79 0000";
        let mut lexer = Lexer::new(number);

        assert_eq!(
            lexer.next().unwrap().unwrap(),
            (0_usize, Token::Number(345), 3_usize)
        );
        assert_eq!(
            lexer.next().unwrap().unwrap(),
            (4_usize, Token::Number(-765), 8_usize)
        );
        assert_eq!(
            lexer.next().unwrap().unwrap(),
            (9_usize, Token::Number(0), 10_usize)
        );
        assert_eq!(
            lexer.next().unwrap().unwrap(),
            (11_usize, Token::Number(1), 12_usize)
        );
        assert_eq!(
            lexer.next().unwrap().unwrap(),
            (13_usize, Token::Number(8888), 17_usize)
        );
        assert_eq!(
            lexer.next().unwrap().unwrap(),
            (18_usize, Token::Number(88), 22_usize)
        );
        assert_eq!(
            lexer.next().unwrap().unwrap(),
            (22_usize, Token::Number(-79), 25_usize)
        );
        assert_eq!(
            lexer.next().unwrap().unwrap(),
            (26_usize, Token::Number(0), 30_usize)
        );
        assert!(lexer.next().is_none());
    }

    #[test]
    fn test_string() {
        let strings = r#""" "foo" "bar" "  " "" "close""strings""#;
        let mut lexer = Lexer::new(strings);

        assert_eq!(lexer.next().unwrap().unwrap(), (0_usize, Token::String("".into()), 2_usize));
        assert_eq!(lexer.next().unwrap().unwrap(), (3_usize, Token::String("foo".into()), 8_usize));
        assert_eq!(lexer.next().unwrap().unwrap(), (9_usize, Token::String("bar".into()), 14_usize));
        assert_eq!(lexer.next().unwrap().unwrap(), (15_usize, Token::String("  ".into()), 19_usize));
        assert_eq!(lexer.next().unwrap().unwrap(), (20_usize, Token::String("".into()), 22_usize));
        assert_eq!(lexer.next().unwrap().unwrap(), (23_usize, Token::String("close".into()), 30_usize));
        assert_eq!(lexer.next().unwrap().unwrap(), (30_usize, Token::String("strings".into()), 39_usize));
        assert!(lexer.next().is_none());
    }

    #[test]
    fn test_ident() {
        let idents = "id _id Id ID _2id ____ _22_";
        let mut lexer = Lexer::new(idents);

        assert_eq!(lexer.next().unwrap().unwrap(), (0_usize, Token::Ident("id".to_string()), 2_usize));
        assert_eq!(lexer.next().unwrap().unwrap(), (3_usize, Token::Ident("_id".to_string()), 6_usize));
        assert_eq!(lexer.next().unwrap().unwrap(), (7_usize, Token::Ident("Id".to_string()), 9_usize));
        assert_eq!(lexer.next().unwrap().unwrap(), (10_usize, Token::Ident("ID".to_string()), 12_usize));
        assert_eq!(lexer.next().unwrap().unwrap(), (13_usize, Token::Ident("_2id".to_string()), 17_usize));
        assert_eq!(lexer.next().unwrap().unwrap(), (18_usize, Token::Ident("____".to_string()), 22_usize));
        assert_eq!(lexer.next().unwrap().unwrap(), (23_usize, Token::Ident("_22_".to_string()), 27_usize));
        assert!(lexer.next().is_none());
    }

    #[test]
    fn test_specials() {
        let specials = "true false";
        let mut lexer = Lexer::new(specials);

        assert_eq!(lexer.next().unwrap().unwrap(), (0_usize, Token::True, 4_usize));
        assert_eq!(lexer.next().unwrap().unwrap(), (5_usize, Token::False, 10_usize));
    }
}
