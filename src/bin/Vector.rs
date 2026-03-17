// vector are growable contiguous array 

// This is syntax : Vec<T>
// It stores many values of the same type next to each other in memory.

// Important Properties
// Random access by index (O(1)).
// Growable with push.
// May reallocate (move its contents) when it grows beyond capacity.

// Creating a vector 

fn main(){
    let v  : Vec<i32> = Vec::new(); // this makes an empty vector 

    // sometime we use vec! macro 

    let v  = vec![1,2,3];
   // Vec::with_capacity(n) reserves space for n elements up front 
   // to avoid early reallocations.

   // Adding elements 

   let mut v = Vec::new();

   v.push(5);
   v.push(6);

   for i in &v {
    println!("{}" , i);
   }

   // Reading Elements: Indexing [] vs get

   let v1 = vec![1, 2, 3, 4, 5];

   // Indexing syntax 
   let third : &i32 = &v1[2];
   // &v[2] gives an immutable reference &i32.
   // if the index is out of range , this panics at runtime and crashes 

   // the second method : get method 

   let third : Option<&i32> = v1.get(2);

   match third {
    Some(val) => println!("{}" , val),
    None => println!("None came")
   }

   // get returns Option<&T> → Some(&value) if present, None if out of range.
   // Use get when you want to handle missing indices gracefully


   // Panic example vs safe get 
   let v2  = vec![1,2,4,3,5];
   // let x  = &v[100]; // panic , index out of bounds 
   let y  = v.get(100); // returns None , Safe

   // dropping vector and their elemets 

   {
    let v3  = vec![1,2,3];
    // use v here in this scope  
   } 
   // v and its elemets are dropped here 
}