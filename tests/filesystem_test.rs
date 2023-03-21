use chrono::prelude::*;
use users::get_group_by_gid;
use std::fs;
use std::io;
use std::os::linux::fs::MetadataExt;
use users::{get_user_by_uid, get_current_uid};

#[test]
fn test_linux_file_metadata() -> io::Result<()> {
    let meta = fs::metadata("tests/filesystem_test.rs")?;
    println!("file type and permission: {:o}", meta.st_mode());
    println!("number of link: {:o}", meta.st_nlink());
    println!("uid: {}", meta.st_uid());
    println!("user name: {}", get_user_by_uid(meta.st_uid()).unwrap().name().to_string_lossy());
    println!("gid: {}", meta.st_gid());
    println!("group name: {}", get_group_by_gid(meta.st_gid()).unwrap().name().to_string_lossy());
    println!("size: {}", meta.st_size());
    println!("modified: {}", meta.st_atime());
    match NaiveDateTime::from_timestamp_opt(meta.st_atime(), 0) {
        Some(format_time) => {
            println!("modified format: {}", format_time)
        }
        None => {}
    }
    Ok(())
}
