use argh::FromArgs;

#[derive(FromArgs)]
/// List information about the FILEs (the current directory by default).
/// ls [OPTION]... [FILE]...
pub struct LsFlags {
    /// enter directory or file path
    #[argh(option, default = "get_current_dir()")]
    pub path: String,
    /// do not ignore entries starting with .
    #[argh(switch, short = 'a')]
    pub all: bool,
    /// show the long format
    #[argh(switch, short = 'l')]
    pub long: bool,
}

pub fn get_current_dir() -> String {
    let dir = std::env::current_dir();
    let mut cwd: String = String::from("");
    match dir {
        Ok(pwd) => match pwd.into_os_string().into_string() {
            Ok(string) => {
                cwd = string;
            }
            Err(_) => {}
        },
        Err(_) => {}
    }
    return cwd;
}
