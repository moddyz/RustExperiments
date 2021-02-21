fn main() {
    // Mutable variables.
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constants.
    const MAX_VAL: i32 = 590;
    println!("The value of MAX_VAL is: {}", MAX_VAL);

    // Variable shadowing.
    let spaces = "     ";
    let spaces = spaces.len();
    println!("The number of spaces: {}", spaces);
}
