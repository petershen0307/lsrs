use chrono::prelude::NaiveDateTime;
use users::{get_group_by_gid, get_user_by_uid};

pub struct FileAttribute {
    pub st_mode: u32,
    pub number_link: u64,
    pub owner_uid: u32,
    pub group_id: u32,
    pub size_in_bytes: u64,
    pub modified_secs: i64,
    pub name: String,
}

impl Default for FileAttribute {
    fn default() -> Self {
        FileAttribute {
            st_mode: 0,
            number_link: 0,
            owner_uid: 0,
            group_id: 0,
            size_in_bytes: 0,
            modified_secs: 0,
            name: String::from(""),
        }
    }
}

impl FileAttribute {
    pub fn permission_to_string_per_bit(permission: libc::mode_t) -> Option<String> {
        if permission < libc::S_IXOTH || permission > libc::S_IRWXU {
            return Some(String::from("-"));
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
        let mut result_str = String::from("");
        /*
        https://linuxize.com/post/how-to-list-files-in-linux-using-the-ls-command/
        S_IFMT
        - - Regular file.
        b - Block special file.
        c - Character special file.
        d - Directory.
        l - Symbolic link.
        n - Network file.
        p - FIFO.
        s - Socket.
        */
        match self.st_mode & libc::S_IFMT {
            libc::S_IFLNK => {
                result_str.push_str("1");
            }
            libc::S_IFREG => {
                result_str.push_str("-");
            }
            libc::S_IFDIR => {
                result_str.push_str("d");
            }
            libc::S_IFCHR => {
                result_str.push_str("c");
            }
            libc::S_IFBLK => {
                result_str.push_str("b");
            }
            libc::S_IFIFO => {
                result_str.push_str("p");
            }
            libc::S_IFSOCK => {
                result_str.push_str("s");
            }
            _ => {}
        }
        let mut mask: u32 = libc::S_IRUSR;
        while mask > 0 {
            let m = FileAttribute::permission_to_string_per_bit(self.st_mode & mask);
            result_str.push_str(&m.unwrap());
            mask >>= 1;
        }

        result_str
    }
    pub fn get_user_name(&self) -> String {
        get_user_by_uid(self.owner_uid)
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

impl std::fmt::Display for FileAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // ls -l file attribute
        // Permissions  Number of links     Owner   Group   Size    Modified        Name
        // -rwxrwxrwx   1                   peter   peter   2058    Nov 17 00:33    Cargo.lock
        write!(
            f,
            "{}, {}, {}, {}, {}, {:19}, {}",
            self.get_permission_string(),
            self.number_link,
            self.get_user_name(),
            self.get_group_name(),
            self.size_in_bytes,
            self.get_modified_time_string(),
            self.name
        )
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
        let result = FileAttribute::permission_to_string_per_bit(libc::S_IXOTH);
        // assert
        assert_eq!(expected, result.unwrap());
    }
    #[test]
    fn test_s_iwgrp_to_string() {
        // arrange
        let expected = String::from("w");
        // act
        let result = FileAttribute::permission_to_string_per_bit(libc::S_IWGRP);
        // assert
        assert_eq!(expected, result.unwrap());
    }
    #[test]
    fn test_s_irusr_to_string() {
        // arrange
        let expected = String::from("r");
        // act
        let result = FileAttribute::permission_to_string_per_bit(libc::S_IRUSR);
        // assert
        assert_eq!(expected, result.unwrap());
    }
    #[test]
    fn test_s_irwxu_to_string() {
        // arrange
        let expected = None;
        // act
        let result = FileAttribute::permission_to_string_per_bit(libc::S_IRWXU);
        // assert
        assert_eq!(expected, result);
    }
}

#[cfg(test)]
mod test_get_permission_string {
    use super::*;
    #[test]
    fn test_0o777() {
        // arrange
        let f = FileAttribute {
            st_mode: libc::S_IFREG | libc::S_IRWXO | libc::S_IRWXG | libc::S_IRWXU,
            ..Default::default()
        };
        let expected = String::from("-rwxrwxrwx");
        // act
        let output = f.get_permission_string();
        // assert
        assert_eq!(expected, output);
    }
    #[test]
    fn test_0o644() {
        // arrange
        let f = FileAttribute {
            st_mode: libc::S_IFREG | libc::S_IRUSR | libc::S_IWUSR | libc::S_IRGRP | libc::S_IROTH,
            ..Default::default()
        };
        let expected = String::from("-rw-r--r--");
        // act
        let output = f.get_permission_string();
        // assert
        assert_eq!(expected, output);
    }
}
