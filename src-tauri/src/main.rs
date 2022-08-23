#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{io::{BufWriter, Write}, fs::{File, OpenOptions}};

use registry::{Hive, Security, Data};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_dword, change_dword])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn get_dword(hkey: &str, path: &str, key: &str) -> u32 {
  let hive;

  match hkey {
    "HKEY_CURRENT_USER" => hive = Hive::CurrentUser,
    _ => unreachable!()
  }

  let regkey = hive.open(path, Security::Read).unwrap();
  let data = regkey.value(key).unwrap();
  let s: String = data.to_string().drain(2..).collect();
  return s.parse().unwrap();
}

#[tauri::command]
fn change_dword(hkey: &str, path: &str, key: &str, value: u32) {
  let hive;

  match hkey {
    "HKEY_CURRENT_USER" => hive = Hive::CurrentUser,
    _ => unreachable!()
  }

  let regkey = hive.open(path, Security::Write).unwrap();
  regkey.set_value(key, &Data::U32(value)).unwrap();
}
