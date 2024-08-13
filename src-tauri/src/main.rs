#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod db;
pub mod models;
pub mod schema;
pub mod commands;

use crate::commands::{get_all_assignments, submit_assignment, get_unsubmitted_assignments};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_all_assignments, submit_assignment, get_unsubmitted_assignments])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}