// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use serde_json::Value;
use std::fs;


#[tauri::command]
fn calculateAverage(indexcount: i8) -> Result<[&'static str; 2], String>{

  let mut compiled_values: [&str; 2] = [""; 2];

  let contents: String = fs::read_to_string("ui\\Assets\\Winners.json").unwrap();
  let json_values: Value = serde_json::from_str(&contents).unwrap();


  for (mut i, numbers) in json_values["Winners"].as_array().iter().enumerate(){
    loop {
      if i == numbers.to_vec().len(){
        break;
      } else {
        println!("{:?}", numbers.to_vec()[i].as_str());
        i += 1;
      }

    }
  }

  compiled_values[0] = "27";
  compiled_values[1] = "52 60 20 52";
  Ok(compiled_values)
}


fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![calculateAverage])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
