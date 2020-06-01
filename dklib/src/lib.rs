#[cfg(test)]
#[macro_use]
mod test;

pub mod chart;
mod thing;
mod units;

pub use thing::the_thing;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Computed {dimen}, {value}, exceeds max width of {max}")]
    ChartTooBig {
        dimen: &'static str,
        value: u32,
        max: u32,
    },

    #[error("{msg} an empty chart")]
    EmptyChart { msg: &'static str },

    #[error("Charts must have the same number of {dimen}. ({rows1} != {rows2})")]
    IncompatibleMerge {
        dimen: &'static str,
        rows1: u32,
        rows2: u32,
    },

    #[error("Header terminated too early")]
    IncompleteHeader,

    #[error("{name} {value} should be less than {max}")]
    RangeCheck {
        name: &'static str,
        value: u32,
        max: u32,
    },

    #[error("'0' is not a legal value for {argname}")]
    ZeroNotLegal { argname: &'static str },

    // Foreign error types
    #[error("Color failed to parse: {source}")]
    Color {
        #[from]
        source: css_color_parser::ColorParseError,
    },

    #[error("An Image error occurred: {source}")]
    Image {
        #[from]
        source: image::ImageError,
    },

    #[error("An error occurred converting an Int: {source}")]
    IntConvert {
        #[from]
        source: std::num::TryFromIntError,
    },

    #[error("An IO error occurred: {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },
}
pub type Result<T> = std::result::Result<T, Error>;
