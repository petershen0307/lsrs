use std::fs;

pub fn list(
    flags: crate::models::flags::LsFlags,
) -> std::io::Result<crate::models::result::Result> {
    let entry_path = if flags.path == String::from(".") {
        crate::models::flags::get_current_dir()
    } else {
        flags.path
    };
    let fs_metadata = fs::metadata(&entry_path)?;
    let mut output = crate::models::result::Result::new();
    if fs_metadata.is_dir() {
        let dir = fs::read_dir(&entry_path).unwrap();
        for path in dir {
            match path.unwrap().file_name().into_string() {
                Ok(string) => {
                    if flags.all || !string.starts_with(".") {
                        output.push(string)
                    }
                }
                Err(_) => {}
            }
        }
    } else {
        output.push(entry_path)
    }
    Ok(output)
}
