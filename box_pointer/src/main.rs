use std::ops::Add;

enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let list_float = Cons(1.1, Box::new(Cons(2.2, Box::new(Cons(3.3, Box::new(Nil))))));

    let sum_list = sum(&list);
    let sum_list_float = sum(&list_float);

    println!("SUM {}", sum_list);
    println!("SUM FLOAT {}", sum_list_float);
}

fn sum<T>(list: &List<T>) -> T
where
    T: Add<Output = T> + Default + Copy,
{
    fn sum_rec<T>(x: &List<T>, sum: T) -> T
    where
        T: Add<Output = T> + Default + Copy,
    {
        match x {
            Cons(val, node) => sum_rec(node, sum + *val),
            Nil => sum,
        }
    }

    sum_rec(list, T::default())
}
