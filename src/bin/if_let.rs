// The problem if let solves 

let number = Some(7);

match number {
    Some(n) => println!("Got {}" , n),
    None => {} // doing nothing 
};

// That None => {} is just empty. You don't care about it. But match forces you to handle every variant. 
// That's annoying when you only care about one case.

// if let is the solution 

if let Pattern = Expression {
    // runs only if the pattern matches
}

let number = Some(7);

if let Some(n) = number {
    println!("Got it {}" , n);
}

// if numebr is none , nothing happens no noise 

// Both does same thing , just the later is cleaner

// Finding a user !!!

fn find_user(id : u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Arjun"));
    }
    else{
        None
    }
}

fn main() {
    // with match
    match find_user(1) {
        Some(name) => println!("Found: {}", name),
        None       => println!("Not found"),
    }

    // with if let — same result, cleaner
    if let Some(name) = find_user(1) {
        println!("Found: {}", name);
    }

    // with if let + else
    if let Some(name) = find_user(99) {
        println!("Found: {}", name);
    } else {
        println!("Not found");  // this runs
    }
}