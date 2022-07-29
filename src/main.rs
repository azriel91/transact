use std::{env, io};

fn main() -> io::Result<()> {
    // Skip 1 to skip executable path.
    let file_path = env::args().skip(1).next();
    if let Some(file_path) = file_path {
        transact::process(file_path.as_ref(), io::stdout())
    } else {
        eprintln!("This program expects the transactions file as the first argument.");
        std::process::exit(1);
    }
}
