// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate derive_builder;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate lazy_static;

extern crate ansi_term;
extern crate atty;
extern crate console;
extern crate content_inspector;
extern crate directories;
extern crate encoding;
extern crate shell_words;
extern crate syntect;

mod assets;
mod builder;
mod decorations;
mod dirs;
mod inputfile;
mod line_range;
mod output;
mod preprocessor;
mod printer;
mod style;
mod syntax_mapping;
mod terminal;

pub use crate::builder::{PagingMode, PrettyPrint, PrettyPrinter};

#[allow(deprecated)] // remove it after error-chain/issues/254 resolved 🤗
mod errors {
    error_chain! {
        foreign_links {
            Clap(::clap::Error);
            Io(::std::io::Error);
            SyntectError(::syntect::LoadingError);
            ParseIntError(::std::num::ParseIntError);
        }
    }
}

pub use errors::Error as PrettyPrintError;

#[cfg(test)]
mod tests {
    use super::*;

    /// Pretty prints its own code
    #[test]
    fn it_works() {
        let printer = PrettyPrinter::default().build().unwrap();
        printer.file("fixtures/fib.rs").unwrap();
    }

    /// Pretty prints its own code with some more formatting shenanigans
    #[test]
    fn it_works_with_output_opts() {
        let printer = PrettyPrinter::default()
            .line_numbers(true)
            .header(true)
            .grid(true)
            .paging_mode(PagingMode::Never)
            .language("ruby")
            .build()
            .unwrap();

        let example = r#"
        def fib(n)        
            return 1 if n <= 1
            fib(n-1) + fib(n-2)
        end
        "#;
        printer.string_with_header(example, "example.rb").unwrap();
    }

    #[test]
    fn it_works_inside_loop() {
        let printer = PrettyPrinter::default()
            .language("markdown")
            .build()
            .unwrap();
        for i in 0..7 {
            printer.string(format!("## Heading {}", i)).unwrap();
        }
    }

    #[test]
    fn it_works_inside_closure() {
        let printer = PrettyPrinter::default()
            .language("markdown")
            .build()
            .unwrap();
        let print_heading = |string| printer.string(format!("## {}", string)).expect("Printed");
        print_heading("Thankyou for making a crate version of `bat` 🥺");
    }

    #[test]
    fn it_can_print_multiple_times() {
        let printer = PrettyPrinter::default().language("rust").build().unwrap();
        printer.string("").unwrap();

        printer.string("let example = Ok(());").unwrap();
        printer
            .string_with_header("let example = Ok(());", "example.rs")
            .unwrap();
        printer.file("fixtures/fib.rs").unwrap();
    }

    #[test]
    fn it_can_load_syntaxset() {
        let buffer: Vec<u8> = include_bytes!("../assets/syntaxes.bin").to_vec();
        let printer = PrettyPrinter::default()
            .language("rust")
            .load_syntax(buffer)
            .build()
            .unwrap();

        printer.file("fixtures/fib.rs").unwrap();
    }

    #[test]
    fn it_can_load_themeset() {
        let buffer = include_bytes!("../assets/themes.bin").to_vec();
        let printer = PrettyPrinter::default()
            .language("rust")
            .load_theme(buffer)
            .build()
            .unwrap();

        printer.file("fixtures/fib.rs").unwrap();
    }

    /// Show available syntax highlighting themes
    #[test]
    fn show_themes() {
        let printer = PrettyPrinter::default().build().unwrap();
        assert!(printer.get_themes().len() > 0);
        println!("{:?}", printer.get_themes().keys());
    }

}
