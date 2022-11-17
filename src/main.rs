pub mod actions;
pub mod models;

fn main() {
    let flags: crate::models::flags::LsFlags = argh::from_env();
    let _ = crate::actions::list::list(flags);
}
