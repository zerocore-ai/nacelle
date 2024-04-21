use wasmtime::component::ResourceTable;

//--------------------------------------------------------------------------------------------------
// Traits
//--------------------------------------------------------------------------------------------------

/// Trait for providing the different components that make up the WASI state.
pub trait WasiState {
    /// Returns the resource table.
    fn table(&self) -> &ResourceTable;

    /// Returns a mutable reference to the resource table.
    fn table_mut(&mut self) -> &mut ResourceTable;

    /// Returns the environment variables.
    fn env(&self) -> &Vec<(String, String)>;
}

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// This is the WASI context that gets passed to the host implementation.
pub struct WasiContext {
    table: ResourceTable,
    env: Vec<(String, String)>,
    // preopens: Vec<(Dir, String)>,
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl WasiState for WasiContext {
    fn table(&self) -> &ResourceTable {
        &self.table
    }

    fn table_mut(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn env(&self) -> &Vec<(String, String)> {
        &self.env
    }
}
