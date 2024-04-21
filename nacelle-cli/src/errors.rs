use nacelle::{wasmtime, NacelleError};
use thiserror::Error;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A specialized `Result` type for the `zerocore` CLI.
pub type Result<T> = std::result::Result<T, NacelleCliError>;

/// Errors that can occur while running the `zerocore` CLI.
#[derive(Debug, Error)]
pub enum NacelleCliError {
    /// An error from nacelle.
    #[error(transparent)]
    Nacelle(#[from] NacelleError),

    /// An error from the wasmtime runtime.
    #[error(transparent)]
    Wasmtime(#[from] wasmtime::Error),

    /// Function not exported.
    #[error("function not exported: {0}")]
    FunctionNotExported(String),

    /// Unsupported parameter type.
    #[error("unsupported function parameter type: {0:?}")]
    UnsupportedFunctionParamType(nacelle::wasmtime::ValType),

    /// Unsupported result type.
    #[error("unsupported function result type: {0:?}")]
    UnsupportedFunctionResultType(nacelle::wasmtime::ValType),

    /// Expected an I32.
    #[error("expected an i32 argument but got: {0}")]
    ExpectedAnI32Argument(String),

    /// Expected an I64.
    #[error("expected an i64 argument but got: {0}")]
    ExpectedAnI64Argument(String),

    /// Expected an F32.
    #[error("expected an f32 argument but got: {0}")]
    ExpectedAnF32Argument(String),

    /// Expected an F64.
    #[error("expected an f64 argument but got: {0}")]
    ExpectedAnF64Argument(String),

    /// An I/O error.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
