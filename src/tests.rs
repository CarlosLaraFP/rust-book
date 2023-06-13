/*
    Each test is run in a new thread, and when the main thread sees that a test thread has died,
    the test is marked as failed.
 */

#[cfg(test)]
mod tests {
    use crate::rectangle::*; // mod rectangle must be in outer scope (lib.rs)

    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        //panic!("Make this test fail");
    }

    #[test]
    fn can_hold_test() {
        let r: Rectangle = Rectangle::new(4, 4);
        let s: Rectangle = Rectangle::new(2, 2);
        assert!(r.can_hold(&s));
    }
}