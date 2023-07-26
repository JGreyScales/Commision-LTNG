// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use serde_json::Value;
use std::fmt::format;
use std::fs;
use std::collections::HashMap;


// slice_size: Determines the maximum numbers allowed to compare
// holder_string: a String object containing the numbers
// current_position: a usize state that determines return time
// compiled_Values: A hashmap containing all values searched so far
// temp_String: stores the progress so far for collapse


fn recursive_iteration_of_string(slice_size: i8, holder_string: String, current_position: usize, mut compiled_Values: HashMap<String, i16>, temp_string: String) -> HashMap<String, i16>{
  
  // pass the compiled values around the depths
  // if no value to pass around create a new one
    
    
    // if current depth = slice_size, return array of possible solutions for depth
    // otherwise iterate over remaining possible objects
    // for each object call this function

      let binding = holder_string.replace('"', "");
      let iterable_string: Vec<_> = binding.split(" ").collect();

      let mut split_temp = temp_string.split(" ");
      let split_count: usize = split_temp.count() + 1;

      




      for x in current_position..7 {
        // if we have reached the target threshold for the length of our search begin compiling the values
        if split_count as i8 == slice_size{

          // before adding the value sort the numbers from smallest to largest. split temp using .next commands
          compiled_Values.entry(format!("{} {}", holder_string, iterable_string[x])).and_modify(|count| *count += 1).or_insert(1);
        } else {
          // add the new number to the holder and parse that value


          // extract value from returned function and check if its already in the system.
          // if does not already exist create new element
        }
      }
    
      return compiled_Values;



}

// main function that caculates the averages
#[tauri::command]
fn calculateAverage(indexcount: i8) -> Result<[&'static str; 2], String>{

  // basic definement
  let mut compiled_values: [&str; 2] = [""; 2];

  let contents: String = fs::read_to_string("ui\\Assets\\Winners.json").unwrap();
  let json_values: Value = serde_json::from_str(&contents).unwrap();


  // I have no idea; but it works and I don't want to refigure it out
  for (mut i, numbers) in json_values["Winners"].as_array().iter().enumerate(){
    loop {

      if i == (numbers.to_vec().len()){
        // no overflows on my watch
        break;
      } else {

        let temp_hash: HashMap<String, i16> = HashMap::new();

        recursive_iteration_of_string(indexcount, numbers.to_vec()[i].to_string(), 0, temp_hash, "".to_string());

        // iter through each object in return and find highest value object
        // take the value of maximum value divided by the sum of all object values * 100%


        // iterate over split string
        // for (mut x, number) in numbers.to_vec()[i].to_string().replace('"', "").split(" ").enumerate() {




        //   if x as i8 == indexcount{
            
        //     // simple try catch logic to logically update the hashmap values
        //     final_values.entry(temp_string).and_modify(|count| *count += 1).or_insert(1);

        //     break;
        //   } else {
        //     // string compulation based on index parameters
        //     // optimization improvements will be made for index = 8 scenerios
        //     temp_string = format!("{} {}", temp_string, number);
        //   }
        //   x += 1;
        // }
        i += 1;
      }

    }
  }

  compiled_values[0] = "27";
  compiled_values[1] = "52 60 20 52";
  println!("finished");
  Ok(compiled_values)
}


fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![calculateAverage])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
