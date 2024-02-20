// use rand::{CryptoRng, ErrorKind, ErrorKind::Transient, Rng};
// use std::io::*;

mod front_of_house;

pub use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// pub fn eat_at_restaurant() {
//     crate::front_of_house::hosting::add_to_waitlist();

//     front_of_house::hosting::add_to_waitlist();
// }

// fn serve_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::serve_order();
//     }
//     fn cook_order() {}
// }

// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }
//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }

//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
// }

// fn eat_at_restaurant() {
//     let mut meal = back_of_house::Breakfast::summer("default");

//     meal.toast = String::from("Wheat");

//     let order1 = back_of_house::Appetizer::Soup;
//     let order2 = back_of_house::Appetizer::Salad;
// }
