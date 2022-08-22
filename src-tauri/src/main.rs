#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{io::{BufWriter, Write}, fs::{File, OpenOptions}};

use registry::{Hive, Security, Data};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![test, test2])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn test() {
  let file: File;
  let mut writer: BufWriter<File>;

  file = OpenOptions::new()
  .write(true)
  .append(true)
  .open("D:\\uwu.txt")
  .expect("File opening failed");
  writer = BufWriter::new(file);

  writer.write_all("123".as_bytes()).unwrap();
}

#[tauri::command]
fn test2() {
  let regkey = Hive::CurrentUser.open(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced", Security::Write).unwrap();

  regkey.set_value("ShowSecondsInSystemClock", &Data::U32(0)).unwrap();
}
