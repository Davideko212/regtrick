#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use registry::{Hive, Security, Data};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_dword, change_dword])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn get_dword(hkey: &str, path: &str, key: &str) -> u32 {
  let hive = get_hive(&hkey);

  let regkey_result = hive.open(path, Security::Read);
  let regkey;
  match regkey_result {
    Ok(RegKey) => regkey = RegKey,
    Err(_) => return u32::MAX,
  }

  let data_result = regkey.value(key);
  let data;
  match data_result {
    Ok(Data) => data = Data,
    Err(_) => return u32::MAX,
  }

  let s: String = data.to_string().drain(2..).collect();
  return s.parse().unwrap();
}

#[tauri::command]
fn change_dword(hkey: &str, path: &str, key: &str, value: u32) {
  let hive = get_hive(&hkey);

  // Creating missing keys incase there are any
  hive.create(path, Security::Write).unwrap();

  let regkey = hive.open(path, Security::Write).unwrap();
  regkey.set_value(key, &Data::U32(value)).unwrap();
}

fn get_hive(hkey: &str) -> Hive {
  let hive: Hive = match hkey {
    "HKEY_CLASSES_ROOT" => Hive::ClassesRoot,
    "HKEY_CURRENT_USER" => Hive::CurrentUser,
    "HKEY_LOCAL_MACHINE" => Hive::LocalMachine,
    "HKEY_USERS" => Hive::Users,
    "HKEY_CURRENT_CONFIG" => Hive::CurrentConfig,
    _ => unreachable!()
  };

  return hive;
}
