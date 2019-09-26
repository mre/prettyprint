//! Create highlighting assets using bat
//! ```
//! bat cache --build --blank --source ./assets --target ./assets/pokemon
//! ```
//! compile by defining ASSET environment variable
//! ```
//! ASSET=../assets/pokemon/syntaxes.bin cargo build --example load-asset
//! ```
//! then run
//! ```
//! cargo run --example load-asset -- syntax
//! ```

use prettyprint::{PrettyPrintError, PrettyPrinter};
use std::{env, process};

const LANGUAGE: &str = "pikalang"; // https://github.com/groteworld/pikalang
const CODE: &str = "
pi pi pi pi pi pi pi pi pi pi pika pipi pi pi pi pi pi pi pi pipi pi pi pi
pi pi pi pi pi pi pi pipi pi pi pi pipi pi pichu pichu pichu pichu ka chu
pipi pi pi pikachu pipi pi pikachu pi pi pi pi pi pi pi pikachu pikachu pi
pi pi pikachu pipi pi pi pikachu pichu pichu pi pi pi pi pi pi pi pi pi pi
pi pi pi pi pi pikachu pipi pikachu pi pi pi pikachu ka ka ka ka ka ka
pikachu ka ka ka ka ka ka ka ka pikachu pipi pi pikachu pipi pikachu";

fn help() -> ! {
    println!(
        "USAGE: \
         \tASSET=../assets/syntaxes.bin {load} syntax\n \
         \tASSET=../assets/themes.bin {load} theme",
        load = "cargo run --example load-asset -- "
    );
    process::exit(2);
}

fn main() -> Result<(), PrettyPrintError> {
    let mut env = env::args().skip(1);

    let buffer = include_bytes!(env!("ASSET"));

    match env.next().as_ref().map(<_>::as_ref) {
        Some("syntax") => {
            let print = PrettyPrinter::default()
                .language(LANGUAGE)
                .load_syntax(buffer.to_vec())
                .build()?;

            print.string(CODE)?;
        }
        Some("theme") => {
            let print = PrettyPrinter::default()
                .language(LANGUAGE)
                .load_theme(buffer.to_vec())
                .build()?;

            print.string(CODE)?;
        }
        _ => help(),
    }

    Ok(())
}
