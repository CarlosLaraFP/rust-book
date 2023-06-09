// Modules can also hold definitions for other items, such as structs, enums, constants, traits
// src/main.rs and src/lib.rs are called crate roots.
// The reason for their name is that the contents of either of these two files form a module
// named crate at the root of the crate’s module structure, known as the module tree.

/*
    While front_of_house isn’t public, because the eat_at_restaurant function is defined in the
    same module as front_of_house (that is, eat_at_restaurant and front_of_house are siblings),
    we can refer to front_of_house from eat_at_restaurant
 */
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

/*
    Our preference in general is to specify absolute paths because it’s more likely
    we’ll want to move code definitions and item calls independently of each other.
 */
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}