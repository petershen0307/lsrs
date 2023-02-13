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

    pub fn print(&self){
        for i in &self.files_and_folders{
            print!("{} ", i);
        }
    }
}
