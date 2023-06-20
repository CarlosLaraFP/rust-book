use std::ops::Deref;

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
/*
    Without the Deref trait, the compiler can only dereference & references.
    The deref method gives the compiler the ability to take a value of any type that implements
    Deref and call the deref method to get a & reference that it knows how to dereference.
 */
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}