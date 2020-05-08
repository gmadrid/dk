use anyhow::{anyhow, Error};
use fehler::throws;
use std::path::{Path, PathBuf};

/// Makes a pathbuf from `path` but with the `.knit` extension.
/// If `suffix` is provided, then append it to the file stem also.
#[throws]
pub fn make_knit_pathbuf(path: impl AsRef<Path>, suffix: Option<&str>) -> PathBuf {
    let name = path
        .as_ref()
        .file_stem()
        .ok_or_else(|| anyhow!("Pathbuf has no filename part: {}", path.as_ref().display()))?;
    let mut owned = name.to_owned();
    if let Some(suffix) = suffix {
        owned.push(suffix);
    }
    let mut result = PathBuf::from(owned);
    result.set_extension("knit");
    result
}
