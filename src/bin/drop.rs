struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

use std::mem::drop;

fn main() {
    let c = CustomSmartPointer {
        data: String::from("some stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    // Deconstruct c early.
    drop(c);

    println!("CustomSmartPointer(s) created.");
}
