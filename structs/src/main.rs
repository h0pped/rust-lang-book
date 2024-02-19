struct User {
    username: String,
    email: String,
}

fn build_user(email: String, username: String) -> User {
    User { email, username }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    // method is associated if no &self was passed as first arg
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut user1: User = build_user(String::from("notawril@gmail.com"), String::from("notawril"));

    let name = user1.username;
    user1.username = String::from("bob");

    println!("{}", name);

    let user2 = User {
        email: String::from("jjj@gmail.com"),
        ..user1
    };

    println!("{}", user2.email);

    struct Color(i32, i32, i32);

    let color1: Color = Color(5, 5, 5);

    println!("{} {} {} ", color1.0, color1.1, color1.2);

    let rect = Rectangle {
        width: 5,
        height: 10,
    };

    println!("rect: {:#?}", rect);
    println!("Area: {}", rect.area());

    let rect_smaller = Rectangle {
        width: rect.width - 1,
        height: rect.height - 1,
    };

    println!("Can hold smaller rect: {}", rect.can_hold(&rect_smaller));
    println!(
        "Can smaller rect hold larger: {}",
        rect_smaller.can_hold(&rect)
    );

    let square = Rectangle::square(5);

    println!("{:#?}", square);
}
