use rand::{thread_rng, Rng};

fn main() {
    // let x: i32 = 5;
    // println!("The value of x is {} ", x);
    // let x = "string";
    // println!("The value of x is {} ", x);

    // const SUBSCRIBER_COUNT: u32 = 10_000;

    // println!("Subs count: {SUBSCRIBER_COUNT}")

    // let tup = ("Hello", 3);

    // let (channel, sub_count) = tup;

    // println!("{:?}", tup);
    // println!("{}", channel);
    // println!("{}", sub_count);

    let square = my_func(5, 10);

    println!("Square in main: {}", square);

    let guess = 7;
    println!("\n\nNumber of tries to guess {} is {}", guess, rnd(guess));
}

fn rnd(x: i32) -> i32 {
    let mut rng = thread_rng();

    let mut loopcount = 1;
    let rnd = loop {
        loopcount += 1;
        let guess = rng.gen_range(0..=10);

        if guess == x {
            break loopcount;
        }
    };
    rnd
}

fn my_func(x: i32, y: i32) -> i32 {
    println!("x: {}\ny: {}", x, y);

    x * y
}
