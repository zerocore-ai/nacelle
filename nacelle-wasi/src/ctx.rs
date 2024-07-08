use std::vec;

use wasmtime::component::ResourceTable;
use zerofs::filesystem::Dir;
use zeroutils_store::MemoryStore;
use zeroutils_wasi::state::WasiTableState;

use crate::{WasiEnvState, WasiPreopenState, WasiState};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A collection of environment variables.
pub type EnvVars = Vec<(String, String)>;

/// A `zerofs` directory with a `MemoryStore`.
pub type DiskDir = Dir<MemoryStore>; // TODO(appcypher): To be replaced by an IPC interface.

/// This represents the necessary WASI-related state materials that is managed
/// on the host side.
pub struct WasiContext {
    table: ResourceTable,
    env: EnvVars,
    preopened_dirs: Vec<(DiskDir, String)>,
}

/// Builder for creating a WASI context.
pub struct WasiContextBuilder<E> {
    table: ResourceTable,
    env: E,
    preopened_dirs: Vec<(DiskDir, String)>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl WasiContext {
    /// Creates a builder for a WASI context.
    pub fn builder() -> WasiContextBuilder<()> {
        WasiContextBuilder {
            table: ResourceTable::default(),
            env: (),
            preopened_dirs: vec![],
        }
    }
}

impl<E> WasiContextBuilder<E> {
    /// Sets the environment variables for the WASI context.
    pub fn env(self, env: EnvVars) -> WasiContextBuilder<EnvVars> {
        WasiContextBuilder {
            env,
            table: self.table,
            preopened_dirs: self.preopened_dirs,
        }
    }

    /// Sets the file system for the WASI context.
    pub fn fs(mut self, dir: DiskDir, name: String) -> WasiContextBuilder<E> {
        self.preopened_dirs.push((dir, name));
        self
    }
}

impl WasiContextBuilder<EnvVars> {
    /// Builds the WASI context.
    pub fn build(self) -> WasiContext {
        WasiContext {
            table: self.table,
            env: self.env,
            preopened_dirs: self.preopened_dirs,
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl WasiEnvState for WasiContext {
    fn env(&self) -> impl Iterator<Item = &(String, String)> {
        self.env.iter()
    }
}

impl WasiTableState for WasiContext {
    fn table(&self) -> &ResourceTable {
        &self.table
    }

    fn table_mut(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl WasiPreopenState for WasiContext {
    fn preopen(&self) -> impl Iterator<Item = &(DiskDir, String)> {
        self.preopened_dirs.iter()
    }

    fn preopen_by_name(&self, name: &str) -> Option<&DiskDir> {
        self.preopened_dirs
            .iter()
            .find(|(_, n)| n == name)
            .map(|(dir, _)| dir)
    }
}

impl WasiState for WasiContext {}
