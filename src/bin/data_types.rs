fn main() {
    // Integers.
    let _x = 32; // i32;
    let _x: u32 = 0xff; // Hex.
    let _x: u32 = 0o77; // Octal.
    let _x: u8 = b'A'; // Byte literal.
    let _x: u8 = 0b1111_0000; // Binary.

    // Floating point.
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // Addition.
    let _sum = 5 + 10;

    // Subtraction.
    let _difference = 95.5 - 4.3;

    // Multiplication.
    let _product = 4 * 30;

    // Division.
    let _quotient = 56.7 / 32.2;

    // Remainder (modulo).
    let _remainder = 43 % 5;

    // Boolean types.
    let _t = true;
    let _f: bool = false;

    // Character type.
    let _char: char = 'z';

    // Tuple type.
    let tuple: (i32, char, bool) = (500, 'z', true);
    println!("tuple.0 = {}", tuple.0);

    // Tuple decompositon.
    let (x, y, z) = tuple;
    println!("x, y, z = ({}, {}, {})", x, y, z);

    // Array.
    let a = [1, 2, 3, 4, 5];
    println!("a = {:#?}", a);

    // Array explicit typed.
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    println!("b = {:#?}", b);

    // Array constant value initializer.
    let c = [7; 9];
    println!("c = {:#?}", c);

    // Array access.
    println!("a[2] = {}", a[2]);
}
