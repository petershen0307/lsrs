pub fn run(){
    let flags: crate::models::flags::LsFlags = argh::from_env();
    match crate::file_system::list::list(flags) {
        Err(e) => {
            print!("got some error={}", e);
        }
        Ok(output) => {
            output.print();
        }
    }
}