// functions can give ownership back by returning 

fn main(){
    let s1 = gives_ownership() // here s1 got the ownership of the string
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);  // here now s2 moved in so the ownership is with this function
    // and then ownership returned as s3 , so s2 is dead and s3 is alive
}


fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string   // moves out to caller — no drop here
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string   // moves back out to caller
}

// Now at the end of main 
// s3 goes out of scope -> drop()->free()


// The problem this creates 

// step 1 : s1 is created  
// let s1 = String::from("hello");
// 
// 
// STACK                    HEAP
// ─────                    ────
// s1
//  ├─ ptr        ────────► ["hello"]
//  ├─ len: 5
//  └─ cap: 5

// step-2 : s1 is passed into the function 

// let (s2, len) = calculate_length(s1);
// The momemt 's1' crosses into the function - ownership moves
// STACK                    HEAP
// ─────                    ────
// s1 ❌ (dead, gone)
//                          ["hello"]
//                               ▲
// fn calculate_length:          │
//   s                           │
//    ├─ ptr        ─────────────┘
//    ├─ len: 5
//    └─ cap: 5

// step-3 : inside the function 

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();   // just reads the length — no ownership change
//     (s, length)             // packages s and length into a tuple to return
// }

// s.len() just reads the data -doesn't move anything 
// Then (s, length) packages everything up to hand back to the caller.

// The function returns (s, length) and then the ownership moves to s2

// ## The full journey in one picture
// ```
// main()              calculate_length()         main()
// ──────              ──────────────────         ──────

// s1 owns "hello"
//        │
//        │  MOVE IN
//        └──────────► s owns "hello"
//                     length = 5
//                         │
//                         │  MOVE OUT (as tuple)
//                         └──────────────────► s2 owns "hello"
//                                              len = 5


// s1 ──── DEAD ────────────────────────────────────────────►

// But the problem you just wanted length but to get it 
// Move s1 into the function — s1 dies
// Do the calculation
// Package the String back up in a tuple
// Move it back out
// Destructure it just to get your original String back as s2

// How to solve this problem  : WITH references