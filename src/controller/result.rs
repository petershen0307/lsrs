use chrono::prelude::NaiveDateTime;
use users::{get_group_by_gid, get_user_by_uid};

pub struct Result {
    files_and_folders: Vec<String>,
}

impl Result {
    pub fn new() -> Result {
        return Result {
            files_and_folders: Vec::new(),
        };
    }

    pub fn push(&mut self, path: String) {
        self.files_and_folders.push(path);
    }

    pub fn print(&self) {
        for i in &self.files_and_folders {
            print!("{} ", i);
        }
    }
}

pub struct FileAttribute {
    st_mode: u32,
    number_link: u64,
    user_id: u32,
    group_id: u32,
    size: u32,
    modified_secs: i64,
    name: String,
}

impl FileAttribute {
    pub fn permission_to_string(permission: libc::mode_t) -> Option<String> {
        if permission < libc::S_IXOTH || permission > libc::S_IRWXU {
            return None;
        }
        let mut alter_permission: u32 = permission;
        if permission & libc::S_IRWXU != 0 {
            alter_permission >>= 6;
        }
        if permission & libc::S_IRWXG != 0 {
            alter_permission >>= 3;
        }
        /*
            S_IRWXO 00007   others (not in group) have read, write, and execute permission
            S_IROTH 00004   others have read permission
            S_IWOTH 00002   others have write permission
            S_IXOTH 00001   others have execute permission
        */
        match alter_permission % 8 {
            1 => Some(String::from("x")),
            2 => Some(String::from("w")),
            4 => Some(String::from("r")),
            _ => None,
        }
    }
    pub fn get_permission_string(&self) -> String {
        String::from("")
    }
    pub fn get_user_name(&self) -> String {
        get_user_by_uid(self.user_id)
            .unwrap()
            .name()
            .to_string_lossy()
            .to_string()
    }
    pub fn get_group_name(&self) -> String {
        get_group_by_gid(self.group_id)
            .unwrap()
            .name()
            .to_string_lossy()
            .to_string()
    }
    pub fn get_modified_time_string(&self) -> String {
        match NaiveDateTime::from_timestamp_opt(self.modified_secs, 0) {
            Some(format_time) => format_time.to_string(),
            None => "".to_string(),
        }
    }
}

#[cfg(test)]
mod test_permission_to_string {
    use super::*;
    #[test]
    fn test_s_ixoth_to_string() {
        // arrange
        let expected = String::from("x");
        // act
        let result = FileAttribute::permission_to_string(libc::S_IXOTH);
        // assert
        assert_eq!(expected, result.unwrap());
    }
    #[test]
    fn test_s_iwgrp_to_string() {
        // arrange
        let expected = String::from("w");
        // act
        let result = FileAttribute::permission_to_string(libc::S_IWGRP);
        // assert
        assert_eq!(expected, result.unwrap());
    }
    #[test]
    fn test_s_irusr_to_string() {
        // arrange
        let expected = String::from("r");
        // act
        let result = FileAttribute::permission_to_string(libc::S_IRUSR);
        // assert
        assert_eq!(expected, result.unwrap());
    }
    #[test]
    fn test_s_irwxu_to_string() {
        // arrange
        let expected = None;
        // act
        let result = FileAttribute::permission_to_string(libc::S_IRWXU);
        // assert
        assert_eq!(expected, result);
    }
}
// ls -l file attribute
// Permissions  Number of links     Owner   Group   Size    Modified        Name
// -rwxrwxrwx   1                   peter   peter   2058    Nov 17 00:33    Cargo.lock
