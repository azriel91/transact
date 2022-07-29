#![deny(missing_docs, missing_debug_implementations)]

//! Toy transaction library

use std::{
    io::{self, Write},
    path::Path,
};

pub mod model;

/// Processes transactions and outputs them to the given stream.
pub fn process<W>(path: &Path, mut out_stream: W) -> io::Result<()>
where
    W: Write,
{
    write!(out_stream, "{}", path.display())?;

    Ok(())
}
