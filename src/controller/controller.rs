pub fn run(flags: crate::ui_cli::flags::LsFlags){
    match crate::file_system::list::list_all_file_attributes(flags.path) {
        Err(e) => {
            print!("got some error={}", e);
        }
        Ok(output) => {
            if flags.long{
                let out=output.get_long_output();
                println!("{}", out.join("\n"))
            }else{
                let names = output.get_all_names();
                println!("{}", names.join(" "));
            }
        }
    }
}