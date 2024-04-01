use std::sync::{Arc, Mutex};
use std::thread;
fn main() {
    // let m = Mutex::new(5);

    // {
    //     let mut num = m.lock().unwrap();

    //     *num = 10;
    // }

    // println!("{:?}", m)
    let counter = Arc::new(Mutex::new(0));

    let mut threads = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            println!("incrementing num from {} thread", i);
            *num += 1;
        });
        threads.push(handle);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    println!("result: {:?}", counter);
}
