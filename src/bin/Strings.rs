// Strings in Rust !!!

// Rust has two main strings types 

// 1. &str (string slice)

let s  = "hello";
// Properties : Immutable , Borrowed , Stored in binary 

// 2. String

let s = String::from("Hello");
// Properties : Mutable , Owns in data (Heap Allocated) , Growable 

// Internal Representation 

// Rust string = Vec<u8>(bytes)

// so this 
let s  = String::from("Hello");
// Actually is : [104, 101, 108, 108, 111]


// Why indexing is not allowed ?

let s  = String::from("Hello");
let x  = s[0]; // error 

// Why ? 
// Because what should s[0] return ?? -> byte , char , or grapheme

// Also performance reason , indexing is expected to be 0(1)
// but in utf-8 you must walk from start , so indexing would be o(n)

// String Slicing , instead of indexing 

let s  = "Здравствуйте";
let part = &s[0..4];

// Iterating over strings 

for c in "नमस्ते".chars() {
    println!("{c}");
}

// for bytes 
for b in "नमस्ते".bytes() {
    println!("{b}");
}

// creating strings 
let s = String::new();
let s   = "hello".to_string();
let s  = String::from("hello")

// all three are valid 

// updating strings 
let mut s = String::from("foo");
s.push_str("bar");

// to push a single char
s.push('!');

// concatenation using + 
let s1 = String::from("Hello ");
let s2 = String::from("World");

let s3 = s1 + &s2;

// better way is to use format 
let s = format!("{s1} {s2}");