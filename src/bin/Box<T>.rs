
fn main(){

let x = 5;           // lives on the stack
let y = Box::new(5); // lives on the heap, y is a pointer to it

println!("{}", x);   // 5
println!("{}", y);   // 5  — Box auto-derefs, feels the same to use

}

// The three cases where we need it !!
// 1. Recursive type — size unknown at compile time
enum Tree {
    Leaf(i32),
    Node(Box<Tree>, Box<Tree>),   // without Box, infinite size → compiler error
}

// 2. Trait object — store different types behind one pointer
let shapes: Vec<Box<dyn Draw>> = vec![
    Box::new(Circle { radius: 5 }),
    Box::new(Square { side: 3 }),
];

// 3. Transfer ownership of large data without copying
let big_data = Box::new([0u8; 1_000_000]); // only pointer moves, not 1MB
let moved = big_data;  // cheap — just moved the 8-byte pointer


// Treating Smart Pointers like regular reference (Deref + Drop)

// Deref Trait 

// Deref lets you use * on a smart pointer just like a regular reference.
// Without it, Box would just be a struct — you couldn't dereference it.


