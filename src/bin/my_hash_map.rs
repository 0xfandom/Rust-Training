// In this we wil build our own hashmap

// Our HashMap will:
//   → store String key → String value pairs
//   → use a simple hash function
//   → handle collisions with chaining
//   → support: insert, get, remove, contains_key


// Step-1 : Understanding the structure 
// A hashmap is an array of buckets .
// Each bucket holds the list of key-value pairs (for collision handlng)

// buckets:
//   [0] → []
//   [1] → [("hello", "world")]
//   [2] → []
//   [3] → [("name", "Arjun"), ("game", "chess")]  ← collision
//   [4] → []
//   ...

pub struct HashMap<K,V> {
    buckets: Vec<Vec<(k,v)>>, // dynamic array , outer vec handles bucket 
    // inner vec - handles collision
    size: usize, // total number of buckets 
} 

// A vector of buckets , each bucket is a vector of key-value tuple pairs
// pub - public can be used outside , <K,V> generic types | k = key , V = Value

// Step-2 Constructor 
// method for this struct 
impl<K,V>HashMap<K,V>{
    
    // function to create new hash map 
    // returns self ->means HashMap<K,V>
    pub fn new(size : usize) -> Self {

        let mut buckets = Vec::with_capacity(size);
        // creates empty vector 

        for _ in 0..size {
            buckets.push(Vec::new());
        } // loop from 0 to capacity , _ ignore the variable 
        // each iteration push an empty vector 
        // buckets = [[], [], [], ..., []]

        Self {buckets , size}
    }
}

