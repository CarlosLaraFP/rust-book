/*
    Integration tests are entirely external to your library. They use your library in the same way
    any other code would, which means they can only call functions that are part of your library’s
    public API. Their purpose is to test whether many parts of your library work together correctly.
    Units of code that work correctly on their own could have problems when integrated, so test
    coverage of the integrated code is important as well.

    We create a tests directory at the top level of our project directory, next to src.
    Cargo knows to look for integration test files in this directory. We can then make as
    many test files as we want, and Cargo will compile each of the files as an individual crate.
    This is useful for creating separate scopes to more closely imitate the way end users will be
    using our crate.

    Each file in the tests directory is a separate crate, so we need to bring our library into each
    test crate’s scope. For that reason we add use the project itself at the top of the code,
    which we didn’t need in the unit tests.

    cargo test --test integration_test // runs a single integration test file
 */
mod common;

use rust_book::rectangle::*;

#[test]
fn identical_rectangles() {
    assert_eq!(Rectangle::new(2, 2), Rectangle::new(2, 2));
}

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, 2 + 2);
}
