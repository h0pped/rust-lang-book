fn find_largest<T: PartialOrd + Copy>(list: Vec<T>) -> Result<T, String> {
    if list.len() == 0 {
        return Err(String::from("Vec is empty!"));
    }

    let mut largest: T = list[0];
    for el in list {
        if el > largest {
            largest = el;
        }
    }
    Ok(largest)
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<U> Point<U> {
    fn x(&self) -> &U {
        &self.x
    }
}
impl Point<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 62, 87, 1];

    let largest_num = find_largest(number_list).unwrap_or(0);
    println!("Largest number is {}", largest_num);

    let chars_list = vec!['a', 'b', 'c', '1'];

    let largest_char = find_largest(chars_list).unwrap_or('-');
    println!("Largest char is {}", largest_char);

    let s = Point { x: 1.0, y: 2.0 };
    println!("{:?}", s);
    println!("{:?}", s.x());
    println!("{:?}", s.y());
    let s = Point { x: 1, y: 2 };
    println!("{:?}", s);

    let s2 = Point2 { x: 1.1, y: 2 };
    let s3 = Point2 { x: 1, y: "Hello" };

    let p3 = s2.mixup(s3);
    println!("{} {} ", p3.x, p3.y);
}
