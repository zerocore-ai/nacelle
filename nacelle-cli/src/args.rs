use crate::styles;
use clap::Parser;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Arguments for managing the nacelle serverless engine.
#[derive(Debug, Parser)]
#[command(name = "nacelle", author, about, version, styles=styles::styles())]
pub struct NacelleArgs {
    /// The subcommand to run.
    #[command(subcommand)]
    pub subcommand: Option<SubCommand>,
}

/// Nacelle has many functionalities. These subcommands lets you use the different functions of the engines
#[derive(Debug, Parser)]
pub enum SubCommand {
    /// Executes a WebAssembly module. Supports both binary and text formats.
    Run {
        /// The function to invoke.
        #[arg(short, long)]
        invoke: String,

        /// The arguments to pass to the invoked function.
        #[arg(short, long, use_value_delimiter = true, value_delimiter = ',')]
        args: Vec<String>,

        /// WASI pre-opened directory.
        #[arg(short, long)]
        dir: Vec<String>,

        // TODO(appcypher):
        /// The base path for permitted directories.
        #[arg(short, long, default_value = None)]
        base: Option<String>,

        /// The path to the WebAssembly module.
        path: String,
    },
}
