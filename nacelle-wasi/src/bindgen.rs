//! This module contains the generated bindings for the WASI API.

//--------------------------------------------------------------------------------------------------
// Modules
//--------------------------------------------------------------------------------------------------

mod generated {
    wasmtime::component::bindgen!({
        path: "../wit/wasi",
        world: "nacelle:wasi/imports@0.1.0",
        tracing: true,
        // trappable_error_type: {
        //      "wasi:io/streams/stream-error" => StreamError,
        //      "wasi:filesystem/types/error-code" => FsError,
        // },
        async: {
            // These are the only methods and functions that are async,
            // all other methods are synchronous.
            // TODO: Add wasi:http async methods
            only_imports: [
                "[method]descriptor.access-at",
                "[method]descriptor.advise",
                "[method]descriptor.change-directory-permissions-at",
                "[method]descriptor.change-file-permissions-at",
                "[method]descriptor.create-directory-at",
                "[method]descriptor.get-flags",
                "[method]descriptor.get-type",
                "[method]descriptor.is-same-object",
                "[method]descriptor.link-at",
                "[method]descriptor.lock-exclusive",
                "[method]descriptor.lock-shared",
                "[method]descriptor.metadata-hash",
                "[method]descriptor.metadata-hash-at",
                "[method]descriptor.open-at",
                "[method]descriptor.read",
                "[method]descriptor.read-directory",
                "[method]descriptor.readlink-at",
                "[method]descriptor.remove-directory-at",
                "[method]descriptor.rename-at",
                "[method]descriptor.set-size",
                "[method]descriptor.set-times",
                "[method]descriptor.set-times-at",
                "[method]descriptor.stat",
                "[method]descriptor.stat-at",
                "[method]descriptor.symlink-at",
                "[method]descriptor.sync",
                "[method]descriptor.sync-data",
                "[method]descriptor.try-lock-exclusive",
                "[method]descriptor.try-lock-shared",
                "[method]descriptor.unlink-file-at",
                "[method]descriptor.unlock",
                "[method]descriptor.write",
                "[method]input-stream.read",
                "[method]input-stream.blocking-read",
                "[method]input-stream.blocking-skip",
                "[method]input-stream.skip",
                "[method]output-stream.forward",
                "[method]output-stream.splice",
                "[method]output-stream.blocking-splice",
                "[method]output-stream.blocking-flush",
                "[method]output-stream.blocking-write",
                "[method]output-stream.blocking-write-and-flush",
                "[method]output-stream.blocking-write-zeroes-and-flush",
                "[method]directory-entry-stream.read-directory-entry",
                "poll",
                "[method]pollable.block",
                "[method]pollable.ready",
            ]
        },
        with: {
            // // === Filesystem ===
            // "wasi:filesystem/types/directory-entry-stream": crate::filesystem::DirectoryEntryStream,
            // "wasi:filesystem/types/descriptor": crate::filesystem::Descriptor,
            // // === IO ===
            // "wasi:io/streams/input-stream": crate::stream::InputStream,
            // "wasi:io/streams/output-stream": crate::stream::OutputStream,
            // "wasi:io/error/error": crate::stream::Error,
            // "wasi:io/poll/pollable": crate::poll::Pollable,
        }
    });
}
