/*
    Structs let you create custom types that are meaningful for your domain.

    To understand when we might want to use structs,
    let’s write a program that calculates the area of a rectangle.

    In addition to the Debug trait, Rust has provided a number of traits for us to use with
    the derive attribute that can add useful behavior to our custom types. Those traits and their
    behaviors are listed in Appendix C. We’ll cover how to implement these traits with custom
    behavior as well as how to create your own traits in Chapter 10. There are also many attributes
    other than derive; for more information, see the “Attributes” section of the Rust Reference.
 */
#[derive(PartialEq, Debug)]
pub struct Rectangle {
    width: u32,
    height: u32
}
impl Rectangle {
    /*
        Often, but not always, when we give a method the same name as a field we want it to only
        return the value in the field and do nothing else. Methods like this are called getters,
        and Rust does not implement them automatically for struct fields as some other languages do.
        Getters are useful because you can make the field private but the method public, and thus
        enable read-only access to that field as part of the type’s public API. We will discuss what
        public and private are and how to designate a field or method as public or private in Chapter 7.

        When you call a method with object.something(), Rust automatically adds in
        &, &mut, or * so object matches the signature of the method.
     */
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle {
            width,
            height: dbg!(height * 2)
        }
    }

    /*
        accessing fields of a borrowed struct instance does not move the field values
        &self is short for self: &Self
        the type Self is an alias for the type that the impl block is for.
        Methods must have a parameter named self of type Self for their first parameter,
        so Rust lets you abbreviate this with only the name self in the first parameter spot.
        Note that we still need to use the & in front of the self shorthand to indicate that this
        method borrows the Self instance, just as we did in rectangle: &Rectangle. Methods can take
        ownership of self, borrow self immutably, as we’ve done here, or borrow self mutably,
        just as they can any other parameter.

        Here, we don’t want to take ownership, and we just want to read the data in the struct,
        not write to it. If we wanted to change the instance that we’ve called the method on as part
        of what the method does, we’d use &mut self as the first parameter. Having a method that
        takes ownership of the instance by using just self as the first parameter is rare; this
        technique is usually used when the method transforms self into something else and you want
        to prevent the caller from using the original instance after the transformation.
     */
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    /*
        We want an instance of Rectangle to take another instance of Rectangle and return
        true if the second Rectangle can fit completely within self (the first Rectangle);
        otherwise, it should return false.
        if both height and width of another are <= to self, return true; else false
     */
    pub fn can_hold(&self, another: &Rectangle) -> bool {
        self.width >= another.width && self.height >= another.height
    }

    // associated constructor function (i.e. static factory method)
    pub fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
