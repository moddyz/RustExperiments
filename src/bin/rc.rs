use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // Basically shared pointers.
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("a ref count: {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("a ref count: {}", Rc::strong_count(&a));
    let c = Cons(4, Rc::clone(&a));
    println!("a ref count: {}", Rc::strong_count(&a));
}
