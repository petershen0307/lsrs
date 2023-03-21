use crate::file_system::file_attribute::FileAttribute;
pub struct Result {
    files_and_folders: Vec<FileAttribute>,
}

impl Result {
    pub fn new() -> Result {
        return Result {
            files_and_folders: Vec::new(),
        };
    }

    pub fn push(&mut self, path: FileAttribute) {
        self.files_and_folders.push(path);
    }

    pub fn get_contents(&self) -> Vec<FileAttribute> {
        let temp: Vec<FileAttribute> = self.files_and_folders.clone();
        temp
    }
}
