fn take_ownership(some_string: String) {
    println!("some_string = {}", some_string);
}

fn gives_ownership() -> String {
    let some_string = String::from("foobar");
    some_string
}

fn take_and_return_ownership(some_string: String) -> String {
    some_string
}

fn calculate_length(some_string: String) -> (String, usize) {
    let length = some_string.len();
    (some_string, length)
}

fn main() {
    // "automatic" copying is cheap.  Ownership is transferred.
    {
        // a has ownership over the string memory.
        let a = String::from("hello");

        // b takes ownership from a.
        // This operation is called a "move".
        let _b = a;

        // Line below will not compile.
        //println!("a = {}", a);
    }

    // Deep copies require a explicit "clone" operation.
    {
        // a has ownership over the string memory.
        let a = String::from("hello");

        // Explicitly create a new copy memory, then let b assume ownership over
        // this new allocation.
        let b = a.clone();

        println!("a = {}, b = {}", a, b);
    }

    // Integers, for example, have the "Copy" trait.
    // This means memory is always (deep) copied and ownership
    // is not transferred when performing an assigmnent from
    // one variable to another.
    {
        // a has ownership over an int.
        let a = 1;

        // b has ownership over a new int.
        let b = a;

        println!("a = {}, b = {}", a, b);
    }

    // Functions take ownership over argument variables unless
    // the argument type has the Copy trait.
    {
        let a = String::from("hello");
        take_ownership(a);

        // Line below will not compile.
        //println!("a = {}", a);
    }

    // Functions can grant ownership of a value by returning a variable to the caller.
    {
        let a = gives_ownership();
        println!("a = {}", a);
    }

    // Functions take ownership of a value via argument variable then return it back to the caller.
    {
        let a = String::from("hello");
        let b = take_and_return_ownership(a);
        println!("b = {}", b);
    }

    // Functions can return tuples to give ownership of value back.
    // However, using reference types would be more concise.
    {
        let a = String::from("hello");
        let (b, length) = calculate_length(a);

        println!("b = {}, length = {}", b, length);
    }
}
