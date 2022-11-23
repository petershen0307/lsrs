use std::fs;

pub fn list(
    flags: crate::models::flags::LsFlags,
) -> std::io::Result<crate::models::result::Result> {
    let fs_metadata = fs::metadata(&flags.path)?;
    let mut output = crate::models::result::Result::new();
    if fs_metadata.is_dir() {
        let dir = fs::read_dir(&flags.path).unwrap();
        for path in dir {
            match path.unwrap().path().into_os_string().into_string() {
                Ok(string) => {
                    output.push(string)
                }
                Err(_) => {}
            }
        }
    } else {
        output.push(flags.path)
    }
    Ok(output)
}
