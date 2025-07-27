use findgrep::{FindGrepConfig, findgrep};
use std::fs;
use std::io;
use tempfile::TempDir;

static DEFAULT_DIRS: &[&str] = &["one/two/three", "one/two/three/directory_foo"];

static DEFAULT_FILES: &[&str] = &[
    "a.foo",
    "one/b.foo",
    "one/two/c.foo",
    "one/two/C.Foo2",
    "one/two/three/d.foo",
    "foo.foo",
    "foo.bar",
    "e1 e2",
];

#[cfg(test)]
fn create_working_directory(
    directories: &[&'static str],
    files: &[&'static str],
) -> Result<TempDir, io::Error> {
    let temp_dir = tempfile::Builder::new().prefix("fd-tests").tempdir()?;

    {
        let root = temp_dir.path();

        for directory in directories {
            fs::create_dir_all(root.join(directory))?;
        }

        for file in files {
            fs::File::create(root.join(file))?;
        }
    }

    Ok(temp_dir)
}

#[test]
fn default() {
    let temp_dir =
        create_working_directory(DEFAULT_DIRS, DEFAULT_FILES).expect("working directory");
    let result = findgrep(
        temp_dir.path().display().to_string(),
        FindGrepConfig::default(),
    )
    .expect("no errors");
    assert_eq!(result.len(), DEFAULT_FILES.len());
}
