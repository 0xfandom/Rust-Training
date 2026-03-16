// The problem which slices solve 

fn main(){

let s = String::from("hello world");
// how do you get "hello" out of this?


}


// Pizza Analogy 

// You order a pizza. It arrives. 8 slices.
// Your mom says: "You can have slices 1 to 3."
// She writes on a sticky note: "slices 1 to 3"

// The Problem — The Sticky Note Lies
// You go outside to play. Come back.
// Your brother ate the whole pizza while you were gone.
// Pizza:        GONE. Empty box.
// Sticky note:  still says "slices 1 to 3"

// The sticky note is useless now !!!

// let mut s = String::from("hello world");
// let word = first_word(&s);   // returns 5
//                              // sticky note says "first word ends at position 5"

// s.clear();                   // pizza is gone — s is now ""

// // word still says 5
// // 5 of what?? the string is empty
// // sticky note — useless
// ```
// ```
// String:   "hello world"   →   ""      (pizza eaten)
// word:          5          →    5      (sticky note still says 1 to 3)

// complete mismatch. number means nothing now.

// What slices do ?? : The better system
// Instead of writing a sticky note with just a number —

// Mom takes the pizza box, draws a line around slices 1 to 3, and says:

// > **"This section right here. Whatever is in THIS part of the box."**

// Now the section is **directly tied to the actual pizza.**

// ---

// ## The Critical Difference
// ```
// Sticky note system:    "slices 1 to 3"
//                         just a number
//                         completely disconnected from the pizza
//                         pizza can change, note never knows

// Direct section system:  a window INTO the pizza itself
//                         directly connected
//                         if pizza changes — the window reflects it
//                         if pizza is gone — window is invalid (Rust catches this)



// In Rust Slices are the window system !!!

let s = String::from("hello world");

let hello = &s[0..5];    // window into s, characters 0 to 5
let world = &s[6..11];   // window into s, characters 6 to 11
```
```
s         →   "hello world"
                ▲     ▲
                │     │
hello ──────────┘     │     (window directly into s)
world ────────────────┘     (another window directly into s)