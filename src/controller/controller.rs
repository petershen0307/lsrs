pub fn run(flags: crate::ui_cli::flags::LsFlags) {
    match crate::file_system::list::list_all_file_attributes(flags.path) {
        Err(e) => {
            print!("got some error={}", e);
        }
        Ok(output) => {
            let contents = output.get_contents();
            let mut result_str: Vec<String> = Vec::new();

            for i in contents {
                if skip_hidden_file(&i.name, flags.all) {
                    // skip hidden file
                    continue;
                }
                if flags.long {
                    result_str.push(i.to_string());
                } else {
                    result_str.push(i.name.clone());
                }
            }
            if flags.long {
                println!("{}", result_str.join("\n"));
            } else {
                println!("{}", result_str.join(" "));
            }
        }
    }
}

fn skip_hidden_file(file_name: &String, flag_all: bool) -> bool {
    !flag_all && file_name.starts_with(".") && file_name.len() > 1
}
