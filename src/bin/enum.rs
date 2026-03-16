enum Direction {
    North, // each options are called variant 
    South,
    East,
    West   
}

// Just like structs enums can have methods 

enum Messgae {
  Quit,
  Write(String),
}

impl Message {
    fn process (&self) {
        match self {
            Message :: Quit
        }
    }
}

fn main(){

let dir = Direction::North; // to access a variant 

match dir {
    Direction::North => println!("Going North"),
    Direction::South => println!("Going South"),
    Direction::East  => println!("Going East"),
    Direction::West  => println!("Going West"),
}

}