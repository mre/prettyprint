use std::collections::HashSet;
use std::env;
use std::io::Write;

use assets::{HighlightingAssets, PRETTYPRINT_THEME_DEFAULT};
use console::Term;
use errors::*;
use inputfile::{InputFile, InputFileReader};
use line_range::RangeCheckResult;
use output::OutputType;
use printer::{InteractivePrinter, Printer};

#[cfg(windows)]
use ansi_term;

use line_range::LineRanges;
use style::{OutputComponent, OutputComponents, OutputWrap};
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
#[builder(name = "PrettyPrinter", setter(into))]
pub struct PrettyPrint {
    // This is a hack, because we can not use skip right now
    // See https://github.com/colin-kiegel/rust-derive-builder/issues/110
    /// Language for syntax highlighting
    #[builder(default = "\"unknown\".to_string()")]
    language: String,

    /// Whether or not to show/replace non-printable
    /// characters like space, tab and newline.
    #[builder(default = "false")]
    show_nonprintable: bool,

    /// The character width of the terminal
    #[builder(default = "Term::stdout().size().1 as usize")]
    term_width: usize,

    /// The width of tab characters.
    /// Currently, a value of 0 will cause tabs to be
    /// passed through without expanding them.
    #[builder(default = "0")]
    tab_width: usize,

    /// Whether or not to simply loop through all input (`cat` mode)
    #[builder(default = "false")]
    loop_through: bool,

    /// Whether or not the output should be colorized
    #[builder(default = "true")]
    colored_output: bool,

    /// Whether or not the output terminal supports true color
    #[builder(default = "is_truecolor_terminal()")]
    true_color: bool,

    /// Print grid
    #[builder(default = "true")]
    grid: bool,

    /// Print header with output file name
    #[builder(default = "true")]
    header: bool,

    /// Print line numbers
    #[builder(default = "true")]
    line_numbers: bool,

    /// Text wrapping mode
    #[builder(default = "OutputWrap::None")]
    output_wrap: OutputWrap,

    /// Pager or STDOUT
    #[builder(default = "PagingMode::QuitIfOneScreen")]
    paging_mode: PagingMode,

    /// Specifies the lines that should be printed
    #[builder(default)]
    line_ranges: LineRanges,

    /// The syntax highlighting theme
    #[builder(default = "String::from(PRETTYPRINT_THEME_DEFAULT)")]
    theme: String,

    /// File extension/name mappings
    #[builder(default)]
    syntax_mapping: SyntaxMapping,

    /// Command to start the pager
    #[builder(default = "None")]
    pager: Option<String>,

    /// Whether to print some characters using italics
    #[builder(default = "false")]
    use_italic_text: bool,
}

impl PrettyPrint {
    pub fn file<T: Into<String>>(self, filename: T) -> Result<()> {
        let file_string = filename.into();
        let input = if file_string == "-" {
            InputFile::StdIn
        } else {
            InputFile::Ordinary(file_string)
        };

        self.run_controller(input, None)
    }

    pub fn string<T: Into<String>>(self, input: T) -> Result<()> {
        self.run_controller(InputFile::String(input.into()), None)
    }

    pub fn string_with_header<T: Into<String>>(self, input: T, header: T) -> Result<()> {
        self.run_controller(InputFile::String(input.into()), Some(header.into()))
    }

    pub fn run_controller(
        &self,
        input_file: InputFile,
        header_overwrite: Option<String>,
    ) -> Result<()> {
        #[cfg(windows)]
        let _ = ansi_term::enable_ansi_support();
        // let interactive_output = atty::is(Stream::Stdout);

        let assets = HighlightingAssets::new();
        let mut reader = input_file.get_reader()?;

        let lang_opt = match self.language.as_ref() {
            "unknown" => None,
            s => Some(s.to_string()),
        };

        // This is faaaar from ideal, I know.
        let mut printer = InteractivePrinter::new(
            &assets,
            &input_file,
            &mut reader,
            self.get_output_components(),
            self.theme.clone(),
            self.colored_output,
            self.true_color,
            self.term_width,
            lang_opt,
            self.syntax_mapping.clone(),
            self.tab_width,
            self.show_nonprintable,
            self.output_wrap,
            self.use_italic_text,
        );

        let mut output_type = OutputType::from_mode(self.paging_mode, self.pager.clone())?;
        let writer = output_type.handle()?;

        self.print_file(reader, &mut printer, writer, &input_file, header_overwrite)?;
        Ok(())
    }

    fn get_output_components(&self) -> OutputComponents {
        let mut components = HashSet::new();
        if self.grid {
            components.insert(OutputComponent::Grid);
        }
        if self.header {
            components.insert(OutputComponent::Header);
        }
        if self.line_numbers {
            components.insert(OutputComponent::Numbers);
        }
        OutputComponents(components)
    }

    fn print_file<'a, P: Printer>(
        &self,
        reader: InputFileReader,
        printer: &mut P,
        writer: &mut Write,
        input_file: &InputFile,
        header_overwrite: Option<String>,
    ) -> Result<()> {
        printer.print_header(writer, &input_file, header_overwrite)?;
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
}

fn is_truecolor_terminal() -> bool {
    env::var("COLORTERM")
        .map(|colorterm| colorterm == "truecolor" || colorterm == "24bit")
        .unwrap_or(false)
}
