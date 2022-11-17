use std::fs;

pub fn list(flags: crate::models::flags::LsFlags) -> std::io::Result<()> {
    let fs_metadata = fs::metadata(&flags.path)?;
    if fs_metadata.is_dir() {
        let dir = fs::read_dir(&flags.path).unwrap();

        for path in dir {
            println!("Name: {}", path.unwrap().path().display())
        }
    }
    Ok(())
}
