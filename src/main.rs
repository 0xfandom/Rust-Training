fn main() {
    let s1 = String::from("hello"); // here i am creating a string and assigning it to s1 , string stores data in the heap memory
    let s2 = s1; // now here i created another variable s2 and assigned it the value of s1
    // Many lang here does copy by value , but in rust here ownership moves 
    println!("{s1}, world!");
}

// The meemory looks lie 
// stack | s1 |
// heap  | hello |

// After the ownership moves to s2 the memory looks like
// stack | s2 |
// heap  | hello |

// Then what happens to s1 

// Internal Memory Layout 
// Stack                     Heap
// -----                     -----
// s1
//  ├ pointer ───────────► "hello"
//  ├ length = 5
//  └ capacity = 5

// A string on the stack actually stored 3 things (pointer, length, capacity)

// Now the assignment happeens 
// let s2 = s1; , here rust copies the stack data not the heap data 

// so internally this is happening 
// Stack                     Heap
// -----                     -----
// s1                         "hello"
//  ├ ptr ────────────────┐
//  ├ len = 5             │
//  └ cap = 5             │
//                        │
// s2                     │
//  ├ ptr ────────────────┘
//  ├ len = 5
//  └ cap = 5

// Both the stack variables point to the same heap data "hello"

// Now Rust Invalidates s1

// Stack                     Heap
// -----                     -----
// s2 ───────────────► "hello"
// s1 → invalid

// but the things is s1 in now invalid / uninitialized

// Why Heap Data Is Harder to Manage

// Heap memory creates several problems:

// 1. Who owns the memory?
// Example:
// two variables pointing to same heap data
// Who should free it?

// 2. Duplicate heap data
// Copying heap data unnecessarily wastes memory.

// 3. Memory leaks
// If memory is never freed:

// heap fills up
// program crashes


// How Rust Solves These Problems
// Rust uses ownership rules to manage heap memory safely.
// Ownership Rules:
// 1. one owner per heap allocation

// Imp thing : ownership mainly exists to manage heap data

// Stack memory is simple: push , pop -> done 
// Heap Memory : allocation , pointers , ownership , freeing memory
// ownership solves this complexity 

// Variable Scope and Ownership

// {                ← start scope
//    let s="hello"
//    use s
// }                ← s destroyed

// Primite type vs Heap type 

// i32 , bool , char , they are fixed size , cheap to copy and store on stack 
// Here string is used to explain the ownership , because string used heap memory 

// String literal &str let s = "hello";

// There is one more memory 