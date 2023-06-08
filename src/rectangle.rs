/*
    To understand when we might want to use structs,
    let’s write a program that calculates the area of a rectangle.
    We’ll start by using single variables, and then refactor the program until we’re using structs instead.
 */

#[derive(PartialEq, Debug)]
pub struct Rectangle {
    width: u32,
    height: u32
}
impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle {
            width,
            height: dbg!(height * 2)
        }
    }

    // accessing fields of a borrowed struct instance does not move the field values
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}
