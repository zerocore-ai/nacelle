//! This module contains the permissions system for the engine.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use structstruck::strike;
use typed_builder::TypedBuilder;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

strike! {
    /// The permissions system for the engine.
    #[strikethrough[derive(Debug, Deserialize, Serialize)]]
    #[derive(TypedBuilder, Default)]
    pub struct Permissions {
        /// The permissions for the WASI interface.
        pub fs:
            /// The permissions for the WASI interface.
            #[derive(TypedBuilder, Default)]
            #[builder(mutators(
                /// Adds allowed directory to the permissions.
                pub fn directory(&mut self, path: PathBuf) {
                    self.directories.push(path);
                }
            ))]
            pub struct FsPermissions {
                /// The directories that WASI should be allowed to access.
                #[builder(default, via_mutators)]
                pub directories: Vec<PathBuf>,
            },
    }
}
