fn main() {
    enum Language {
        English,
        Russian,
        Japanese,
    }

    let language = Language::Japanese;

    match language {
        Language::English => println!("hello World!"),
        Language::Russian => println!("привет!"),
        _ => println!("Unsupported language!"),
    }

    let authorization_status: Option<&str> = None;
    let is_admin = false;
    let group_id: Result<u8, _> = "34".parse();

    if let Some(status) = authorization_status {
        println!("Auth status: {}", status);
    } else if is_admin {
        println!("admin");
    } else if let Ok(group_id) = group_id {
        println!("belongs to {} group", group_id)
    }

    let mut stack: Vec<i32> = vec![];

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(x) = stack.pop() {
        println!("x: {}", x);
    }

    let (x, y, z) = (1, 2, 3);

    println!("{} {} {}", x, y, z);
}
