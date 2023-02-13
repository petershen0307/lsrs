pub mod file_system;
pub mod models;
mod ui_cli;
mod controller;

fn main() {
    crate::ui_cli::cli_exec::run();
}
