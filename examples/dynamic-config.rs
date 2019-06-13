//! Run
//! ```
//! cargo run --example dynamic-config
//! ```

use prettyprint::{PrettyPrintError, PrettyPrinter};

fn main() -> Result<(), PrettyPrintError> {
    let print = PrettyPrinter::default()
        .language("rust")
        .grid(false)
        .line_numbers(false)
        .build()
        .unwrap();

    print.string("fn main() {")?;

    for x in 0..9 {
        let printer = print.configure().grid(true).build().unwrap();
        printer.string(format!("let x = {};", x))?;
    }

    print.string("}")
}
