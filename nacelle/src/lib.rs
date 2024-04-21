#![warn(missing_docs)]
//! `zerocore-runtime` is the core of the `zerocore` serverless engine. It is concerned with setting up the webassembly runtime,
//! providing necessary host functionalities to get it running properly.
//!
//! It is also a capability-based system, where the host provides capabilities to the webassembly runtime.

mod errors;
mod permissions;
mod runtime;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub use errors::*;
pub use permissions::*;
pub use runtime::*;

//--------------------------------------------------------------------------------------------------
// Re-exports
//--------------------------------------------------------------------------------------------------

pub use wasmtime;
