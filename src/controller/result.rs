use crate::file_system::file_attribute::FileAttribute;
pub struct Result {
    files_and_folders: Vec<String>,
    files_and_folders2: Vec<FileAttribute>,
}

impl Result {
    pub fn new() -> Result {
        return Result {
            files_and_folders: Vec::new(),
            files_and_folders2: Vec::new(),
        };
    }

    pub fn push(&mut self, path: String) {
        self.files_and_folders.push(path);
    }

    pub fn push2(&mut self, path: FileAttribute) {
        self.files_and_folders2.push(path);
    }

    pub fn print(&self) {
        for i in &self.files_and_folders {
            print!("{} ", i);
        }
    }

    pub fn print2(&self) {
        for i in &self.files_and_folders2 {
            println!("{} ", i);
        }
    }
}
