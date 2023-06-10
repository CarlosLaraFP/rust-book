/*
    By default, Rust has a set of items defined in the standard library that it brings into the
    scope of every program. This set is called the prelude.
 */

mod guessing_game;
mod rectangle;
use rectangle::*;

// sub-module
use crate::garden::vegetables::Asparagus;
// public module
pub mod garden;
/*
    When we bring a name into scope with the use keyword, the name available in the new scope
    is private. To enable the code that calls our code to refer to that name as if it had been
    defined in that code’s scope, we can combine pub and use. This technique is called re-exporting
    because we’re bringing an item into scope but also making that item available for others to
    bring into their scope.

    Re-exporting is useful when the internal structure of your code is different from how
    programmers calling your code would think about the domain. For example, in this restaurant
    metaphor, the people running the restaurant think about “front of house” and “back of house.”
    But customers visiting a restaurant probably won’t think about the parts of the restaurant in
    those terms. With pub use, we can write our code with one structure but expose a different
    structure. Doing so makes our library well organized for programmers working on the library
    and programmers calling the library.
 */


fn main() {
    // match guessing_game::start_game() {
    //     Err(error) => println!("{error}"),
    //     _ => ()
    // };

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    /*
        Arrays in Rust have a fixed length. An array is a single chunk of memory of a known,
        fixed size that can be allocated on the stack.
        Arrays are useful when you want your data allocated on the stack rather than the heap
        or when you want to ensure you always have a fixed number of elements.
        You can also initialize an array to contain the same value for each element by specifying the
        initial value, followed by a semicolon, and then the length of the array in square brackets.

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    //let a = [3; 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
    */
    // Rust has three kinds of loops: loop, while, and for

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };

    println!("The result is {result}");

    let mut count = 0;

    // loop label for nested loops
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    /*
        The safety and conciseness of for loops make them the most commonly used loop construct in Rust.
        Even in situations in which you want to run some code a certain number of times, as in the
        countdown example that used a while, most Rustaceans would use a for loop. The way to do
        that would be to use a Range, provided by the standard library, which generates all numbers
        in sequence starting from one number and ending before another number.
     */

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let mut s = String::from("hello");

    s.push_str("twice");

    change(&mut s);

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM IF r1 AND r2 are referenced afterwards

    println!("{r3}");

    /*
        Write a function that takes a string of words separated by spaces and returns
        the first word it finds in that string. If the function does not find a space
        in the string, the whole string must be one word, so the entire string should be returned.
     */
    let slice_one = "The lowest level is the quantum processor";
    let slice_two = "Quantum";

    let moved_one = first_word(slice_one);
    let moved_two = first_word(slice_two);

    assert_eq!(moved_one, "The");
    assert_eq!(moved_two, slice_two);

    let s = String::from("hello world of software");

    let hello = &s[..5];
    let world = &s[6..11];
    let software = &s[15..];
    let slice = &s[..];

    println!("{hello} {world} {software}");
    println!("{}", first_word(&s));

    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    //println!("the first word is: {}", word);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

    let mut user1 = User::build_user(
        String::from("someusername123"),
     String::from("someone@example.com")
    );

    user1.email = String::from("anotheremail@example.com");

    // The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{}", user2.email);
    // Both active and sign_in_count are types that implement the Copy trait, so the behavior we discussed in the “Stack-Only Data: Copy” section would apply.
    // Compilation error if we try to access username because it's been moved into user2 (ownership transferred)
    println!("{}", user1.email);

    // black and origin values are different types because they’re instances of different tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;

    let rectangle_a = Rectangle::new(2, 2);
    let rectangle_b = Rectangle::new(2, 2);
    let rectangle_c = Rectangle::new(4, 4);

    println!("{:#?}", rectangle_a); // println! macro automatically adds & to the object
    println!("{:?}", rectangle_b);
    println!("{}", rectangle_c.area());
    assert!((rectangle_a == rectangle_b));
    assert!(!(rectangle_a == rectangle_c));
    assert!((rectangle_b == rectangle_a));
    assert!(!(rectangle_b == rectangle_c));
    assert!(!(rectangle_c == rectangle_b));
    assert!((rectangle_c == rectangle_c));
    /*
        Another way to print out a value using the Debug format is to use the dbg! macro,
        which takes ownership of an expression (as opposed to println!, which takes a reference),
        prints the file and line number of where that dbg! macro call occurs in your code along
        with the resultant value of that expression, and returns ownership of the value.

        Calling the dbg! macro prints to the standard error console stream (stderr),
        as opposed to println!, which prints to the standard output console stream (stdout).
        The dbg! macro can be really helpful when you’re trying to figure out what your code is doing!
     */
    dbg!(&rectangle_a);

    assert!(rectangle_a.can_hold(&rectangle_b));
    assert!(!rectangle_a.can_hold(&rectangle_c));
    assert!(!rectangle_b.can_hold(&rectangle_c));
    assert!(rectangle_c.can_hold(&rectangle_a));
    assert!(rectangle_c.can_hold(&rectangle_b));

    //  We automatically get constructor functions defined as a result of defining the enum.
    let four = IpAddr::V4(Ipv4Addr(127, 0, 0, 1));
    let six = IpAddr::V6(Ipv6Addr(String::from("::1")));

    route(&four);
    route(&six);

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    /*
        Option<T> does not implement the Copy trait by default.
        Therefore, Option<T> parameters are moved, not copied.

        The Copy trait is only implemented for types that can be safely copied with a simple bit-wise copy,
        without needing to run any special logic. This includes basic numeric types, references, and other types that do not own heap data.

        In the case of Option<T>, whether it implements Copy depends on whether T implements Copy.
        If T is a type that implements Copy, then Option<T> will also implement Copy.
        If T does not implement Copy (for example, if T is a String, Vec<T>, or any other type owning heap data),
        then Option<T> will not implement Copy.
     */

    let coin = Coin::Quarter(UsState::Washington);
    let x = value_in_cents(&coin);
    println!("{x}");

    println!("{:?}", plus_one(Some(99)));
    println!("{:?}", plus_one(None));

    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // The code in the if let block isn’t run if the value doesn’t match the pattern.
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let coins = [Coin::Penny, Coin::Nickel, Coin::Dime, coin];
    let mut count = 0;

    for coin in coins {
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}!", state);
        } else {
            count += 1;
        }
    }

    println!("{count}");

    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);

    // Vectors allow you to store more than one value in a single data structure that
    // puts all the values next to each other in memory.
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    // We can also iterate over mutable references to each element in a mutable vector to make changes to all the elements
    // To change the value that the mutable reference refers to, we have to use
    // the * dereference operator to get to the value in i before we can use the += operator.
    for i in &mut v {
        *i += 50; // the dereference operator follows the pointer to the value
    }

    // There are two ways to reference a value stored in a vector: via indexing or using the get method

    let third: &i32 = &v[2];

    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);

    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}



fn plus_one(x: Option<i32>) -> Option<i32> {
    Some(x? + 1)
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
    Washington
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn route(ip_kind: &IpAddr) {
    match ip_kind {
        IpAddr::V4(kind) => println!("V4 -> {}.{}.{}.{}", kind.0, kind.1, kind.2, kind.3),
        IpAddr::V6(kind) => println!("V6 -> {}", kind.0)
    }
}

// another advantage to using an enum rather than a struct: each variant can have different types and amounts of associated data
struct Ipv4Addr(u8, u8, u8, u8);

struct Ipv6Addr(String);

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
impl User {
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }
}

/*
    Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different
    type from other tuples, and when naming each field as in a regular struct would be verbose or redundant.
    Tuple struct instances are similar to tuples in that you can deconstruct them into their individual pieces,
    and you can use a . followed by the index to access an individual value.
    Unit-like structs can be useful when you need to implement a trait on some type
    but don’t have any data that you want to store in the type itself
 */
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

fn first_word(slice: &str) -> &str {
    match slice.split_whitespace().rev().last() {
        Some(word) => word,
        _ => slice
    }
}

fn first_word_book(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
