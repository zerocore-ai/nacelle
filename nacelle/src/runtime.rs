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
    _fs_base: Option<PathBuf>,
    _distributed_fs: bool,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Nacelle {
    /// Creates a new Zerocore runtime.
    pub fn with_base(fs_base: PathBuf) -> Result<Self> {
        // Canonicalize the system base if requested.
        let fs_base = Some(fs_base.canonicalize()?);

        Ok(Self {
            engine: Engine::default(),
            _fs_base: fs_base,
            _distributed_fs: false, // TODO(appcypher): Support distributed fs. And should not be optional.
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
        // // Create linker.
        // let mut linker = Linker::new(&self.engine);

        // // Add the WASI functions to the linker.
        // nacelle_wasi::add_to_linker(&mut linker);

        // // Prepare the WASI context.
        // let wasi_ctx = self.prepare_wasi_context()?;

        // // Create a store based on the engine and the WASI context.
        // let mut store = Store::new(&self.engine, wasi_ctx);

        // // Instantiate the module.
        // let instance = linker.instantiate(&mut store, module)?;

        // Ok((store, instance))

        todo!("Prepare the instance for execution")
    }

    fn _prepare_wasi_context(&self) -> Result<WasiContext> {
        todo!("Prepare the WASI context")
    }

    // /// Prepares the context for running modules.
    // /// TODO: To be removed.
    // fn prepare_context(
    //     &self,
    //     permissions: &Permissions,
    //     partition: Option<&Path>,
    // ) -> Result<WasiCtx> {
    //     // Create the WASI contex.
    //     let mut wasi_ctx_builder = WasiCtxBuilder::new();

    //     // Pre-open the permissioned directories.
    //     if !permissions.fs.directories.is_empty() {
    //         for permissioned_dir in &permissions.fs.directories {
    //             // If system base is provided, join, otherwise canonicalize dir.
    //             let permissioned_dir = if let Some(fs_base) = &self.fs_base {
    //                 // If partition is provided, join and canonicalize.
    //                 let final_base = if let Some(partition) = partition {
    //                     fs_base.join(partition).canonicalize()?
    //                 } else {
    //                     fs_base.clone()
    //                 };

    //                 // Important: We still need to canonicalize the join to make sure it is not escaping
    //                 // the system base.
    //                 let canon_permissioned_dir =
    //                     final_base.join(permissioned_dir).canonicalize()?;
    //                 if canon_permissioned_dir.starts_with(&final_base) {
    //                     canon_permissioned_dir
    //                 } else {
    //                     return Err(NacelleError::PermissionedDirectoryEscape(
    //                         permissioned_dir.clone(),
    //                         final_base,
    //                     ));
    //                 }
    //             } else {
    //                 permissioned_dir.canonicalize()?
    //             };

    //             // Pre-open the directory.
    //             let permissioned_dir =
    //                 Dir::open_ambient_dir(&permissioned_dir, ambient_authority())?;

    //             // Add the directory to the WASI context.
    //             wasi_ctx_builder.preopened_dir(permissioned_dir, "/")?;
    //         }
    //     }

    //     Ok(wasi_ctx_builder.build())
    // }
}
