use serde_json::{json, Value};

fn type_of<T>(_: T) -> String{
  let a = std::any::type_name::<T>();
  return a.to_string()
}

fn main(){
  let input_path = "./test.dat";
  let output_path = "./test.dat";

  let text = std::fs::read_to_string(&input_path).unwrap();
  let mut txt = serde_json::from_str::<Value>(&text).unwrap() ;

  println!("typeof txt: {}", type_of(&txt));
  println!("txt: {}", txt);
  println!("txt[\"ra\"]: {}", txt["ra"]);
  println!("txt[\"rb\"]: {}", txt["rb"]);
  println!("txt[\"rc\"]: {}", txt["rc"]);

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
