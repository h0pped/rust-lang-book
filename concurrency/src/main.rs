use std::{thread, time::Duration};

use std::sync::mpsc;

fn main() {
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("spawned thread: {}", i);
    //         thread::sleep(Duration::from_millis(500));
    //     }
    // });

    // handle.join().unwrap();

    // for i in 1..5 {
    //     println!("main thread {}", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // let v = vec![1, 2, 3];

    // let handle = thread::spawn(move || {
    //     println!("Here is a vector: {:?}", v);
    // });

    // handle.join().unwrap();

    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("threads"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for you"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_millis(2500));
        }
    });

    for msg in rx {
        println!("{}", msg)
    }

    // loop {
    //     if let Ok(val) = rx.try_recv() {
    //         println!("received new msg: {}!", val)
    //     }
    //     println!("waiting...");
    //     thread::sleep(Duration::from_millis(500));
    // }
}
