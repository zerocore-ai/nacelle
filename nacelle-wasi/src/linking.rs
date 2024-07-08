use wasmtime::component::Linker;

use crate::WasiState;

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// TODO: Document
pub fn add_to_linker<S>(_linker: &mut Linker<S>)
where
    S: WasiState,
{
    todo!("Add WASI imports to the linker")
}
