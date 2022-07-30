#![deny(missing_docs, missing_debug_implementations)]

//! Toy transaction library

use std::{io::Write, path::Path};

pub use crate::error::Error;

pub mod model;

mod error;

/// Processes transactions and outputs them to the given stream.
pub fn process<W>(path: &Path, mut out_stream: W) -> Result<(), Error>
where
    W: Write,
{
    write!(out_stream, "{}", path.display()).map_err(Error::OutputWrite)?;

    Ok(())
}
