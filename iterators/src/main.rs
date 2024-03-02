#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for value in v1_iter {
        println!("value {}", value);
    }

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);

    let counter = Counter::new();

    let coutner_iter = counter.into_iter();

    for i in coutner_iter {
        println!("Counter: {}", i);
    }

    let sum: u32 = Counter::new().into_iter().map(|x| x * 2).sum();

    let sum2: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a + b)
        .sum();

    println!("sum: {}", sum);
    println!("sum2: {}", sum2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shoes_in_my_size_test() {
        let shoes = vec![
            Shoe {
                size: 1,
                style: String::from("Cool"),
            },
            Shoe {
                size: 1,
                style: String::from("not cool"),
            },
            Shoe {
                size: 2,
                style: String::from("Cool"),
            },
        ];

        let res = shoes_in_my_size(shoes, 1);
        assert_eq!(
            res,
            vec![
                Shoe {
                    size: 1,
                    style: String::from("Cool"),
                },
                Shoe {
                    size: 1,
                    style: String::from("not cool"),
                },
            ]
        )
    }
}
