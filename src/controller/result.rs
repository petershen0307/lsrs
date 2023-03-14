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

    pub fn get_all_names(&self) -> Vec<String> {
        let mut names: Vec<String> = Vec::new();
        for i in &self.files_and_folders {
            names.push(i.name.clone());
        }
        names
    }

    pub fn get_long_output(&self) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        for i in &self.files_and_folders {
            out.push(i.to_string());
        }
        out
    }
}
