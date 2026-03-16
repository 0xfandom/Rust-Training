
#[derive(Debug)]
struct Task {
    title: String,
    done: bool,
}


fn main() {
    let scale  = 2;
    let task = Task {
        title: String::from("Learn Rust"),
        done:  dbg!(scale > 1),
    };

    dbg!(&task);
}