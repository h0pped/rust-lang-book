// enum IpAddrKind {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn some_function() {
//         println!("Hello from impl Message!");
//     }
// }

// struct IpAddress {
//     kind: IpAddrKind,
//     address: String,
// }

#[derive(Debug)]
enum Color {
    Black,
    White,
    Red,
    Blue,
    Green,
}

#[derive(Debug)]
struct Rgb(i16, i16, i16);

fn main() {
    // let localhost = IpAddrKind::V4(127, 0, 0, 1);

    // route(localhost);

    // let some_number = Some(5);
    // let some_string = Some("a string");

    // let none: Option<i32> = None;

    // let x: i8 = 5;
    // let y: Option<i8> = None;

    // let sum = x + y.unwrap_or(0);

    println!("255,0,0 to color: {:?}", rgb_to_color(Rgb(255, 255, 255)));
    println!("Black to rgb: {:?}", color_to_rgb(Color::Black));

    let val = Some(19);

    println!("plus one: {}", plus_one(val).unwrap());
    println!("plus one after sending to plus_one(): {}", val.unwrap());
    println!("plus one on none: {}", plus_one(None).unwrap_or(0));

    println!("{:?}", plus_one_if_let(Some(3)));
    println!("{:?}", plus_one_if_let(Some(5)));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(val) => Some(val + 1),
        _ => None,
    }
}

fn plus_one_if_let(x: Option<i32>) -> Option<i32> {
    if let Some(3) = x {
        println!("Three!");
        return Some(3);
    }
    None
}

fn color_to_rgb(color: Color) -> Rgb {
    match color {
        Color::Black => Rgb(0, 0, 0),
        Color::White => Rgb(255, 255, 255),
        Color::Blue => Rgb(0, 0, 255),
        Color::Red => Rgb(255, 0, 0),
        Color::Green => Rgb(0, 255, 0),
    }
}

fn rgb_to_color(rgb: Rgb) -> Color {
    match rgb {
        Rgb(0, 0, 0) => Color::White,
        Rgb(255, 255, 255) => Color::Black,
        Rgb(0, 0, 255) => Color::Blue,
        Rgb(0, 255, 0) => Color::Green,
        Rgb(255, 0, 0) => Color::Red,
        _ => Color::White,
    }
}

// fn route(ip_kind: IpAddrKind) {}
