pub fn run(flags: crate::ui_cli::flags::LsFlags){
    match crate::file_system::list::list_all_file_attributes(flags.path) {
        Err(e) => {
            print!("got some error={}", e);
        }
        Ok(output) => {
            output.print();
        }
    }
}