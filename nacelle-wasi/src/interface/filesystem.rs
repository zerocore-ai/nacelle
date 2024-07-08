//! `wasi:filesystem` implementation for Nacelle WASI runtime.

use async_trait::async_trait;
use wasmtime::component::Resource;
use zerofs::filesystem::{Entity, FsResult};
use zeroutils_store::MemoryStore;
use zeroutils_wasi::io::{InputStreamHandle, OutputStreamHandle};

use crate::{bindgen::filesystem::types, WasiState};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

pub type DiskEntity = Entity<MemoryStore>; // TODO: Change to IPC interface.

pub struct DirectoryEntryStream {} // TODO

pub struct DescriptorHandle(DiskEntity); // TODO

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

#[async_trait]
impl<T> types::HostDescriptor for T
where
    T: WasiState,
{
    async fn open_at(
        &mut self,
        descriptor: Resource<DescriptorHandle>,
        path_flags: types::PathFlags,
        path: String,
        open_flags: types::OpenFlags,
        flags: types::DescriptorFlags,
    ) -> FsResult<Resource<DescriptorHandle>> {
        todo!("open_at")
    }

    fn read_via_stream(
        &mut self,
        descriptor: Resource<DescriptorHandle>,
        offset: types::Filesize,
    ) -> FsResult<Resource<InputStreamHandle>> {
        todo!()
    }

    fn write_via_stream(
        &mut self,
        descriptor: Resource<DescriptorHandle>,
        offset: types::Filesize,
    ) -> FsResult<Resource<OutputStreamHandle>> {
        todo!()
    }

    fn append_via_stream(
        &mut self,
        descriptor: Resource<DescriptorHandle>,
    ) -> FsResult<Resource<OutputStreamHandle>> {
        todo!()
    }

    async fn advise(
        &mut self,
        descriptor: Resource<DescriptorHandle>,
        offset: types::Filesize,
        length: types::Filesize,
        advice: types::Advice,
    ) -> FsResult<()> {
        todo!()
    }

    async fn sync_data(&mut self, descriptor: Resource<DescriptorHandle>) -> FsResult<()> {
        todo!()
    }

    async fn get_flags(
        &mut self,
        descriptor: Resource<DescriptorHandle>,
    ) -> FsResult<types::DescriptorFlags> {
        todo!()
    }

    async fn get_type(
        &mut self,
        descriptor: Resource<DescriptorHandle>,
    ) -> FsResult<types::DescriptorType> {
        todo!()
    }

    async fn set_size(
        &mut self,
        descriptor: Resource<DescriptorHandle>,
        size: types::Filesize,
    ) -> FsResult<()> {
        todo!()
    }

    async fn set_times(
        &mut self,
        descriptor: Resource<DescriptorHandle>,
        data_access_timestamp: types::NewTimestamp,
        data_modification_timestamp: types::NewTimestamp,
    ) -> FsResult<()> {
        todo!()
    }

    async fn read(
        &mut self,
        descriptor: Resource<DescriptorHandle>,
        length: types::Filesize,
        offset: types::Filesize,
    ) -> FsResult<(Vec<u8>, bool)> {
        todo!()
    }

    async fn write(
        &mut self,
        descriptor: Resource<DescriptorHandle>,
        buffer: Vec<u8>,
        offset: types::Filesize,
    ) -> FsResult<types::Filesize> {
        todo!()
    }

    async fn read_directory(
        &mut self,
        descriptor: Resource<DescriptorHandle>,
    ) -> FsResult<Resource<DirectoryEntryStream>> {
        todo!()
    }

    async fn sync(&mut self, descriptor: Resource<DescriptorHandle>) -> FsResult<()> {
        todo!()
    }

    async fn create_directory_at(
        &mut self,
        descriptor: Resource<DescriptorHandle>,
        path: String,
    ) -> FsResult<()> {
        todo!()
    }

    async fn stat(
        &mut self,
        descriptor: Resource<DescriptorHandle>,
    ) -> FsResult<types::DescriptorStat> {
        todo!()
    }

    async fn stat_at(
        &mut self,
        descriptor: Resource<DescriptorHandle>,
        path_flags: types::PathFlags,
        path: String,
    ) -> FsResult<types::DescriptorStat> {
        todo!()
    }

    async fn set_times_at(
        &mut self,
        descriptor: Resource<DescriptorHandle>,
        path_flags: types::PathFlags,
        path: String,
        data_access_timestamp: types::NewTimestamp,
        data_modification_timestamp: types::NewTimestamp,
    ) -> FsResult<()> {
        todo!()
    }

    async fn link_at(
        &mut self,
        descriptor: Resource<DescriptorHandle>,
        old_path_flags: types::PathFlags,
        old_path: String,
        new_descriptor: Resource<DescriptorHandle>,
        new_path: String,
    ) -> FsResult<()> {
        todo!()
    }

    async fn readlink_at(
        &mut self,
        descriptor: Resource<DescriptorHandle>,
        path: String,
    ) -> FsResult<String> {
        todo!()
    }

    async fn remove_directory_at(
        &mut self,
        descriptor: Resource<DescriptorHandle>,
        path: String,
    ) -> FsResult<()> {
        todo!()
    }

    async fn rename_at(
        &mut self,
        descriptor: Resource<DescriptorHandle>,
        old_path: String,
        new_descriptor: Resource<DescriptorHandle>,
        new_path: String,
    ) -> FsResult<()> {
        todo!()
    }

    async fn symlink_at(
        &mut self,
        descriptor: Resource<DescriptorHandle>,
        old_path: String,
        new_path: String,
    ) -> FsResult<()> {
        todo!()
    }

    async fn unlink_file_at(
        &mut self,
        descriptor: Resource<DescriptorHandle>,
        path: String,
    ) -> FsResult<()> {
        todo!()
    }

    async fn is_same_object(
        &mut self,
        descriptor: Resource<DescriptorHandle>,
        other: Resource<DescriptorHandle>,
    ) -> wasmtime::Result<bool> {
        todo!()
    }

    async fn metadata_hash(
        &mut self,
        descriptor: Resource<DescriptorHandle>,
    ) -> FsResult<types::MetadataHashValue> {
        todo!()
    }

    async fn metadata_hash_at(
        &mut self,
        descriptor: Resource<DescriptorHandle>,
        path_flags: types::PathFlags,
        path: String,
    ) -> FsResult<types::MetadataHashValue> {
        todo!()
    }

    fn drop(&mut self, rep: Resource<DescriptorHandle>) -> wasmtime::Result<()> {
        todo!()
    }
}

// impl<T> types::HostDirectoryEntryStream for T where T: WasiContext {}

// impl<T> types::Host for T where T: WasiContext {}
