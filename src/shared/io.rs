use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Structures that can be saved and restored.
pub trait Save<'d>: Serialize + Deserialize<'d> {
    /// Save path.
    fn path() -> PathBuf;
    /// Restore from saved.
    fn restore() -> Self;
    /// Save to path.
    fn save(&self);
}
