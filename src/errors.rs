use thiserror::Error;

#[derive(Debug, Error)]
pub enum BoilerError {
    #[error("Failed to get your selection. {0}")]
    SelectionError(#[from] std::io::Error),
}
