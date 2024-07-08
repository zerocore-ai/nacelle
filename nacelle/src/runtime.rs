use crate::{Permissions, Result};
use nacelle_wasi::WasiContext;
use std::path::{Path, PathBuf};
use wasmtime::{Engine, Instance, Module, Store};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

// TODO(appcypher): Support distributed fs. This uses wasi-fs with zerofs as the filesystem.
//
/// Nacelle is a wasmtime-based runtime for executing webassembly modules.
///
/// It provides the main implementations for wasm programs to interact with zerocore
/// providers and services.
#[derive(Default)]
pub struct Nacelle {
    engine: Engine,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Nacelle {
    /// Creates a new Zerocore runtime.
    pub fn new() -> Result<Self> {
        Ok(Self {
            engine: Engine::default(),
        })
    }

    /// Compiles a webassembly module from a file.
    pub fn compile_from_file(&self, path: impl AsRef<Path>) -> Result<Module> {
        let module = Module::from_file(&self.engine, path.as_ref())?;
        Ok(module)
    }

    /// Compiles a webassembly module from bytes.
    pub fn compile_from_bytes(&self, bytes: &[u8]) -> Result<Module> {
        let module = Module::from_binary(&self.engine, bytes)?;
        Ok(module)
    }

    /// Compiles a webassembly text format.
    pub fn compile_from_wat(&self, wat: &str) -> Result<Module> {
        let module = Module::new(&self.engine, wat)?;
        Ok(module)
    }

    /// Executes a webassembly module start function with the provided permissions.
    pub fn execute(
        &self,
        module: &Module,
        permissions: &Permissions,
        partition: Option<&Path>,
    ) -> Result<()> {
        let (mut store, instance) = self.prepare_instance(module, permissions, partition)?;
        let start_fn = instance.get_typed_func::<(), ()>(&mut store, "_start")?;
        Ok(start_fn.call(&mut store, ())?)
    }

    /// Prepares the instance for execution.
    pub fn prepare_instance(
        &self,
        _module: &Module,
        _permissions: &Permissions,
        _partition: Option<&Path>,
    ) -> Result<(Store<WasiContext>, Instance)> {
        todo!("Prepare the instance for execution")
    }

    fn _prepare_wasi_context(&self) -> Result<WasiContext> {
        todo!("Prepare the WASI context")
    }
}
