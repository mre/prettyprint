//! Run
//! ```
//! cargo run --example load-theme
//! ```

use prettyprint::{PrettyPrintError, PrettyPrinter};

fn main() -> Result<(), PrettyPrintError> {
    let theme = include_bytes!("../assets/themes.bin");

    let print = PrettyPrinter::default()
        .language("rust")
        .load_theme(theme.to_vec())
        .build()?;

    print.string(include_str!("../fixtures/fib.rs"))
}
