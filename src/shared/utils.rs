use crate::prelude::*;

pub fn unsafe_path_string(path: &PathBuf) -> String {
    path.to_str().unwrap().to_string()
}
