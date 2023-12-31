pub type Result<T> = std::result::Result<T, FSError>;

#[derive(thiserror::Error, Debug)]
pub enum FSError {
    #[error("Uncategorized: {message}")]
    Uncategorized { message: &'static str },
}

impl FSError {
    pub fn from_str(s: &'static str) -> Self {
        FSError::Uncategorized { message: s }
    }
}
