use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    // let w = Wrapper(vec![String::from("Hello"), String::from("World")]);

    // println!("{}", w);

    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    type Thunk = Box<dyn Fn() + Send + 'static>;

    // println!("{} {}", x, y)

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {}
    // fn returns_long_type() -> Thunk {

    //     //snip
    // }
}

fn generic<T: ?Sized>(t: &T) {}
