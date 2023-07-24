//! # Rust Book
//!
//! `rust_book` contains the book's code written by hand.
//! Projects are separated into their own crates, in their own GitHub repos.
//! Documentation comments within items are useful for describing crates and modules especially.
//! Use them to explain the overall purpose of the container to help your users understand the crate’s organization.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

// Modules can also hold definitions for other items, such as structs, enums, constants, traits
// src/main.rs and src/lib.rs are called crate roots.
// The reason for their name is that the contents of either of these two files form a module
// named crate at the root of the crate’s module structure, known as the module tree.

/*
    Packages with this pattern of containing both a library and a binary crate will have just
    enough code in the binary crate to start an executable that calls code with the library crate.
    This lets other projects benefit from the most functionality that the package provides,
    because the library crate’s code can be shared. The module tree should be defined in src/lib.rs.
    Then, any public items can be used in the binary crate by starting paths with the name of the package.
    The binary crate becomes a user of the library crate just like a completely external crate would
    use the library crate: it can only use the public API. This helps you design a good API;
    not only are you the author, you’re also a client!
 */

/*
    If you plan on sharing your library crate so other projects can use your code,
    your public API is your contract with users of your crate that determines how
    they can interact with your code. There are many considerations around managing
    changes to your public API to make it easier for people to depend on your crate.
    If you’re interested in this topic, see The Rust API Guidelines.
 */

/*
    While front_of_house isn’t public, because the eat_at_restaurant function is defined in the
    same module as front_of_house (that is, eat_at_restaurant and front_of_house are siblings),
    we can refer to front_of_house from eat_at_restaurant
 */
mod front_of_house;

pub use crate::front_of_house::hosting;

use std::collections::*; // glob operator
//use std::{cmp::Ordering, io};
use std::io::{self, Write}; // brings std::io and std::io::Write into scope

// lib.rs controls module visibility (pub => available for integration testing and external users)
mod unit_tests;
pub mod rectangle;
pub mod guessing_game;
pub mod shirts;
pub mod shoes;
pub mod smart_pointers;
pub mod interior_mutability;
pub mod oop;
pub mod macros;

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = rust_book::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
/*
    cargo doc --open

    We used the # Examples Markdown heading to create a section in the HTML with the title “Examples.”
    Here are some other sections that crate authors commonly use in their documentation:

    Panics: The scenarios in which the function being documented could panic. Callers of the function
    who don’t want their programs to panic should make sure they don’t call the function in these situations.

    Errors: If the function returns a Result, describing the kinds of errors that might occur and
    what conditions might cause those errors to be returned can be helpful to callers so they can
    write code to handle the different kinds of errors in different ways.

    Safety: If the function is unsafe to call (we discuss unsafety in Chapter 19), there should be a section
    explaining why the function is unsafe and covering the invariants that the function expects callers to uphold.

    Most documentation comments don’t need all of these sections, but this is a good checklist to
    remind you of the aspects of your code users will be interested in knowing about.
 */

/*
    Our preference in general is to specify absolute paths because it’s more likely
    we’ll want to move code definitions and item calls independently of each other.
 */
pub fn eat_at_restaurant() {
    // use only creates the shortcut for the particular scope in which the use occurs
    hosting::add_to_waitlist();
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // In contrast, if we make an enum public, all of its variants are then public. We only need the pub before the enum keyword.
    pub enum Appetizer {
        Soup,
        Salad,
    }

    /*
         We think the back_of_house module and the deliver_order function are likely to stay
         in the same relationship to each other and get moved together should we decide to
         reorganize the crate’s module tree. Therefore, we used super so we’ll have fewer
         places to update code in the future if this code gets moved to a different module.
     */
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

fn deliver_order() {}

use std::fmt;
use std::io::Result as IoResult;

fn function1() -> fmt::Result {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Purple
    }
}
