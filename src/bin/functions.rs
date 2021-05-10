fn main() {
    another_function(5, 6);

    println!("The value of 5 + 1 == {}.", plus_one_expression(5));
    println!("The value of 8 + 1 == {}.", plus_one_statement(8));
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}.", x);
    println!("The value of y is: {}.", y);

    let z = {
        let k = x + 1;
        k * 8
    };
    println!("The value of z is: {}.", z);
}

fn plus_one_expression(x: i32) -> i32 {
    x + 1
}

fn plus_one_statement(x: i32) -> i32 {
    return x + 1;
}
