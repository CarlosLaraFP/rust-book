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

    Recall in the “Performance of Code Using Generics” section in Chapter 10 our discussion on the
    monomorphization process performed by the compiler when we use trait bounds on generics: the
    compiler generates non-generic implementations of functions and methods for each concrete type
    that we use in place of a generic type parameter. The code that results from monomorphization
    is doing static dispatch, which is when the compiler knows what method you’re calling at compile
    time. This is opposed to dynamic dispatch, which is when the compiler can’t tell at compile time
    which method you’re calling. In dynamic dispatch cases, the compiler emits code that at runtime
    will figure out which method to call.

    When we use trait objects, Rust must use dynamic dispatch.
    The compiler doesn’t know all the types that might be used with the code that’s using trait
    objects, so it doesn’t know which method implemented on which type to call. Instead, at runtime,
    Rust uses the pointers inside the trait object to know which method to call.
    This lookup incurs a runtime cost that doesn’t occur with static dispatch. Dynamic dispatch
    also prevents the compiler from choosing to inline a method’s code, which in turn prevents
    some optimizations. However, we did get extra flexibility in the code that we wrote in Listing
    17-5 and were able to support in Listing 17-9, so it’s a trade-off to consider.
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

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String
}
impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new()
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text)
    }

    pub fn content (&self) -> &str {
        // unwrap will never panic because the methods on Post ensure that state
        // will always contain a Some value when those methods are done. This is one
        // of the cases we talked about in the “Cases In Which You Have More Information
        // Than the Compiler” section of Chapter 9 when we know that a None value is
        // never possible, even though the compiler isn’t able to understand that.
        self.state.as_ref().unwrap().content(self)
        // when we call content on the &Box<dyn State>, deref coercion will take effect on the & and
        // the Box so the content method will ultimately be called on the type that implements the State trait
    }

    pub fn request_review(&mut self) {
        /*
            This syntax takes ownership of Box<Self>, invalidating the old state
            so the state value of the Post can transform into a new state.
            We call the take method to take the Some value out of the state field and leave
            a None in its place, because Rust doesn’t let us have unpopulated fields in structs.
            This lets us move the state value out of Post rather than borrowing it.
            Then we’ll set the post’s state value to the result of this operation.
         */
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    // Default implementation keeps code ergonomic
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // Unlike OOP classes, structs and traits are independent, even though it's composition
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
