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
    size: u32,
    owner: String,
}
impl FileAttribute {}

// ls -l file attribute
// Permissions  Number of links     Owner   Group   Size    Modified        Name
// -rwxrwxrwx   1                   peter   peter   2058    Nov 17 00:33    Cargo.lock
