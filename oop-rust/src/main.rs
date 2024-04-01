use oop_rust::AveragedCollection;

use oop_rust::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        //draw selectBox
    }
}

fn main() {
    //     let mut collection = AveragedCollection::new();

    //     collection.add(1);
    //     collection.add(2);
    //     collection.add(3);

    //     println!("average {} ", collection.average());

    //     collection.remove();
    //     println!("average {} ", collection.average());

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 1,
                height: 1,
                options: vec![
                    String::from("option 1"),
                    String::from("option 2"),
                    String::from("option 3"),
                ],
            }),
            Box::new(Button {
                width: 1,
                height: 1,
                label: String::from("Button"),
            }),
        ],
    };

    screen.run()
}
