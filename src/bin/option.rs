fn main() {
    let x: Option<i8> = None;
    println!("x.is_some == {}", x.is_some());

    let y: Option<i8> = Some(8);
    println!("y.is_some == {}", y.is_some());
    println!("y.take() == {}", y.unwrap());
}
