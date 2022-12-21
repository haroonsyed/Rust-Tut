use std::collections::HashMap;

pub fn run() {
  let mut map: HashMap<&str, i32> = HashMap::new();
  map.insert("One", 1);
  map.insert("Two", 2);
  map.insert("Three", 3);
  map.insert("Ten", 10);

  for (k, v) in map.iter() {
    println!("Key {}, value {}", k, v);
  }

  // All the other hashmap functions are obvious
}
