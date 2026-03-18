// Generic 



// Removing duplication by extracting the function 
// To Eliminate this duplication 

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// we had two functions: one that finds the largest item in a slice of i32 values
// and one that finds the largest item in a slice of char values. 
// How would we eliminate that duplication?


fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// The function bodies have the same code, so let’s eliminate the duplication 
// by introducing a generic type parameter in a single function.


// The function is generic over some type T. T is generic type parameter here 
// &[T] = reference to a slice (borrowed array/vector)
// Eg : &[i32] , &[char]
// The function has one parameter names list which is silce of values of type T
// The largest function will return a reference to a value of same type T


// > < >= <= comes from trait

fn general<T> (list: &[T]) -> &T {

    if list.is_empty() {
        return None;
    } // This wont cause runtime panic 

    // what is largest ? 
    // list[0] is T
    // &list[0] -> &T
    let mut largest = &list[0];

    for item in list { // here list is &T , iteration gives item : &T

        // so what happens here !
        // largest : &T and item : &T
        // comparing &T with &T , &T > &T
        // Rust says I dont know if type T support comparison 
        
        // so this function is missing trait bound : PartialOrd

        if item > largest {
            largest = item;
        }
    }

    largest
}

// Generic in Structs 

struct Point<T> {
    x : T,
    y : T,
}

// Point<T> : A template for creating structs
// This is basically defining the blueprint



fn main(){
    
    // Finding the largest number in the list of number
    let number_list = vec![34,50,25,100,65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result  = largest_char(&char_list);
    println!("The largest number is {result}");


    let integer = Point{x : 5, y : 10}; // compiler decides what type 
    let float = Point {x : 1.0 , y : 4.0};

    let wont_work = Point { x: 5, y: 4.0 }; // fails
    // This fails as first x = 5 -> rust infers T = i32
    // Now rust becomes 
    Point<i32> {
    x: i32,
    y: i32,
    }
    // Then y = 4.0 // f64 mismatch 

    // Generics in enum 

    enum Option<T> {
        Some(T),
        None,
    }

    // Option<T> = Maybe a value of type T 

    let a = Some(5);      // Option<i32>
    let b = Some("hi");   // Option<&str>

    // Result <T,E>
    enum Result<T,E> {
        Ok(T),
        Err(E),
    }

    // Mthods on generic structs

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    // Why impl<T> is important ? 
    // We must tell rust that this impl is generic 

    // Specialized Implementation 

    impl Point<f32> {
        fn distance_from_origin(&self)->f32{
            ...
        }

        // this method only exist for point<f32>
    }

}
