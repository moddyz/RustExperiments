use std::error::Error;
use std::fs;
use std::io;
use std::io::Read;

fn read_username_from_file_super_concise() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn read_username_from_file_concise() -> Result<String, io::Error> {
    let mut s = String::new();
    fs::File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = fs::File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_verbose() -> Result<String, io::Error> {
    let f = fs::File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = fs::File::open("hello.txt");
    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => match fs::File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file: {:?}", e),
            },
            _other_error => panic!("Error opening file: {:?}", error),
        },
    };

    let _f = fs::File::open("hello.txt").expect("error opening recently created hello.txt!");

    Ok(())
}
