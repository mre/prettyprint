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
mod controller;
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

use ansi_term::Colour::Green;
use ansi_term::Style;
use std::collections::HashSet;
use std::io;
use std::io::Write;

use builder::{App, Config, PrettyPrint};
use controller::Controller;
use inputfile::InputFile;
use style::{OutputComponent, OutputComponents};

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

use errors::*;

#[cfg(test)]
mod tests {
    use super::*;

    /// Pretty prints its own code
    #[test]
    fn it_works() {
        let printer = PrettyPrint::default();
        printer.run(vec!["src/lib.rs".to_string()]).unwrap();
    }
}
