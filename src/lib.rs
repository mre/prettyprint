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
extern crate wild;

mod assets;
mod builder;
mod clap_app;
mod config;
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
mod util;

pub use builder::PrettyPrinter;
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

    /// Pretty prints its own code
    #[test]
    fn it_works() {
        let printer = PrettyPrinter::default().build().unwrap();
        printer.run(vec!["src/lib.rs".to_string()]).unwrap();
    }

    // Pretty prints its own code with some more formatting shenanigans
    // #[test]
    // fn it_works_with_output_opts() {
    //     let printer = PrettyPrintBuilder::default().with_line_numbers().with_header().with_grid()
    //         .output_components([PrettyPrint::OutputComponent::Numbers])
    //         .build()
    //         .unwrap();
    //     printer.run(vec!["src/lib.rs".to_string()]).unwrap();
    // }
}
