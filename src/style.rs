use std::collections::HashSet;
use std::str::FromStr;

use errors::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum OutputComponent {
    Auto,
    Changes,
    Grid,
    Header,
    Numbers,
    Full,
    Plain,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum OutputWrap {
    Character,
    None,
}

impl Default for OutputWrap {
    fn default() -> Self {
        OutputWrap::None
    }
}

impl FromStr for OutputComponent {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "auto" => Ok(OutputComponent::Auto),
            "changes" => Ok(OutputComponent::Changes),
            "grid" => Ok(OutputComponent::Grid),
            "header" => Ok(OutputComponent::Header),
            "numbers" => Ok(OutputComponent::Numbers),
            "full" => Ok(OutputComponent::Full),
            "plain" => Ok(OutputComponent::Plain),
            _ => Err(format!("Unknown style '{}'", s).into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OutputComponents(pub HashSet<OutputComponent>);

impl Default for OutputComponents {
    fn default() -> Self {
        let mut set = HashSet::new();
        set.insert(OutputComponent::Auto);
        OutputComponents(set)
    }
}

impl OutputComponents {
    pub fn grid(&self) -> bool {
        self.0.contains(&OutputComponent::Grid)
    }

    pub fn header(&self) -> bool {
        self.0.contains(&OutputComponent::Header)
    }

    pub fn numbers(&self) -> bool {
        self.0.contains(&OutputComponent::Numbers)
    }
}
