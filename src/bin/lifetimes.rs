struct Referencer<'a> {
    member: &'a str,
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let foo;
    {
        let string3 = String::from("xyz");
        foo = Referencer {
            member: string3.as_str(),
        };
        println!("{}", foo.member);
    }
    // println!("{}", foo.member); Cannot print borrowed member.
}
