use argh::FromArgs;

#[derive(FromArgs)]
/// List information about the FILEs (the current directory by default).
/// ls [OPTION]... [FILE]...
struct LsFlags {
    /// enter directory or file path
    #[argh(option)]
    path: Option<String>,
    /// do not ignore entries starting with .
    #[argh(option, short = 'a')]
    all: Option<String>,
}

fn main() {
    let flags: LsFlags = argh::from_env();
}