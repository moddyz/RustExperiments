fn main() {
    let x = 4;
    let compute = |y| y + x;
    println!("result: {}", compute(9));
    println!("result: {}", compute(7));
}
