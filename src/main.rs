use std::{env, io};

use transact::Error;

fn main() -> Result<(), Error> {
    // `nth(1)` skips executable path.
    let file_path = env::args().nth(1);
    if let Some(file_path) = file_path {
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .expect("Failed to initialize tokio runtime");

        rt.block_on(transact::process(file_path.as_ref(), io::stdout()))
    } else {
        eprintln!("This program expects the transactions file as the first argument.");
        std::process::exit(1);
    }
}
