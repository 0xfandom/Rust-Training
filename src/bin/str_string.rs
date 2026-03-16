// &str vs String::new()

// String Literal (&str)

fn main(){

    let s = "hello"; // this is of type &str, the literals are stored in read only memory 
    // important properties : immutable , known at compile time , stores as executable 


    let s = String::from("hello"); // String Type

    // Mutable String 

    let mut ss = String::from("hello");
    ss.push_str(", world!"); // The heap memory allows growing text.

    println!("{}",ss);

}

// Memory Layout 
// Stack
// -----
// s ─────► "hello"

// Program binary (read-only memory)
// ---------------------------------
// "hello"

// String Literals fails in the situation like
//              user input (we cannot write that in binary before hand)
//              dynamic data  


// The String Type : Rust also provides string type 
// here memory requested from heap , text stored in heap , the pointer is in stack

// Stack                         Heap
// -----                         -----
// s
// ├ pointer ───────────────►   "hello"
// ├ length = 5
// └ capacity = 5


// Heap Allocation Responsibility 

// Whenever heap memory is used , two things must happen 
// 1. Allocation : Request Memory 
// 2. DeAllocation : free memory when done , otherwise memory leak 

// How other language solve this thing ?
// Ans : GC , Manual Memory Management

// Rust Solution is Ownership , memory is automatically freed , when variable goes out of scope

// The Drop Funtion : drop: this is implemented for string , when the scope ends (drop(s));

// What drop does : free heap memory ,  clean resources
