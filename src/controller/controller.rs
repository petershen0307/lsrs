pub fn run(flags: crate::ui_cli::flags::LsFlags){
    match crate::file_system::list::list(flags) {
        Err(e) => {
            print!("got some error={}", e);
        }
        Ok(output) => {
            output.print();
        }
    }
}