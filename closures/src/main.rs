use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("calculating slowly");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

struct Cacher<T, U>
where
    T: Fn(U) -> U,
{
    calculation: T,
    values: HashMap<U, U>,
}

impl<T, U> Cacher<T, U>
where
    T: Fn(U) -> U,
    U: Hash + Eq + PartialEq + Copy,
{
    fn new(calculation: T) -> Cacher<T, U> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn generate_new_value(&mut self, arg: U) -> U {
        let val = (self.calculation)(arg);

        self.values.entry(arg).or_insert(val);
        val
    }
    fn get_value(&mut self, arg: U) -> U {
        if let Some(value) = self.values.get(&arg) {
            return *value;
        } else {
            let value = self.generate_new_value(arg);
            self.values.insert(arg, value);
            return value;
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut cached_result = Cacher::new(|num: u32| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("{} pushups", cached_result.get_value(random_number));
        println!("{} situps!", cached_result.get_value(random_number + 1));
        println!("{} mins rest!", cached_result.get_value(random_number + 1));
    } else {
        if random_number == 3 {
            println!("Take a break today!");
        } else {
            println!("run for {} minutes", cached_result.get_value(random_number));
        }
    }
}

fn main() {
    let x = 4;
    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));

    let simulated_intensity = 22;
    let simulated_random_number = 3;

    generate_workout(simulated_intensity, simulated_random_number);
}
