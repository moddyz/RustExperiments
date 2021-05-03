fn main() {
    // Creating a new empty vector.
    let v: Vec<i32> = Vec::new();

    // Creating a vector with a few entries.
    let v = vec![1, 2, 3];

    // Create a mutable vector and push some elements to it.
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);

    // Assign value to a mutable reference.
    let e = &mut v[0];
    *e = 2;

    // Pretty-print a vector.
    println!("{:?}", v);

    // Iterate over read-only references to the elements.
    for i in &v {
        println!("{}", i);
    }

    // Iterate over mutable references and increment each value.
    for i in &mut v {
        *i += 1;
        println!("{}", i);
    }

    // Storing multiple types in a single vector.
    enum Variant {
        Int(i32),
        Float(f64),
    }

    let v = vec![Variant::Int(3), Variant::Float(3.5)];
}
