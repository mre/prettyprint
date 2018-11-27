use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use syntect::dumps::{from_binary, from_reader};
use syntect::highlighting::{Theme, ThemeSet};
use syntect::parsing::{SyntaxReference, SyntaxSet};

use dirs::PROJECT_DIRS;

use errors::*;
use inputfile::{InputFile, InputFileReader};
use syntax_mapping::SyntaxMapping;

pub const PRETTYPRINT_THEME_DEFAULT: &str = "Monokai Extended";

pub struct HighlightingAssets {
    pub syntax_set: SyntaxSet,
    pub theme_set: ThemeSet,
}

impl HighlightingAssets {
    pub fn new() -> Self {
        Self::from_cache().unwrap_or_else(|_| Self::from_binary())
    }

    fn from_cache() -> Result<Self> {
        let theme_set_path = theme_set_path();
        let syntax_set_file = File::open(&syntax_set_path()).chain_err(|| {
            format!(
                "Could not load cached syntax set '{}'",
                syntax_set_path().to_string_lossy()
            )
        })?;
        let syntax_set: SyntaxSet = from_reader(BufReader::new(syntax_set_file))
            .chain_err(|| "Could not parse cached syntax set")?;

        let theme_set_file = File::open(&theme_set_path).chain_err(|| {
            format!(
                "Could not load cached theme set '{}'",
                theme_set_path.to_string_lossy()
            )
        })?;
        let theme_set: ThemeSet = from_reader(BufReader::new(theme_set_file))
            .chain_err(|| "Could not parse cached theme set")?;

        Ok(HighlightingAssets {
            syntax_set,
            theme_set,
        })
    }

    fn get_integrated_syntaxset() -> SyntaxSet {
        from_binary(include_bytes!("../assets/syntaxes.bin"))
    }

    fn get_integrated_themeset() -> ThemeSet {
        from_binary(include_bytes!("../assets/themes.bin"))
    }

    fn from_binary() -> Self {
        let syntax_set = Self::get_integrated_syntaxset();
        let theme_set = Self::get_integrated_themeset();

        HighlightingAssets {
            syntax_set,
            theme_set,
        }
    }

    pub fn get_theme(&self, theme: &str) -> &Theme {
        match self.theme_set.themes.get(theme) {
            Some(theme) => theme,
            None => {
                use ansi_term::Colour::Yellow;
                eprintln!(
                    "{}: Unknown theme '{}', using default.",
                    Yellow.paint("[prettyprint warning]"),
                    theme
                );
                &self.theme_set.themes[PRETTYPRINT_THEME_DEFAULT]
            }
        }
    }

    pub fn get_syntax(
        &self,
        language: Option<String>,
        filename: &InputFile,
        reader: &mut InputFileReader,
        mapping: &SyntaxMapping,
    ) -> &SyntaxReference {
        let syntax = match (language, filename) {
            (Some(language), _) => self.syntax_set.find_syntax_by_token(&language),
            (None, InputFile::Ordinary(filename)) => {
                let path = Path::new(&filename);
                let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                let extension = path.extension().and_then(|x| x.to_str()).unwrap_or("");

                let file_name = mapping.replace(file_name);
                let extension = mapping.replace(extension);

                let ext_syntax = self
                    .syntax_set
                    .find_syntax_by_extension(&file_name)
                    .or_else(|| self.syntax_set.find_syntax_by_extension(&extension));
                let line_syntax = if ext_syntax.is_none() {
                    String::from_utf8(reader.first_line.clone())
                        .ok()
                        .and_then(|l| self.syntax_set.find_syntax_by_first_line(&l))
                } else {
                    None
                };
                let syntax = ext_syntax.or(line_syntax);
                syntax
            }
            (None, InputFile::StdIn) => String::from_utf8(reader.first_line.clone())
                .ok()
                .and_then(|l| self.syntax_set.find_syntax_by_first_line(&l)),
        };

        syntax.unwrap_or_else(|| self.syntax_set.find_syntax_plain_text())
    }
}

fn theme_set_path() -> PathBuf {
    PROJECT_DIRS.cache_dir().join("themes.bin")
}

fn syntax_set_path() -> PathBuf {
    PROJECT_DIRS.cache_dir().join("syntaxes.bin")
}
