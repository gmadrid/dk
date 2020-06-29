mod parser;
mod span;
mod spanning_reader;

use crate::span::Location;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Internal Error parsing Ident: no valid Ident variant returned: {location}.")]
    IdentFailed { location: Location },

    #[error("Inverted span, start ({first}) must come before end ({second}).")]
    InvertedSpan { first: Location, second: Location },

    #[error("An IO error occurred: {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },

    #[error("TODO Fill in this message")]
    ParseError { msg: String, location: Location },

    #[error("TODO: Get rid of this, and make it a parse error")]
    ParseIntError {
        #[from]
        source: std::num::ParseIntError,
    },
}
