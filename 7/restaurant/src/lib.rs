mod front_of_house;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }
    fn cook_order() {}
}

// absolut use. When pub, the user of this crate also can use it
pub use crate::front_of_house::hosting;

// relative use
// use self::front_of_house::hosting;

// not idiomatic for functions: unclear where add_to_waitlist is defined
// use crate::front_of_house::hosting::add_to_waitlist;

// but idiomatic for structs, enums etc.
use std::collections::HashMap;

use std::fmt::Result;
use std::io::Result as IoResult;

// nested paths in use:
// use std::cmp::Ordering;
// use std::io;
// same as:
use std::{cmp::Ordering, io};

// also:
// use std::io;
// use std::io::Write;
// same as:
// use std::io::{self, Write};

// glob:
use std::collections::*;

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order1 = back_of_house::Appetizer::Salad;
}
