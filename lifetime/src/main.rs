struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    //'a is automatically added to self by compiler and all other references fill have the same lifetime as self
    fn return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        return x;
    }
    y
}

fn main() {
    let a = "abc";
    let b = "abcd";

    let mut ann = String::from("ANn!!");

    longest_with_an_announcement(a, b, &ann);

    ann = ann + "aaa";

    println!("Ann: {}", ann);

    //dangling reference - references to the value which doesn't live long enough because it is inside of the inner scope
    // let r;

    // {
    //     let x = 5;
    //     r = &x;
    // }

    //x is invalidaded and r references to it
    // println!("r:{}", r);

    let x = 5;

    let r = &x;

    println!("r: {}", r);

    let string1 = String::from("abcd");
    let result;
    let string2 = String::from("xyz");

    result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is: {}", result);

    let novel = String::from("call me ishmael. Some years ago...");
    let first_sentence = novel.split(".").next().expect("Could not find");

    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let sentence = "Hello my name is Illia";

    let first_sentence_word = first_word(sentence);

    println!("First: {}", first_sentence_word);

    let s: &'static str = "I have a static lifetime";
}

fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        return str1;
    }
    str2
    // str1

    // let result = String::from("really long string");
    // result
}
