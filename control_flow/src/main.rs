fn main() {
    // if conditional.
    let x = 8;
    if x > 1 {
        println!("x > 1");
    } else if x == 1 {
        println!("x == 1");
    } else {
        println!("x <= 1");
    }

    // Conditional assignment (ternary operator).
    let y = if x > 1 { 1 } else { 2 };
    println!("y is {}", y);

    // Returning values from loops.
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result is {}", result);

    // while loop.
    while counter != 0 {
        println!("counter is {}", counter);
        counter -= 1;
    }

    // Iterating over a collection.
    let collection = [10, 20, 30, 40, 50];
    for element in collection.iter() {
        println!("element is {}", element);
    }

    // Iterating over a collection.
    let collection = [10, 20, 30, 40, 50];
    for element in collection.iter() {
        println!("element is {}", element);
    }

    // Iterating over a range.
    for element in 1..10 {
        println!("element is {}", element);
    }
    for element in (1..10).rev() {
        println!("element is {}", element);
    }
}
