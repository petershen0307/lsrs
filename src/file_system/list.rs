use std::fs;
use std::os::linux::fs::MetadataExt;
use crate::file_system::file_attribute::FileAttribute;

pub fn list_all_file_attributes(
    entry_path: String,
) -> std::io::Result<crate::controller::result::Result> {
    let fs_metadata = fs::metadata(&entry_path)?;
    let mut output = crate::controller::result::Result::new();
    if fs_metadata.is_dir() {
        let dir = fs::read_dir(&entry_path).unwrap();
        for path in dir {
            match path.unwrap().file_name().into_string() {
                Ok(file_str) => {
                    let metadata = fs::metadata(&file_str)?;
                    output.push(FileAttribute {
                        name: file_str,
                        st_mode: metadata.st_mode(),
                        owner_uid: metadata.st_uid(),
                        group_id: metadata.st_gid(),
                        number_link: metadata.st_nlink(),
                        size_in_bytes: metadata.st_size(),
                        modified_secs: metadata.st_atime(),
                    });
                },
                Err(_) => {}
            }
        }
    }

    output.push(FileAttribute {
        name: entry_path,
        st_mode: fs_metadata.st_mode(),
        owner_uid: fs_metadata.st_uid(),
        group_id: fs_metadata.st_gid(),
        number_link: fs_metadata.st_nlink(),
        size_in_bytes: fs_metadata.st_size(),
        modified_secs: fs_metadata.st_atime(),
    });
    Ok(output)
}
