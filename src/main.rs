/*
    By default, Rust has a set of items defined in the standard library that it brings into the
    scope of every program. This set is called the prelude.
 */

mod guessing_game;
mod rectangle;

use rectangle::*;


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

    println!("{:?}, {}, {}", rectangle_a, rectangle_a == rectangle_b, rectangle_a == rectangle_c);
    println!("{:?}, {}, {}", rectangle_b, rectangle_b == rectangle_a, rectangle_b == rectangle_c);
    println!("{:?}, {}, {}", rectangle_c, rectangle_c == rectangle_b, rectangle_c == rectangle_a);
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
