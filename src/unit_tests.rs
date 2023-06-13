/*
    Unit unit_tests are small and more focused, testing one module in isolation at a time, and can test
    private interfaces. Integration unit_tests are entirely external to your library and use your code
    in the same way any other external code would, using only the public interface and potentially
    exercising multiple modules per test.

    Unit tests are placed in the src directory in each file with the code that they’re testing.
    The convention is to create a module named tests in each file to contain the test functions
    and to annotate the module with cfg(test). Because integration tests go in a different directory,
    they don’t need the #[cfg(test)] annotation. However, because unit tests go in the same files as
    the code, you’ll use #[cfg(test)] to specify that they shouldn’t be included in the compiled result.

    Each test is run in parallel in a new thread, and when the main thread sees that a test thread
    has died, the test is marked as failed. To run unit_tests sequentially, run:
    cargo test -- --test-threads=1
    cargo test -- --show-output // to show println! calls
    cargo test -- --include-ignored or cargo test -- --ignored
    cargo test one_hundred // runs a single test

    We can also specify part of a test name, and any test whose name matches that value will be run.
    The module in which a test appears becomes part of the test’s name,
    so we can run all the unit_tests in a module by filtering on the module’s name!

    Under the surface, the assert_eq! and assert_ne! macros use the operators == and !=, respectively.
    When the assertions fail, these macros print their arguments using debug formatting, which means
    the values being compared must implement the PartialEq and Debug traits. All primitive types and
    most of the standard library types implement these traits. For structs and enums that you define
    yourself, you’ll need to implement PartialEq to assert equality of those types. You’ll also need
    to implement Debug to print the values when the assertion fails. Because both traits are derivable
    traits, this is usually as straightforward as adding the #[derive(PartialEq, Debug)] annotation
    to your struct or enum definition. See Appendix C, “Derivable Traits,” for more details about
    these and other derivable traits.
 */

#[cfg(test)]
mod tests {
    // corresponding mod declarations must be in lib.rs
    use crate::guessing_game::*;
    //use super::*; // only counts for the current file scope

    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, super::add_two(2));
    }

    #[test]
    #[ignore]
    fn another() {
        panic!("Make this test fail");
    }

    // We can see the value we actually got in the test output, which would help us debug
    // what happened instead of what we were expecting to happen.
    #[test]
    fn greeting_contains_name() {
        let result = super::greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    /*
        Tests that use should_panic can be imprecise. A should_panic test would pass even if the
        test panics for a different reason from the one we were expecting. To make should_panic
        unit_tests more precise, we can add an optional expected parameter to the should_panic attribute.
        The test harness will make sure that the failure message contains the provided text.

        This test will pass because the value we put in the should_panic attribute’s expected
        parameter is a substring of the message that the Guess::new function panics with. We could
        have specified the entire panic message that we expect, which in this case would be Guess
        value must be less than or equal to 100, got 200. What you choose to specify depends on how
        much of the panic message is unique or dynamic and how precise you want your test to be.
        In this case, a substring of the panic message is enough to ensure that the code in the
        test function executes the else if value > 100 case.
     */
    #[test]
    #[should_panic(expected = "value must be between 1 and 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    /*
        Writing tests so they return a Result<T, E> enables you to use the question mark operator
        in the body of tests, which can be a convenient way to write tests that should fail if any
        operation within them returns an Err variant. You can’t use the #[should_panic] annotation
        on tests that use Result<T, E>. To assert that an operation returns an Err variant, don’t use
        the question mark operator on the Result<T, E> value. Instead, use assert!(value.is_err())
     */
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    /*
        The assert_ne! macro will pass if the two values we give it are not equal and fail if
        they’re equal. This macro is most useful for cases when we’re not sure what a value will be,
        but we know what the value definitely shouldn’t be. For example, if we’re testing a function
        that is guaranteed to change its input in some way, but the way in which the input is
        changed depends on the day of the week that we run our unit_tests, the best thing to assert
        might be that the output of the function is not equal to the input.
     */
}

// module-specific unit_tests can be organized in different mods (i.e. mod rectangle_tests { ... })

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

// private functions can be tested as easily as public functions
fn add_two(a: i32) -> i32 {
    a + 2
}
