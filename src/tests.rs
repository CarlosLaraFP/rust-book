/*
    Each test is run in a new thread, and when the main thread sees that a test thread has died,
    the test is marked as failed.

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
    use crate::rectangle::*; // mod rectangle must be in outer scope (lib.rs)
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
    fn another() {
        //panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let r = Rectangle::new(4, 4);
        let s = Rectangle::new(2, 2);
        assert!(r.can_hold(&s));
    }

    #[test]
    fn smaller_cannot_hold_smaller() {
        let r = Rectangle::new(4, 4);
        let s = Rectangle::new(2, 2);
        assert!(!s.can_hold(&r));
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
        The assert_ne! macro will pass if the two values we give it are not equal and fail if
        they’re equal. This macro is most useful for cases when we’re not sure what a value will be,
        but we know what the value definitely shouldn’t be. For example, if we’re testing a function
        that is guaranteed to change its input in some way, but the way in which the input is
        changed depends on the day of the week that we run our tests, the best thing to assert
        might be that the output of the function is not equal to the input.
     */
}

// module-specific tests can be organized in different mods (i.e. mod rectangle_tests { ... })

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}
