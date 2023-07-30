// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use serde_json::Value;
use std::fs;
use std::collections::HashMap;



// slice_size: Determines the maximum numbers allowed to compare
// holder_string: a String object containing the numbers
// current_position: a usize state that determines return time
// compiled_Values: A hashmap containing all values searched so far
// temp_String: stores the progress so far for collapse


fn recursive_iteration_of_string(
  slice_size: i8, 
  holder_string: String, 
  current_position: usize,
   mut compiled_Values: HashMap<String, i16>, 
   mut temp_string: Vec<u16>) -> HashMap<String, i16>{

  
  // pass the compiled values around the depths
  // if no value to pass around create a new one
    
    
    // if current depth = slice_size, return array of possible solutions for depth
    // otherwise iterate over remaining possible objects
    // for each object call this function

      let binding = holder_string.replace('"', "");
      let iterable_string: Vec<_> = binding.split(" ").collect();

  
      temp_string = temp_string.clone();

      let mut split_count: usize = 0;
      for value in temp_string.iter(){
        if value != &9999{
          split_count += 1;
        }
      }



      for x in current_position..8 {


        // if we have reached the target threshold for the length of our search begin compiling the values
        if split_count as i8 == slice_size{
          let mut key: String = "".to_string();
          temp_string[current_position + 1] = iterable_string[x].parse::<u16>().unwrap();

          // before adding the value sort the numbers from smallest to largest.
          let mut q: Vec<u16> = temp_string.clone();
          q.sort();
          q.retain(|value| *value != 9999);
          q.iter().for_each(|num: &u16| key = format!("{} {}", key, num.to_string()));

          compiled_Values.entry(key).or_insert(1);

        } 
        else {

          temp_string[current_position] = iterable_string[x].parse::<u16>().unwrap();
          // add the new number to the holder and parse that value
          let returned_values: HashMap<String, i16> = recursive_iteration_of_string(slice_size, (*holder_string).to_string(), 
          x + 1, 
          compiled_Values.clone(), (*temp_string).to_vec());
          // extract value from returned function and check if its already in the system.
          // if does not already exist create new element

          returned_values.iter();

          for returned_value in returned_values{
            compiled_Values.entry(returned_value.0).or_insert(1);
          }
        }
      }

      return compiled_Values;



}

// main function that caculates the averages
#[tauri::command]
fn calculateAverage(indexcount: i8) -> Result<(f64, String), String>{

  // basic definement
  let mut compiled_values: (f64, String) = (0.0, "".to_string());

  let contents: String = fs::read_to_string("ui\\Assets\\Winners.json").unwrap();
  let json_values: Value = serde_json::from_str(&contents).unwrap();

  let mut temp_hash: HashMap<String, i16> = HashMap::new();

  // I have no idea; but it works and I don't want to refigure it out
  for (mut i, numbers) in json_values["Winners"].as_array().iter().enumerate(){
    loop {

      if i == (numbers.to_vec().len()){
        // no overflows on my watch
        break;
      } else {

        {
          let burner_hash: HashMap<String, i16> = HashMap::new();
          for (key, value) in recursive_iteration_of_string(indexcount - 1, numbers.to_vec()[i].to_string(), 0, burner_hash, vec![9999; 9]){
            temp_hash.entry(key).and_modify(|mut value| *value += 1).or_insert(1);
          };
        }


        i += 1;
      }
    }
  }

  let mut current_key: (String, i16) = ("".to_string(), 0);
  let mut ticker: u16 = 0;
  for (key, value) in temp_hash.clone(){
    if value > current_key.1{
      current_key = (key, value);
    }
    ticker += 1;
  }

  // Round to x.xxx
  let percent: f64 = format!("{:.1$}", current_key.1 as f64 / ticker as f64 * 100.0, 3).parse::<f64>().unwrap();
  println!("{} | {}", current_key.0, current_key.1);
  compiled_values.0 = percent;
  compiled_values.1 = current_key.0;
  println!("finished: index {}", indexcount);
  Ok(compiled_values)
}

fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![calculateAverage])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
