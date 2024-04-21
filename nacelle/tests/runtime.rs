use anyhow::Ok;
use nacelle::{FsPermissions, Nacelle, Permissions};

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[test]
fn test_basic_runtime() -> anyhow::Result<()> {
    let runtime = Nacelle::default();
    let file = helper::fixture_path("add.wat");
    let module = runtime.compile_from_file(&file)?;

    let (mut store, instance) = runtime.prepare_instance(&module, &Permissions::default(), None)?;

    let add_fn = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add")?;

    assert_eq!(add_fn.call(&mut store, (20, 35))?, 55);

    Ok(())
}

#[test]
fn test_wasi_fs_permission() -> anyhow::Result<()> {
    let tempdir = tempfile::tempdir()?;
    helper::create_dir(&tempdir, "archives")?;
    helper::create_file(&tempdir, "archives/foo.txt", b"lorem ipsum dolor sit amet")?;

    let runtime = Nacelle::with_base(tempdir.path().to_path_buf())?;
    let file = helper::fixture_path("copy_file.wat");
    let module = runtime.compile_from_file(&file)?;

    let permissions = Permissions::builder()
        .fs(FsPermissions::builder().directory(".".into()).build())
        .build();

    let (mut store, instance) = runtime.prepare_instance(&module, &permissions, None)?;

    let copy_file_fn = instance.get_typed_func::<(), i32>(&mut store, "copy_file")?;

    let errno = copy_file_fn.call(&mut store, ())?;

    assert_eq!(errno, 0);
    assert_eq!(
        helper::read_file(&tempdir, "archives/bar.txt")?,
        "lorem ipsum dolor sit amet"
    );

    Ok(())
}

//--------------------------------------------------------------------------------------------------
// Utilities
//--------------------------------------------------------------------------------------------------

mod helper {
    use std::{
        fs::{self, File},
        io::Write,
        path::PathBuf,
    };

    use tempfile::TempDir;

    pub(super) fn fixture_path(name: &str) -> PathBuf {
        PathBuf::from(format!(
            "{}/tests/fixtures/{}",
            env!("CARGO_MANIFEST_DIR"),
            name
        ))
    }

    pub(super) fn create_dir(tempdir: &TempDir, path: &str) -> anyhow::Result<()> {
        let dest = tempdir.path().join(path);
        fs::create_dir_all(&dest)?;
        Ok(())
    }

    pub(super) fn create_file(
        tempdir: &TempDir,
        path: &str,
        contents: &[u8],
    ) -> anyhow::Result<File> {
        let dest = tempdir.path().join(path);
        let mut file = File::create(&dest)?;
        file.write_all(contents)?;
        Ok(file)
    }

    pub(super) fn read_file(tempdir: &TempDir, path: &str) -> anyhow::Result<String> {
        let dest = tempdir.path().join(path);
        fs::read_to_string(&dest).map_err(Into::into)
    }
}
