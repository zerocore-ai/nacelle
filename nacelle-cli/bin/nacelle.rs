use clap::{CommandFactory, Parser};
use nacelle::{
    wasmtime::{Func, Store, Val, ValType},
    FsPermissions, Nacelle, Permissions,
};
use nacelle_cli::{NacelleArgs, NacelleCliError, Result, SubCommand};
use std::fs;

//--------------------------------------------------------------------------------------------------
// Main
//--------------------------------------------------------------------------------------------------

fn main() -> Result<()> {
    // Parse CLI arguments.
    let args = NacelleArgs::parse();

    // Run the subcommand.
    match args.subcommand {
        Some(SubCommand::Run {
            invoke,
            args,
            path,
            dir,
            base,
        }) => {
            let runtime = if let Some(base) = base {
                Nacelle::with_base(base.into())?
            } else {
                Nacelle::default()
            };

            // Compile module.
            let module = runtime.compile_from_file(path)?;

            // Prepare permissions.
            let perms = prepare_permissions(&dir)?;

            // Prepare instance.
            let (mut store, instance) = runtime.prepare_instance(&module, &perms, None)?;

            // Get the function to invoke.
            let func = instance
                .get_func(&mut store, &invoke)
                .ok_or(NacelleCliError::FunctionNotExported(invoke))?;

            // Convert args to values
            let params = convert_args_to_values(&args, &func, &store)?;
            let mut results = create_expected_results(&func, &store)?;

            // Call function
            func.call(&mut store, &params, &mut results)?;

            // Print results
            print_results(&results)?;
        }
        None => NacelleArgs::command().print_help()?,
    }

    Ok(())
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Prepares the permissions.
fn prepare_permissions(dir: &[String]) -> Result<Permissions> {
    let mut fs_perms_builder = FsPermissions::builder();

    for dir in dir {
        let dir = fs::canonicalize(dir)?;
        fs_perms_builder = fs_perms_builder.directory(dir);
    }

    let fs_perms = fs_perms_builder.build();

    Ok(Permissions::builder().fs(fs_perms).build())
}

/// Converts the arguments to values.
fn convert_args_to_values<T>(args: &[String], func: &Func, store: &Store<T>) -> Result<Vec<Val>> {
    // First get the param types of the function.
    let param_types = func.ty(store).params().collect::<Vec<_>>();

    // Then convert the args to values.
    let values = args
        .iter()
        .zip(param_types.iter())
        .map(|(arg, param_type)| convert_string_to_val(arg, param_type))
        .collect::<Result<Vec<_>>>()?;

    Ok(values)
}

/// Converts a string to a value.
fn convert_string_to_val(string: &str, val_type: &ValType) -> Result<Val> {
    match val_type {
        ValType::I32 => {
            Ok(Val::I32(string.parse::<i32>().map_err(|_| {
                NacelleCliError::ExpectedAnI32Argument(string.to_string())
            })?))
        }
        ValType::I64 => {
            Ok(Val::I64(string.parse::<i64>().map_err(|_| {
                NacelleCliError::ExpectedAnI64Argument(string.to_string())
            })?))
        }
        ValType::F32 => Ok(Val::F32(f32::to_bits(string.parse::<f32>().map_err(
            |_| NacelleCliError::ExpectedAnF32Argument(string.to_string()),
        )?))),
        ValType::F64 => Ok(Val::F64(f64::to_bits(string.parse::<f64>().map_err(
            |_| NacelleCliError::ExpectedAnF64Argument(string.to_string()),
        )?))),
        _ => Err(NacelleCliError::UnsupportedFunctionParamType(
            val_type.clone(),
        )),
    }
}

/// Creates the expected results of the function call.
fn create_expected_results<T>(func: &Func, store: &Store<T>) -> Result<Vec<Val>> {
    let result_types = func.ty(store).results().collect::<Vec<_>>();

    result_types
        .iter()
        .map(create_expected_result)
        .collect::<Result<Vec<_>>>()
}

/// Creates the expected result of the function call.
fn create_expected_result(val_type: &ValType) -> Result<Val> {
    match val_type {
        ValType::I32 => Ok(Val::I32(0)),
        ValType::I64 => Ok(Val::I64(0)),
        ValType::F32 => Ok(Val::F32(0)),
        ValType::F64 => Ok(Val::F64(0)),
        _ => Err(NacelleCliError::UnsupportedFunctionResultType(
            val_type.clone(),
        )),
    }
}

/// Prints the results of the function call side by side.
fn print_results(results: &[Val]) -> Result<()> {
    let mut first = true;

    for result in results {
        if !first {
            print!(" ");
        }

        first = false;

        match result {
            Val::I32(i) => print!("{}", i),
            Val::I64(i) => print!("{}", i),
            Val::F32(f) => print!("{}", f32::from_bits(*f)),
            Val::F64(f) => print!("{}", f64::from_bits(*f)),
            _ => unreachable!(),
        }
    }

    println!();

    Ok(())
}
