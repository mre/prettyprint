use std::env;
use std::io::Write;

use assets::HighlightingAssets;
use errors::*;
use inputfile::{InputFile, InputFileReader};
use line_range::RangeCheckResult;
use output::OutputType;
use printer::{InteractivePrinter, Printer};

use atty::{self, Stream};

#[cfg(windows)]
use ansi_term;

// use assets::PRETTYPRINT_THEME_DEFAULT;
// use errors::*;
// use line_range::{LineRange, LineRanges};
use line_range::LineRanges;
use style::{OutputComponents, OutputWrap};
use syntax_mapping::SyntaxMapping;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PagingMode {
    Always,
    QuitIfOneScreen,
    Never,
}

impl Default for PagingMode {
    fn default() -> Self {
        PagingMode::Always
    }
}

#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
pub struct PrettyPrint {
    /// List of files to print
    pub files: Vec<InputFile>,

    /// The explicitly configured language, if any
    pub language: Option<String>,

    /// Whether or not to show/replace non-printable characters like space, tab and newline.
    pub show_nonprintable: bool,

    /// The character width of the terminal
    pub term_width: usize,

    /// The width of tab characters.
    /// Currently, a value of 0 will cause tabs to be passed through without expanding them.
    pub tab_width: usize,

    /// Whether or not to simply loop through all input (`cat` mode)
    pub loop_through: bool,

    /// Whether or not the output should be colorized
    pub colored_output: bool,

    /// Whether or not the output terminal supports true color
    pub true_color: bool,

    /// Style elements (grid, line numbers, ...)
    pub output_components: OutputComponents,

    /// Text wrapping mode
    pub output_wrap: OutputWrap,

    /// Pager or STDOUT
    pub paging_mode: PagingMode,

    /// Specifies the lines that should be printed
    pub line_ranges: LineRanges,

    /// The syntax highlighting theme
    pub theme: String,

    /// File extension/name mappings
    pub syntax_mapping: SyntaxMapping,

    /// Command to start the pager
    pub pager: Option<String>,

    /// Whether to print some characters using italics
    pub use_italic_text: bool,
}

impl PrettyPrint {
    pub fn run(self, inputs: Vec<String>) -> Result<()> {
        let files = inputs
            .iter()
            .map(|filename| {
                if filename == "-" {
                    InputFile::StdIn
                } else {
                    InputFile::Ordinary(filename.to_string())
                }
            }).collect();

        #[cfg(windows)]
        let _ = ansi_term::enable_ansi_support();
        // let interactive_output = atty::is(Stream::Stdout);

        // let config = self.config(files)?;
        self.run_controller(files)
    }

    pub fn run_controller(&self, input_files: Vec<InputFile>) -> Result<()> {
        let mut output_type = OutputType::from_mode(self.paging_mode, self.pager.clone())?;
        let writer = output_type.handle()?;

        let assets = HighlightingAssets::new();
        for input_file in &input_files {
            let mut reader = input_file.get_reader()?;

            // This is faaaar from ideal, I know.
            let mut printer = InteractivePrinter::new(
                &assets,
                input_file,
                &mut reader,
                self.output_components.clone(),
                self.theme.clone(),
                self.colored_output,
                self.true_color,
                self.term_width,
                self.language.clone(),
                self.syntax_mapping.clone(),
                self.tab_width,
                self.show_nonprintable,
                self.output_wrap,
                self.use_italic_text,
            );
            self.print_file(reader, &mut printer, writer, input_file)?;
        }
        Ok(())
    }

    fn print_file<'a, P: Printer>(
        &self,
        reader: InputFileReader,
        printer: &mut P,
        writer: &mut Write,
        input_file: &InputFile,
    ) -> Result<()> {
        printer.print_header(writer, &input_file)?;
        self.print_file_ranges(printer, writer, reader, &self.line_ranges)?;
        printer.print_footer(writer)?;

        Ok(())
    }

    fn print_file_ranges<'a, P: Printer>(
        &self,
        printer: &mut P,
        writer: &mut Write,
        mut reader: InputFileReader,
        line_ranges: &LineRanges,
    ) -> Result<()> {
        let mut line_buffer = Vec::new();
        let mut line_number: usize = 1;

        while reader.read_line(&mut line_buffer)? {
            match line_ranges.check(line_number) {
                RangeCheckResult::OutsideRange => {
                    // Call the printer in case we need to call the syntax highlighter
                    // for this line. However, set `out_of_range` to `true`.
                    printer.print_line(true, writer, line_number, &line_buffer)?;
                }
                RangeCheckResult::InRange => {
                    printer.print_line(false, writer, line_number, &line_buffer)?;
                }
                RangeCheckResult::AfterLastRange => {
                    break;
                }
            }

            line_number += 1;
            line_buffer.clear();
        }
        Ok(())
    }
    // pub fn config(&self, files: Vec<InputFile>) -> Result<Config> {
    // let output_components = self.output_components()?;

    // let paging_mode = match self.matches.value_of("paging") {
    //     Some("always") => PagingMode::Always,
    //     Some("never") => PagingMode::Never,
    //     Some("auto") | _ => {
    //         if files.contains(&InputFile::StdIn) {
    //             // If we are reading from stdin, only enable paging if we write to an
    //             // interactive terminal and if we do not *read* from an interactive
    //             // terminal.
    //             if self.interactive_output && !atty::is(Stream::Stdin) {
    //                 PagingMode::QuitIfOneScreen
    //             } else {
    //                 PagingMode::Never
    //             }
    //         } else {
    //             if self.interactive_output {
    //                 PagingMode::QuitIfOneScreen
    //             } else {
    //                 PagingMode::Never
    //             }
    //         }
    //     }
    // };

    // let mut syntax_mapping = SyntaxMapping::new();

    // if let Some(values) = self.matches.values_of("map-syntax") {
    //     for from_to in values {
    //         let parts: Vec<_> = from_to.split(":").collect();

    //         if parts.len() != 2 {
    //             return Err("Invalid syntax mapping. The format of the -m/--map-syntax option is 'from:to'.".into());
    //         }

    //         syntax_mapping.insert(parts[0].into(), parts[1].into());
    //     }
    // }

    // Ok(Config {
    //     true_color: is_truecolor_terminal(),
    //     language: self.matches.value_of("language").or_else(|| {
    //         if self.matches.is_present("show-all") {
    //             Some("show-nonprintable")
    //         } else {
    //             None
    //         }
    //     }),
    //     show_nonprintable: self.matches.is_present("show-all"),
    //     output_wrap: if !self.interactive_output {
    //         // We don't have the tty width when piping to another program.
    //         // There's no point in wrapping when this is the case.
    //         OutputWrap::None
    //     } else {
    //         match self.matches.value_of("wrap") {
    //             Some("character") => OutputWrap::Character,
    //             Some("never") => OutputWrap::None,
    //             Some("auto") | _ => {
    //                 if output_components.plain() {
    //                     OutputWrap::None
    //                 } else {
    //                     OutputWrap::Character
    //                 }
    //             }
    //         }
    //     },
    //     colored_output: match self.matches.value_of("color") {
    //         Some("always") => true,
    //         Some("never") => false,
    //         Some("auto") | _ => self.interactive_output,
    //     },
    //     paging_mode,
    //     term_width: self
    //         .matches
    //         .value_of("terminal-width")
    //         .and_then(|w| {
    //             if w.starts_with("+") || w.starts_with("-") {
    //                 // Treat argument as a delta to the current terminal width
    //                 w.parse().ok().map(|delta: i16| {
    //                     let old_width: u16 = Term::stdout().size().1;
    //                     let new_width: i32 = old_width as i32 + delta as i32;

    //                     if new_width <= 0 {
    //                         old_width as usize
    //                     } else {
    //                         new_width as usize
    //                     }
    //                 })
    //             } else {
    //                 w.parse().ok()
    //             }
    //         }).unwrap_or(Term::stdout().size().1 as usize),
    //     loop_through: !(self.interactive_output
    //         || self.matches.value_of("color") == Some("always")
    //         || self.matches.value_of("decorations") == Some("always")),
    //     files,
    //     tab_width: self
    //         .matches
    //         .value_of("tabs")
    //         .map(String::from)
    //         .or_else(|| env::var("PRETTYPRINT_TABS").ok())
    //         .and_then(|t| t.parse().ok())
    //         .unwrap_or(
    //             if output_components.plain() && paging_mode == PagingMode::Never {
    //                 0
    //             } else {
    //                 4
    //             },
    //         ),
    //     theme: self
    //         .matches
    //         .value_of("theme")
    //         .map(String::from)
    //         .or_else(|| env::var("PRETTYPRINT_THEME").ok())
    //         .unwrap_or(String::from(PRETTYPRINT_THEME_DEFAULT)),
    //     line_ranges: LineRanges::from(
    //         transpose(
    //             self.matches
    //                 .values_of("line-range")
    //                 .map(|vs| vs.map(LineRange::from).collect()),
    //         )?.unwrap_or(vec![]),
    //     ),
    //     output_components,
    //     syntax_mapping,
    //     pager: self.matches.value_of("pager"),
    //     use_italic_text: match self.matches.value_of("italic-text") {
    //         Some("always") => true,
    //         _ => false,
    //     },
    // })
    // }
}

fn is_truecolor_terminal() -> bool {
    env::var("COLORTERM")
        .map(|colorterm| colorterm == "truecolor" || colorterm == "24bit")
        .unwrap_or(false)
}
}
}
