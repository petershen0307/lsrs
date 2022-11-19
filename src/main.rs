pub mod actions;
pub mod models;

fn main() {
    let flags: crate::models::flags::LsFlags = argh::from_env();
    match crate::actions::list::list(flags) {
        Err(e) => {
            print!("got some error={}", e);
        }
        Ok(_) => {}
    }
}
