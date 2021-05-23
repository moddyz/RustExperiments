enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // Allocate the int32 5 on the heap (not very exciting).
    let b = Box::new(5);
    println!("b = {}", b);

    // Create a List.
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
}
