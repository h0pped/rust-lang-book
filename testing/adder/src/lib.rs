pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn substract(left: i32, right: i32) -> i32 {
    left - right
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_fit(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

pub fn greetings(name: &str) -> String {
    // format!("hello {}!", name)
    format!("hello!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100!")
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_substract() {
        let result = substract(10, 11);

        assert_eq!(result, -1);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            height: 100,
            width: 100,
        };
        let smaller = Rectangle {
            height: 50,
            width: 40,
        };

        assert!(larger.can_fit(&smaller));
    }
    #[test]
    fn smaller_can_not_hold_larger() {
        let larger = Rectangle {
            height: 100,
            width: 100,
        };
        let smaller = Rectangle {
            height: 50,
            width: 40,
        };
        assert!(!smaller.can_fit(&larger));
    }
    #[test]
    fn cannot_hold_same_size() {
        let rect = Rectangle {
            height: 10,
            width: 10,
        };
        let res = rect.can_fit(&rect);

        assert_eq!(res, false);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greetings("Bob");

        assert!(
            result.contains("Bob"),
            "Greetings did not contain provided name: {}",
            "Bob"
        );
    }

    #[test]
    fn guess_should_not_panic() {
        let res = Guess::new(32);

        assert_eq!(res.value, 32);
    }
    #[test]
    #[should_panic(expected = "between 1 and 100!")]
    fn guess_should_panic_above_100() {
        Guess::new(101);
    }
    #[test]
    #[should_panic(expected = "between 1 and 100!")]

    fn guess_should_panic_below_1() {
        Guess::new(-1);
    }

    #[test]
    fn it_works_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal 4"))
        }
    }

    // #[test]
    // fn failing_test() {
    //     panic!("test panic!");
    // }
}
