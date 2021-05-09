fn main() {
    // Create a new mutable string.
    let mut s = String::new();

    // Initialize a String from a string literal.
    // All 3 "string = ..." are equivalent.
    let string_literal = "my string.";
    let string = string_literal.to_string();
    let string = "my_string".to_string();
    let string = String::from("my_string");

    // Appending string to a string.
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    // Appending a character to a string.
    let mut s = String::from("foo");
    s.push('x');
    println!("{}", s);

    // Formatting strings.
    let f = format!("{}, {}", s, "baz");
    println!("{}", f);

    // Concatnating two strings.
    // The + operator takes ownership of a, but not b.
    let a = String::from("a");
    let b = String::from("b");
    let c = a + &b;
    println!("{}", c);

    // Iterating over the characters of a String.
    for c in "foobar".chars() {
        println!("{}", c);
    }

    // Iterating over the bytes of a String.
    for b in "foobar".chars() {
        println!("{}", b);
    }
}
