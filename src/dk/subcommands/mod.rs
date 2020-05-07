use anyhow::{anyhow, Result};
use fehler::{throw, throws};
use std::io::{Read, BufRead, BufReader};
use std::io::Write;
use std::path::PathBuf;

mod image_convert;
mod knitchart;
mod reflect;
mod split;
mod trim;
mod zip;

pub use image_convert::image_convert; // image filename, outfile
pub use knitchart::knitchart; // infile, image filename
pub use reflect::reflect; // infile, outfile
pub use split::split; // infile, (outfile, outfile)
pub use trim::trim; // infile, outfile
pub use zip::zip; // (infile, infile), outfile

// pub use zip::reflectzip // infile, outfile [2 possibilities]

//type PipeCommand = FnOnce(&mut impl Read, &mut impl Write) -> Result<()>;

pub fn pipe(
    in_path: Option<PathBuf>,
    out_path: Option<PathBuf>,
    cmd: impl FnOnce(&mut dyn Read, &mut dyn Write) -> Result<()>,
) {
    let rdr: &dyn BufRead;
    let rdr: &dyn BufRead = in_path.map_or_else(
        || BufReader::<&dyn Read>::new(std::io::stdin()),
        |p| BufReader::<&dyn Read>::new(std::fs::File::open(p).unwrap()));

}
