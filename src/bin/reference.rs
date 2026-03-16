// reference in rust which solves that ownership move problem 
// which is ratther than transferring ownership we lend the ownership nand the function
// borrow the ownership , temproarily untill it returns.



// A reference lets you point to data without owning it. No ownership = no move = original stays alive.

// what a reference actually is in memory?
// let s1 = String::from("hello");
// let r = &s1;   // r is a reference to s1

// STACK                         HEAP
// ─────                         ────
// s1
//  ├─ ptr          ───────────► ["hello"]
//  ├─ len: 5                       ▲
//  └─ cap: 5                       │
//                                   │
// r                                 │
//  └─ ptr  ───► s1 ────────────────┘


// r doesn't point to the heap directly. 
// It points to s1, which points to the heap. 
// It's a pointer to a pointer. r has no ownership — it's just borrowing the view.

// The & Symbol — What It Means In Each Position

// In a function 
fn greet (name : &String) {} // parameter is a reference to a string.

// In an expression - means "create a reference to this"

fn main(){

let mut s1 = String::from("Hello");

greet(&s1);

let r = &s1; // r borrow s1

add_world(&mut s1); // this is how you pass mutable reference 

println!("{s1}");


}

// Simple rule:
// ```
// &Type       →  "I accept a reference"
// &value      →  "give me a reference to this value"

// Mutable and Immutable reference 

// Immutable Reference 
// fn calculate_length(s: &String) -> usize {
//     s.len()   // can read, can't modify
// }

// fn main() {
//     let s1 = String::from("hello");
//     let len = calculate_length(&s1);   // lend s1
//     println!("{s1} has length {len}"); // s1 still alive ✅
// }
// ```

// Step by step in memory:

// **When `&s1` is passed:**
// ```
// STACK                         HEAP
// ─────                         ────
// main:
//   s1
//    ├─ ptr        ───────────► ["hello"]
//    ├─ len: 5                     ▲
//    └─ cap: 5                     │
//                                   │
// calculate_length:                 │
//   s: &String  ─────► s1 ─────────┘
//                     (borrowing, not owning)
// ```

// **When function returns:**
// ```
// STACK                         HEAP
// ─────                         ────
// main:
//   s1
//    ├─ ptr        ───────────► ["hello"]  (still here, never moved)
//    ├─ len: 5
//    └─ cap: 5

//   len: 5

// Mutable Reference - Looking and Changing 

fn add_world(s: &mut String) {
    s.push_str(" world");   // can modify through mutable reference
}


// Two things must for the mutable reference 

// 1. The variables must be declared mut
// 2. you pass &mut not just &

// Memory during the function :

// STACK                         HEAP
// ─────                         ────
// main:
//   s1 (mut)
//    ├─ ptr        ───────────► ["hello world"]  ← modified in place
//    ├─ len: 11                    ▲
//    └─ cap: ...                   │
//                                   │
// add_world:                        │
//   s: &mut String  ──► s1 ────────┘
//                     (can read AND write)

// Rules of reference (Only 2 but strict)
// Rust enforces this at compile time 

// Rule 1 : Many Immutable or 1 Mutable (Never Both)

let mut s = String::from("hello");

let r1  = &s1; // immutable borrow
let r2  = &s1; // another immutable fine 

let r3 = &mut s; // this will throw compile error 

// Why ? 
// if someone is reading , while someone is writing - the reader gets curropted
// Rust makes this physically impossible.

// ALLOWED:                    NOT ALLOWED:
// ────────                    ────────────
// r1 ──► s  (reading)         r1 ──► s  (reading)
// r2 ──► s  (reading)         r3 ──► s  (WRITING) ← conflict
// r3 ──► s  (reading) 

// Rule : 2 -> References must always be valid

// fn dangle() -> &String {    // ❌ COMPILE ERROR
//     let s = String::from("hello");
//     &s   // return a reference to s
// }   // s is dropped here — reference now points to freed memory

// After function required : 

// s is gone -> memory freed




