fn add_once(x: i32) -> i32 {
    x + 1
}

fn do_twice<T>(f: T, arg: i32) -> i32
where
    T: Fn(i32) -> i32,
{
    f(arg) + f(arg)
}

fn num_to_string(num: i32) -> String {
    num.to_string()
}

//fn fnmut, fnonce

fn main() {
    println!("{}", add_once(10));

    println!("{}", do_twice(add_once, 10));
    println!("{}", do_twice(|a| a + 1, 10));

    let list_of_numbers = vec![1, 2, 3, 123];

    let list_of_strings: Vec<String> = list_of_numbers
        .clone()
        .into_iter()
        .map(num_to_string)
        .collect();

    let list_of_strings_closure: Vec<String> = list_of_numbers
        .clone()
        .into_iter()
        .map(|num| num.to_string())
        .collect();

    let list_of_strings_closure_trait: Vec<String> = list_of_numbers
        .clone()
        .iter()
        .map(ToString::to_string)
        .collect();

    println!("{:?}", list_of_strings);
    println!("{:?}", list_of_strings_closure);
    println!("{:?}", list_of_strings_closure_trait);

    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

fn returns_closure(a: i32) -> Box<dyn Fn(i32) -> i32> {
    if a > 0 {
        Box::new(move |b| a + b)
    } else {
        Box::new(move |b| a - b)
    }
}
