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
          // println!("{}", *value);
          split_count += 1;
        }
      }



      for x in current_position..7 {


        // if we have reached the target threshold for the length of our search begin compiling the values
        if split_count as i8 == slice_size{
          let mut key: String = "".to_string();
          temp_string[current_position + 1] = iterable_string[x].parse::<u16>().unwrap();

          // before adding the value sort the numbers from smallest to largest.
          let mut q: Vec<u16> = temp_string.clone();
          q.sort();
          q.iter().for_each(|num: &u16| key = format!("{} {}", key, num.to_string()));

          println!("{} OASS", key);
          compiled_Values.entry(key).and_modify(|count| *count += 1).or_insert(1);

        } else {

          temp_string[current_position] = iterable_string[x].parse::<u16>().unwrap();
          // add the new number to the holder and parse that value

          recursive_iteration_of_string(slice_size, (*holder_string).to_string(), 
          x + 1, 
          compiled_Values.clone(), (*temp_string).to_vec());
          // extract value from returned function and check if its already in the system.
          // if does not already exist create new element
        }
      }
      let temp_hash: HashMap<String, i16> = HashMap::new();
      return temp_hash;



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

        let temp_hash: HashMap<String, i16> = HashMap::new();
        println!("{:?}", recursive_iteration_of_string(indexcount, numbers.to_vec()[i].to_string(), 0, temp_hash, vec![9999; 8]));

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
