fn main() {
    println!("Hello, world!");

    // {
    //     let s = String::from("hello");

    //     println!("{}", s);
    // }

    // println!("{}", s); //error because s was invalidaded

    let mut x = 5;
    let y = x; //copy as primitives implement copy trait
    println!("{} {}", x, y);
    x = 3;
    println!("{} {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1; //Not copy, value from s1 moved to s2 and s1 variable was invalidaded

    // println!("{}", s1); //error
    println!("{}", s2);

    let s3 = String::from("I am gonna be owned by a function");
    takes_ownership(s3);

    // println!("{}", s3); cannot be used as ownership was taken by a func

    let s4 = String::from("I am gonna be borrowed");
    borrows(&s4);
    println!("{}", s4); //should not panic as value was borrowed and returned

    let s5 = takes_ownership_and_gives_back(s4);

    // println!("{}", s4);not available as takes_ownership_and_gives_back() moved ownership to s5
    println!("{}", s5);

    let mut s = String::from("hello world");
    let s2 = "hello world";
    // let hello = &s[..5];
    // let world = &s[6..];

    let first_word_slice = first_word(s2);

    println!("{}", first_word_slice);

    let a = [1, 2, 3, 4, 5];
    let a_slice = &a[..2];
}

fn takes_ownership(string: String) {
    println!("{}", string);
}

fn borrows(string: &String) {
    println!("{}", string);
}

fn takes_ownership_and_gives_back(string: String) -> String {
    string
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
