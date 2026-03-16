// creating the struct 

struct User {
    name : String , // Owned - user owns this string
    age : u32, // copy type - no ownership issues
    active : bool, // copy type - no ownership issues
}


fn main(){
    let mut user = User {
        name: String::from("Arjun"), // heap memory allocated
        age: 28, // stack value 
        active: true, // stack value
    };

    user.age = 29;

    let user2 = User{
        name : String::from("Rahul"); // different name
        ..user // rest copied from the user 
    }

    println!("{}" , user.name);
    println!("{}" , user.age);
    println!("{}" , user.active);

    let point  = (10,20); // this is regular tuple 

    // tuple struct 
    struct Point (i32,i32);

    let point2 = Point(10,20); // now its clear - its a point

    let p = Point(10,20);
    let Point(x,y) = p; // destructure into x and y 
    println!("{}", x);  // 10
    println!("{}", y);  // 20

}

// Mutability In Structs 

// The whole struct is either mutable or not !
// You can't make individual feild mutable


// Ownership inside structs 

// why not to use &str in Structs ? 

struct User {
    name: &str,   // ❌ problem
} // This means name is borrowed, but borrowed from where , and how long 
// does it live

fn create_user() -> User {
    let name = String::from("Arjun");
    let user = User{name : &name} // name dies here , dangling reference 
}

// Why does the whole struct need to be mut instead of individual fields?
//Ans : Rust ownership level works at variable level not field level

// when we declare let mut user = User { ... };
// `mut` belongs to the **binding** `user` — not to the struct itself or its fields.

// struct is just a variable that happens to have feild in it.

// Tuple struct : A tuple struct is a struct that has no field names — just types. Like a tuple but with a name.

// Two tuple structs with same types are different types
// and this thing matter when you write functions 

fn draw (p : point){

}

draw(color); // error Color is not a point 
draw(point): // correct type

// Real world use case of Tuple Struct 

// Tuple structs are great for wrapping a single value to give it meaning 

struct meters(f64);
struct kilogram(f64);

fn calculate_speed(distance: Meters, weight: Kilograms) { ... }

let d = Meters(100.0);
let w = Kilograms(75.0);

// you can never accidentally pass weight where distance is expected
// compiler catches it

// This pattern is called newtype pattern - wrapping a primite to give it a distinct type and meaning 


// ### Unit Structs 
// A struct with no fields at all.

struct alwaysTrue; // no {} , () , no fields


// Struct Methods

struct User {
    name : string; // data lives here 
}

impl User {
    fn greet(&self){ // behavior lives here 
        println!("Hi {}" , self.name);  
    }
}

let user = User {name:String::fro("fandom")};

user.greet(); 

// &self what this actually is ??
// Every Method takes self as the first parameter. It refers to the instance of calling the method

// 3 Versions - each means something different

impl User {
    fn read(&self) { }          // borrows self — can READ, cannot modify
    fn modify(&mut self) { }    // borrows self mutably — can READ and MODIFY
    fn consume(self) { }        // takes ownership — self is GONE after this
}

// &self      →  immutable borrow  →  just looking
// &mut self  →  mutable borrow    →  looking and changing
// self       →  move              →  takes ownership, instance gone after call

