//! Run
//! ```
//! cargo run --example load-syntax
//! ```

use prettyprint::{PrettyPrintError, PrettyPrinter};

const LANGUAGE: &str = "pikalang"; // https://github.com/groteworld/pikalang
const CODE: &str = "
pi pi pi pi pi pi pi pi pi pi pika pipi pi pi pi pi pi pi pi pipi pi pi pi
pi pi pi pi pi pi pi pipi pi pi pi pipi pi pichu pichu pichu pichu ka chu
pipi pi pi pikachu pipi pi pikachu pi pi pi pi pi pi pi pikachu pikachu pi
pi pi pikachu pipi pi pi pikachu pichu pichu pi pi pi pi pi pi pi pi pi pi
pi pi pi pi pi pikachu pipi pikachu pi pi pi pikachu ka ka ka ka ka ka
pikachu ka ka ka ka ka ka ka ka pikachu pipi pi pikachu pipi pikachu";

fn main() -> Result<(), PrettyPrintError> {
    let syntax = include_bytes!("../assets/syntaxes.bin");

    let print = PrettyPrinter::default()
        .language(LANGUAGE)
        .load_syntax(syntax.to_vec())
        .build()?;

    print.string(CODE)
}
