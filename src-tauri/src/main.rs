// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod data;
pub use data::Data;

use once_cell::sync::Lazy;

static mut DATA: Lazy<Data> = Lazy::new(|| { Data::new() });


fn main() {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![get, create, update, delete])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}


#[tauri::command]
fn get(a: &str, pid: &str, tid: &str) -> String {
	unsafe { DATA.get(a, pid, tid) }
}

#[tauri::command]
fn create(a: &str, pid: &str) -> String {
	unsafe { DATA.create(a, pid) }
}

#[tauri::command]
fn update(a: &str, pid: &str, tid: &str, param: &str, value: &str) {
	unsafe { DATA.update(a, pid, tid, param, value) };
}

#[tauri::command]
fn delete(a: &str, pid: &str, tid: &str) {
	unsafe { DATA.delete(a, pid, tid) };
}