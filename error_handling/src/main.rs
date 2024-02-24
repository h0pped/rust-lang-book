use std::fs::{self, File};
use std::io::Error;
use std::io::ErrorKind;
use std::io::{self, Read};

fn opening_file_closure(error: Error) -> File {
    if error.kind() == ErrorKind::NotFound {
        File::create("hello.txt").unwrap_or_else(|creation_error| {
            panic!("Problem creating file: {}", creation_error);
        })
    } else {
        panic!("Problem opening file: {}", error);
    }
}

fn main() {
    // a();

    let result = divide(5 as f32, 1 as f32);

    match result {
        Err(x) => println!("{}", x),
        Ok(x) => println!("Result is: {} ", x),
    }

    let f = File::open("hello.txt");

    let f = match f {
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {}", e),
            },
            other_error => panic!("Error opening file: {}", other_error),
        },
        Ok(x) => x,
    };

    let f = File::open("hello.txt").unwrap_or_else(opening_file_closure);

    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    println!("{:?}", f);

    let username = read_username_from_file().unwrap();
    println!("username: {}", username);
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn divide(a: f32, b: f32) -> Result<f32, String> {
    if b == 0.0 {
        return Err(String::from("Division by zero!"));
    }
    Ok(a / b)
}

// fn a() {
//     b();
// }
// fn b() {
//     c(22);
// }
// fn c(num: i32) {
//     panic!("Panic with {}", num)
// }
