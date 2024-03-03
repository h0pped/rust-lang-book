struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping customSmartPointer with data {}!", self.data);
    }
}

fn main() {
    let mut c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    drop(c);
    let d = CustomSmartPointer {
        data: String::from("other "),
    };

    println!("CustomSmartPoitners created!");
}
