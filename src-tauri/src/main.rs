// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]
#![allow(warnings)]
fn main() {
    ybkauto_lib::run()
}
