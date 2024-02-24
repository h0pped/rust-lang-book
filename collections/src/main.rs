use unicode_segmentation::UnicodeSegmentation;

use std::collections::HashMap;

fn main() {
    // let a = [1, 2, 3];

    // let mut v: Vec<i32> = Vec::new();
    // v.push(1);
    // v.push(2);
    // v.push(3);

    // {
    //     let v2 = vec![1, 2, 3];
    // }

    // enum Cell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }
    // let v = vec![
    //     Cell::Int(3),
    //     Cell::Float(1.23),
    //     Cell::Text(String::from("hello")),
    // ];

    // for i in &v {
    //     match i {
    //         Cell::Int(x) => println!("{}", x),
    //         Cell::Float(x) => println!("{}", x),
    //         Cell::Text(x) => println!("{}", x),
    //     }
    // }

    //string is a collection of UTF-8 encoded bytes

    // let s1 = String::new();
    // let s2 = "hello";
    // let s3 = s2.to_string();
    // let s4 = String::from("hello from &str");

    // let mut s = String::from("hell");

    // s.push_str("o world");
    // s.push('!');

    // let s3 = String::from("hello ") + &String::from("world!");

    // let s4 = format!("{}{}", s, "aaa");

    // println!("{}", s3);
    // println!("{}", s4)

    // for b in String::from("Привет").graphemes(true) {
    //     println!("{}", b);
    // }

    // let blue = String::from("Blue");
    // let yellow = String::from("Yellow");

    // let mut scores = HashMap::new();

    // scores.insert(blue, 10);
    // scores.insert(yellow, 50);

    // let team_name = String::from("Blue");
    // println!("{}", scores.get(&team_name).unwrap_or(&0));

    // for (key, value) in &scores {
    //     println!("{}: {}", key, value);
    // }

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Blue"), 20);

    // scores.entry(String::from("Yellow")).or_insert(30);
    // scores.entry(String::from("Yellow")).or_insert(40);

    // for (key, value) in &scores {
    //     println!("{}: {}", key, value);
    // }

    let text = "hello world wonderful world ";

    let mut map: HashMap<String, u32> = HashMap::new();

    for word in text.trim().split_whitespace() {
        let count = map.entry(String::from(word)).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
