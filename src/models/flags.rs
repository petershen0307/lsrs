use argh::FromArgs;

#[derive(FromArgs)]
/// List information about the FILEs (the current directory by default).
/// ls [OPTION]... [FILE]...
pub struct LsFlags {
    /// enter directory or file path
    #[argh(option)]
    pub path: String,
    /// do not ignore entries starting with .
    #[argh(option, short = 'a')]
    pub all: Option<String>,
}
