use users::{get_group_by_gid, get_user_by_uid};
use chrono::prelude::NaiveDateTime;

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
    pub fn get_permission_string(&self)->String{
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
            None => "".to_string()
        }
    }
}

// ls -l file attribute
// Permissions  Number of links     Owner   Group   Size    Modified        Name
// -rwxrwxrwx   1                   peter   peter   2058    Nov 17 00:33    Cargo.lock
