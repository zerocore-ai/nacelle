use wasmtime::component::Linker;

use crate::ctx::WasiState;

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
