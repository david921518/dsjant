// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
fn main() {
    // this method needs to be inside main() method
    env::set_var("RUST_BACKTRACE", "1");
    dsjant_lib::run()
}
