use std::path::PathBuf;

use super::driver::*;

/// Initial LLM driver.
pub struct DefactoModel {}

impl DefactoModel {
    fn from_path(path: PathBuf) -> Self {
        Self {}
    }
}

impl LLMDriver for DefactoModel {}
