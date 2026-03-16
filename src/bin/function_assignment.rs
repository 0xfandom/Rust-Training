fn main() {
    let s = String::from("hello");

    takes_ownership(s);   // s is MOVED into the function
                          // s is dead here

    println!("{s}");      // ❌ COMPILE ERROR
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}  // some_string goes out of scope → drop() called → memory freed

// but with a copy type this would have been diffferent 
fn main() {
    let x = 5;

    makes_copy(x);        // x is COPIED into the function
                          // x is still alive here

    println!("{x}");      // ✅ fine — x was copied, not moved
}


fn makes_copy(nun : i32){
    println!("{num}");
}