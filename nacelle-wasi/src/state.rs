use zeroutils_wasi::state::WasiTableState;

use crate::DiskDir;

//--------------------------------------------------------------------------------------------------
// Traits
//--------------------------------------------------------------------------------------------------

/// Trait for providing the environment variables of a WASI context.
pub trait WasiEnvState: Send {
    /// Returns the environment variables.
    fn env(&self) -> impl Iterator<Item = &(String, String)>;
}

/// Trait for providing preopened directories of a WASI context.
pub trait WasiPreopenState: Send {
    /// Returns the preopened directories.
    fn preopen(&self) -> impl Iterator<Item = &(DiskDir, String)>;

    /// Returns a preopened directory by name.
    fn preopen_by_name(&self, name: &str) -> Option<&DiskDir>;
}

/// Trait for providing the different components that make up the WASI state.
pub trait WasiState: WasiTableState + WasiEnvState + WasiPreopenState {}
