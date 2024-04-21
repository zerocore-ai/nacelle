//! Error types of the zerocore crate.

use std::path::PathBuf;

use thiserror::Error;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A specialized `Result` type for zerocore-runtime operations.
pub type Result<T> = std::result::Result<T, NacelleError>;

/// The error type for zerocore-runtime operations.
#[derive(Debug, Error)]
pub enum NacelleError {
    /// An error from the wasmtime runtime.
    #[error(transparent)]
    Wasmtime(#[from] wasmtime::Error),

    /// An IO error.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// The permissioned directory is not a subdirectory of the system base.
    #[error("the permissioned directory {0:?} is not a subdirectory of the system base {1:?}")]
    PermissionedDirectoryEscape(PathBuf, PathBuf),
}
