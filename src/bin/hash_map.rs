// a hash map stores data as key->value pairs , Like a dictionary in python 
// or object in JS.

// In hash map we access keys not index

use std::collections::HashMap;

fn main() {

let mut store = HashMap::new(); // creating the hash map

store.insert(String::from("name") , String::from("Arjun"));
println!("{}" , store["name"]);

}


// creating the hashmap 

let mut map : HashMap<String,String> = HashMap::new();

// with initial values - using collect 

let keys  = vec!["one" , "two" , "three"];
let values = vec! [1,2,3];

let map HashMap<&str,i32> = keys.into_iter()
                            .zip(values.into_iter())
                            .collect();



// The four core operations 


// Insert
map.insert(String::from("key1"), String::from("value1"));

// GET - return Option<&V>

let val = map.get("key1");

match val {
    Some(v) => println!("Found:{}" , v),
    None => println!("Not Found"),
}

// contains key 

if map.contains_key("key1") {
    println!("Exists");
}

// removing the key 

map.remove("key1");


// 3 Common Patters 

// Patter-1 - Always Overwrite
map.insert(String::from("key"), String::from("new_value"));

// Pattern-2 only insert if key doesn't exist
map.entry(String::from("key")).or_insert(String::from("default"));

// Pattern-3 - Update based on existing values

let count  = map.entry(String::from("word")).or_insert(0);
*count += 1;  // * dereferences the mutable reference


// Lets debug Pattern-1
// If we set a name : abc in hashmap via the insert and if we run insert
// again , it will overwrite that name abc with the new one xyz.

// Now About Pattern-2
// With entry().or_insert()
map.entry(String::from("name")).or_insert(String::from("Arjun"));
// Think of it like  : Find the entry for key "name"
// if it exists do nothing , keep existing value
// if it doesnt exits - insert arjun


// Patter 3 - Update based on existing values 

// Normal Variable 

let mut count = 0;
count += 1;
println!("{}", count);  // 1


let mut count  = 0 ; 
let r = &mut count; // here r is reference to count 
// r points to count , r is not count itself

// To change the actual value , you need *

*r +=1; // go to what r points to 
       // goes to count and add 1

// basically what is happening here : we are having r pointing toward 
// count , reference of count so we cannot directly change the value 
// of count , but what we need to do is we can use * , what it 
// does is it says go to what r points to , this is like following an arrow


// Iterating 

// Iterate over all key value pairs 

for(key , value) in &map {
     println!("{}: {}", key, value);
}

// just keys
for key in map.keys() {
    println!("{}", key);
}

// just values
for value in map.values() {
    println!("{}" , value);
}


// The entry api (Most Useful Pattern)

let text  = "Hello world hello rust hello"

let mut word_count = HashMap<&str , u32> = HashMap::new();

for word in text.split_whitespaces(){
    let count = word_count.entry(word).or_insert(0);
    *count +=1;
}



// lets understand Internal Working of hash Map 


// key -> hash function -> number -> bucket -> value

// "Blue" → hash → 123456 → index 6 → store value 10

// Iterating over hash map

for(key , value) in &something {
    println!("{key} : {value}")
}

// order is not guaranteed , hashmap is unordered