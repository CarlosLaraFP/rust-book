/*
    A trait object points to both an instance of a type implementing our specified trait and a
    table used to look up trait methods on that type at runtime. We create a trait object by
    specifying some sort of pointer, such as a & reference or a Box<T> smart pointer, then the
    dyn keyword, and then specifying the relevant trait. We can use trait objects in place of a
    generic or concrete type. Wherever we use a trait object, Rust’s type system will ensure at
    compile time that any value used in that context will implement the trait object’s trait,
    followed by type erasure (we don’t need to know all the possible types at compile time).

    Not using trait objects restricts us to instances that have a list of components all of type
    Button or all of type TextField. If you’ll only ever have homogeneous collections, using
    generics and trait bounds is preferable because the definitions will be monomorphized at
    compile time to use the concrete types. On the other hand, with the method using trait objects,
    one Screen instance can hold a Vec<T> that contains a Box<Button> as well as a Box<TextField>.
 */
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // Especially useful if we do not know all item sizes at compile time
    pub components: Vec<Box<dyn Draw>>,
}
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
        println!("Drawing Button...")
    }
}


pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}