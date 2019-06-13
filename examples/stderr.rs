//! Run
//! ```
//! cargo run --example stderr 2> error.log
//! ```
//! then the `error.log` file should contain `ERROR: unexpected` with some gibberish

use prettyprint::{PagingMode, PrettyPrintError, PrettyPrinter};

fn main() -> Result<(), PrettyPrintError> {
    let epprint = PrettyPrinter::default()
        .paging_mode(PagingMode::Error)
        // Comment ☝️ to make `error.log` empty
        .build()
        .unwrap();

    epprint.string("ERROR: unexpected")
}
