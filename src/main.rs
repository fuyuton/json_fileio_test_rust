//use serde_json::{json, Value, Result};
use serde_json::json;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]

struct HashFile {
  //string_data: HashMap <String, HashMap<String, String>>,
  //integer_data: HashMap <String, HashMap<String, i32>>
  string_data: HashMap <String, String>,
  integer_data: HashMap <String, i32>
}



fn type_of<T>(_: T) -> String{
  let a = std::any::type_name::<T>();
  return a.to_string()
}

/*
fn json_to_hashmap(json: &str, keys: Vec<&str>) -> Result<HashMap<String, Value>> {
    let mut lookup: HashMap<String, Value> = serde_json::from_str(json).unwrap();
    let mut map = HashMap::new();
    for key in keys {
        let (k, v) = lookup.remove_entry (key).unwrap();
        map.insert(k, v);
    }
    Ok(map)
}
*/

fn main(){
  let input_path = "./json_test.dat";
  let output_path = "./test.dat";

  let file = File::open(input_path).unwrap();
  let reader = BufReader::new(file);
  let de: HashFile = serde_json::from_reader(reader).unwrap();

  for (key, value) in de.string_data.iter(){
    print!("{}\t", key);
    print!("{}\n", value);
    /*
    for (k, v) in value.chars(){
      print!("{} {}\n", k, v);
    }
    */
  }
  for (key, value) in de.integer_data.iter(){
    print!("{}\t", key);
    print!("{}\n", value);
    /*
    for (k, v) in value{
      print!("{} {}\n", k, v);
    }
    */
  }

  //let text = std::fs::read_to_string(&input_path).unwrap();


  //let keys = vec!["ra", "rb", "rc"];
  // うまく行かない
  //let mut hash = serde_json::from_str::<Value>(&text).unwrap() ;
  //let mut rc = deserialize_i32(hash["rc"]);
  //println!("rc: {}", rc);
  //rc += 1;


/*
  let mut hash = json_to_hashmap(&text, keys);
  println!("typeof hash: {}", type_of(&hash));
*/


  /*
  println!("hash: {:?}", hash);
  println!("hash[\"ra\"]: {:?}", hash["ra"]);
  println!("hash[\"rb\"]: {:?}", hash["rb"]);
  println!("hash[\"rc\"]: {:?}", hash["rc"]);
  println!("typeof hash[\"rc\"]: {:?}", type_of(&hash["rc"]));
*/

  let k = json!({
    "ra": " ra ",
    "rb": "is not ruby",
    "rc": 3
  });

  println!("typeof k: {}", type_of(&k));
  println!("json k: {}", k);
  println!("json k[\"ra\"]: {}", k["ra"]);
  println!("json k[\"rb\"]: {}", k["rb"]);
  println!("json k[\"rc\"]: {}", k["rc"]);


  // Save the JSON structure into the other file.
  std::fs::write(
      output_path,
      serde_json::to_string_pretty(&k).unwrap(),
  )
  .unwrap();

}
