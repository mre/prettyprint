use std::io::Write;

use app::Config;
use assets::HighlightingAssets;
use errors::*;
use inputfile::{InputFile, InputFileReader};
use line_range::{LineRanges, RangeCheckResult};
use output::OutputType;
use printer::{InteractivePrinter, Printer};

pub struct Controller<'a> {
    config: &'a Config<'a>,
    assets: &'a HighlightingAssets,
}

impl<'b> Controller<'b> {
    pub fn new<'a>(config: &'a Config, assets: &'a HighlightingAssets) -> Controller<'a> {
        Controller { config, assets }
    }

    pub fn run(&self) -> Result<bool> {
        let mut output_type = OutputType::from_mode(self.config.paging_mode, self.config.pager)?;
        let writer = output_type.handle()?;

        for input_file in &self.config.files {
            let mut reader = input_file.get_reader()?;
            let mut printer =
                InteractivePrinter::new(&self.config, &self.assets, input_file, &mut reader);
            self.print_file(reader, &mut printer, writer, input_file);
        }

        Ok(true)
    }

    fn print_file<'a, P: Printer>(
        &self,
        reader: InputFileReader,
        printer: &mut P,
        writer: &mut Write,
        input_file: &InputFile,
    ) -> Result<()> {
        printer.print_header(writer, &input_file)?;
        self.print_file_ranges(printer, writer, reader, &self.config.line_ranges)?;
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
