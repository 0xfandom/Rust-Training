trait Summary {
    fn summarize(&self) -> String
}

// Any type implementing Summary must provide summarize()

// Implementing a trait

struct NewsArticle {
    headline : String
}

impl Summary for NewsArticle {
    fn summarize(&self)->String {
        format!("Headline : {}" , self.headline)
    }
}

// Using Trait as Constant 

fn notify<T:Summary>(item:T){
    println!("{}" , item.summarize());
}

// i dont care what type T is - as long as it behaves like summary 


trait Animal {
    fn speak(&self)
}

Struct Dog;

impl Animal for Dog {
    fn speak(&self){
        println!("Bark");
    }
}

// Traits are basically interface but with more power such as built in trait

// Trait as Functional Parameters

fn notify(item: &impl Summary)
/// Access any type that implements summary 

// Equivalent to 
fn notify<T: Summary>(item: &T)

// Multiple Trait Bounds 

fn notify<T: Summary + Display>(item: &T)

// Where Claude Readability Tool

// Instead of:

fn f<T: Display + Clone, U: Debug + Clone>

// Use:

fn f<T, U>()
where
    T: Display + Clone,
    U: Debug + Clone,