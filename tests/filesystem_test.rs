use std::fs;
use std::io;
use std::os::linux::fs::MetadataExt;

#[test]
fn test_linux_file_metadata() -> io::Result<()> {
    let meta = fs::metadata("tests/filesystem_test.rs")?;
    println!("{:o}", meta.st_mode());
    Ok(())
}
