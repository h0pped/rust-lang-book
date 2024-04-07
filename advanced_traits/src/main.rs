// use std::path::Iter;

use std::fmt;
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
impl Add<u32> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: u32) -> Millimeters {
        Millimeters(self.0 + other)
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Human {
    fn fly(&self) {
        println!("*waving arms");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

// pub trait Iterator<T> {
//     // type Item;

//     fn next(&mut self) -> Option<T>;
// }

// struct Counter {}

// impl Iterator<u32> for Counter {
//     // type Item = u32;

//     fn next(&mut self) -> Option<u32> {
//         Some(0)
//     }
// }

// impl Iterator<u16> for Counter {
//     // type Item = u16;

//     fn next(&mut self) -> Option<u16> {
//         Some(0)
//     }
// }

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let x = Point { x: 1, y: 1 };
    let y = Point { x: 2, y: 2 };
    println!("{:?}", x + y);

    let mut mm = Millimeters(100);
    let m = Meters(20);

    let res = mm + m;

    // mm = mm + 10;

    println!("{:?}", res.0);

    let person = Human;

    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);

    let x = Point { x: 1, y: 1 };
    x.outline_print()
}
