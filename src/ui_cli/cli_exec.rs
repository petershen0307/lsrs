pub fn run() {
    let flags: crate::ui_cli::flags::LsFlags = argh::from_env();
    crate::controller::controller::run(flags);
}
