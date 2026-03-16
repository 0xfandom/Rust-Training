// struct Counter {
//     count : u32,
// }

// impl Counter {
//     fn value(&self) -> u32 { // read only
//         self.count
//     }

//     fn increment(&mut self) {
//         self.count +=1;
//     }

//     fn reset(self) -> Counter {
//         Counter {count : 0}
//     }
// }

// fn main (){
//     let mut c  = Counter {count : 0};

//     c.increment();
//     c.increment();

//     println!("{}" , c.value());

//     let c  = c.reset();
//     println!("{}" , c.value());

// }


// funtions without self are associatedf funtions - no self parameter  

struct Box {
    value: i32,
}

impl Box {
    fn new(v: i32) -> Box { // this will return the value but to b 
        Box { value: v }
    }

    fn double(&mut self) {
        self.value *= 2;
    }

    fn get(&self) -> i32 {
        self.value
    }
}

fn main() {
    let mut b = Box::new(5);
    b.double(); // here the value is 10
    b.double(); // here it becomes 20
    println!("{}", b.get()); // prints 20
}

// this is what i understood