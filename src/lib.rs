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

pub use builder::{PagingMode, PrettyPrinter};
// pub use style::OutputComponent;

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

#[cfg(test)]
mod tests {
    use super::*;

    // /// Pretty prints its own code
    // #[test]
    // fn it_works() {
    //     let printer = PrettyPrinter::default().build().unwrap();
    //     printer.file("src/lib.rs").unwrap();
    // }

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
}
