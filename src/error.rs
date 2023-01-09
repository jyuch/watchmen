use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExecuteError {
    #[error("Broken stdout or stderr pipe.")]
    BrokenStdioPipeError,
}
