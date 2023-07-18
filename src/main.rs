/*
    By default, Rust has a set of items defined in the standard library that it brings into the
    scope of every program. This set is called the prelude.
 */
use std::fmt::Display;

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

use std::fs::File;
use std::io::{self, Read};
use rust_book::rectangle::*; // lib.rs has made this part of the public API with pub mod
use rust_book::shirts::*;
use rust_book::shoes::*;
use rust_book::smart_pointers::*;
use rust_book::oop::*;
use std::sync::{Arc, mpsc};
use std::thread;
use std::sync::Mutex;


fn main() -> Result<(), Box<dyn std::error::Error>> { // "catches" any kind of error
    /*
        When a main function returns a Result<(), E>, the executable will exit with a value of 0
        if main returns Ok(()) and will exit with a nonzero value if main returns an Err value.
        Executables written in C return integers when they exit: programs that exit successfully
        return the integer 0, and programs that error return some integer other than 0.
        Rust also returns integers from executables to be compatible with this convention.

        The main function may return any types that implement the std::process::Termination trait,
        which contains a function report that returns an ExitCode. Consult the standard library
        documentation for more information on implementing the Termination trait for your own types.
     */

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

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // Vectors can hold different variants of the same enum
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    /*
        String is implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities.
        String is a wrapper over a Vec<u8>
     */

    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    // strings are UTF-8 encoded, so we can include any properly encoded data in them
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Hola");
    /*
        Asked how long the string is, you might say 12. In fact, Rust’s answer is 24:
        that’s the number of bytes it takes to encode “Здравствуйте” in UTF-8, because each
        Unicode scalar value in that string takes 2 bytes of storage. Therefore, an index into
        the string’s bytes will not always correlate to a valid Unicode scalar value.
     */
    let hello = String::from("Здравствуйте");
    // s is a &str that contains the first 4 bytes of the string. Since each of these characters is 2 bytes, s == Зд
    let s = &hello[0..4];
    /*
        The best way to operate on pieces of strings is to be explicit about whether you want
        characters or bytes. For individual Unicode scalar values, use the chars method.
        Calling chars on “Зд” separates out and returns two values of type char,
        and you can iterate over the result to access each element.
        Alternatively, the bytes method returns each raw byte, which might be appropriate for your domain.
        Remember that valid Unicode scalar values may be made up of more than 1 byte
     */
    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }

    let mut s1 = String::from("foo");
    let s2 = "bar";
    // The push_str method takes a string slice because we don’t necessarily want to take ownership of the parameter
    s1.push_str(s2);
    println!("s2 is {s2}");
    // The push method takes a single character as a parameter and adds it to the String
    s1.push('l');
    println!("s1 is {s1}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // The reason we’re able to use &s2 in the call to add is that the compiler can coerce the
    // &String argument into a &str. When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..]
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // the code generated by the format! macro uses references so that this call doesn’t take ownership of any of its parameters
    let s = format!("{s1}-{s2}-{s3}");

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    let blue_team = "Blue";
    let yellow_team = "Yellow";

    scores.insert(blue_team, 10);
    scores.insert(yellow_team, 50);

    let score = scores
        .get(blue_team)
        .copied()
        .unwrap_or(0);

    println!("Blue team score: {score}");

    // prints in arbitrary order because HashMaps are unordered
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Overwriting a Value
    scores.insert(blue_team, 25);
    println!("{:?}", scores);

    /*
        Adding a Key and Value Only If a Key Isn’t Present

        The or_insert method on Entry is defined to return a mutable reference to the value for the
        corresponding Entry key if that key exists, and if not, inserts the parameter as the new
        value for this key and returns a mutable reference to the new value.
     */
    let x = scores
        .entry("Green")
        .or_insert(100);

    let y = scores
        .entry(blue_team)
        .or_insert(50);

    println!("{:?}", scores);

    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map
            .entry(word)
            .or_insert(0);

        *count += 1;
    }

    println!("{:?}", map);

    //let v = vec![1, 2, 3];
    // RUST_BACKTRACE=1 cargo run
    // run with `RUST_BACKTRACE=full` for a verbose backtrace
    //v[99];

    // like the Option enum, the Result enum and its variants have been brought into scope by the
    // prelude, so we don’t need to specify Result:: before the Ok and Err variants in the match arms
    let greeting_file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };

    // FP closures (methods) to clean up large nested match expressions when dealing with errors
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == io::ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    /*
        In production-quality code, most Rustaceans choose expect rather than unwrap and give more
        context about why the operation is expected to always succeed. That way, if your assumptions
        are ever proven wrong, you have more information to use in debugging.
     */
    // let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");
    // let greeting_file = File::open("hello.txt").unwrap();

    let greeting_file = std::fs::read_to_string("hello.txt")?;

    let home: std::net::IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");

    let number_list = vec![34, 50, 25, 100, 65];

    println!("The largest number is {}", largest(&number_list));

    let char_list = vec!['z', 'x', 'a', 'q'];

    println!("The largest char is {}", largest(&char_list));

    let mut p = Point { x: 5.0, y: 10.0 };
    println!("p.x = {}", p.x());

    println!("{}", p.distance_from_origin());

    p.set_x(99.9);
    println!("p.x = {}", p.x());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let tweet = Tweet {
        username: String::from("mountain_ebooks"),
        content: String::from(
            "of course, as you probably already know, nature",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    notify(&tweet);

    /*
        &i32        // a reference
        &'a i32     // a reference with an explicit lifetime
        &'a mut i32 // a mutable reference with an explicit lifetime
     */
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let novel = String::from("Call me Camila. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    /*
        One special lifetime we need to discuss is 'static, which denotes that the affected
        reference can live for the entire duration of the program. The text of this string is
        stored directly in the program’s binary, which is always available. Therefore,
        the lifetime of all string literals is 'static.
     */
    let s: &'static str = "I have a static lifetime.";

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        std::thread::sleep(std::time::Duration::from_secs(1));
        num
    };

    //println!("{}", expensive_closure(32));

    fn  add_one_v1(x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    /*
        let add_one_v3 = |x| { x + 1 };
        let add_one_v4 = |x| x + 1;
        The closures need to be evaluated to be able to compile because the types will be inferred
        from their usage. This is similar to let v = Vec::new(); needing either type annotations or
        values of some type to be inserted into the Vec for Rust to be able to infer the type.
     */

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From closure: {:?}", &list);
    println!("Before calling closure: {:?}", &list);
    only_borrows();
    println!("After calling closure: {:?}", &list);

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);
    // fails compilation because list is still borrowed mutably until the final closure call
    //println!("Before calling closure: {:?}", list);
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    /*
        force the closure to take ownership of the values it uses in the environment
        Capture a closure's environment by value. Keyword move converts any variables captured by
        reference or mutable reference to variables captured by value. This technique is mostly
        useful when passing a closure to a new thread to move the data so that it’s owned by the new thread.
     */
    std::thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    let mut list = [
        Rectangle::new(10, 1),
        Rectangle::new(3, 5),
        Rectangle::new(7, 12),
    ];

    let mut num_sort_operations = 0;

    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });

    println!("{:#?}, sorted in {num_sort_operations} operations", list);

    // Many iterator methods take closure arguments

    /*
        Calling the next method on an iterator changes internal state that the iterator uses to keep
        track of where it is in the sequence. In other words, this code consumes, or uses up, the
        iterator. Each call to next eats up an item from the iterator. We didn’t need to make
        v1_iter mutable when we used a for loop because the loop took ownership of v1_iter and
        made it mutable behind the scenes. Also note that the values we get from the calls to next
        are immutable references to the values in the vector. The iter method produces an iterator
        over immutable references. If we want to create an iterator that takes ownership of v1 and
        returns owned values, we can call into_iter instead of iter. Similarly, if we want to
        iterate over mutable references, we can call iter_mut instead of iter.
     */

    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    let mut v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter_mut();
    *v1_iter.next().unwrap() += 1;
    *v1_iter.next().unwrap() += 1;
    *v1_iter.next().unwrap() += 1;

    assert_eq!(&v1, &vec![2, 3, 4]);

    let x = 10;
    v1
        .iter()
        .map(|i| i + &x)
        .filter(|i| i < &(x + 4))
        .for_each(|j| println!("{j}"));

    // Smart pointer type that uniquely owns a heap allocation of type T
    // The box is stored on the stack and the data it points to is stored on the heap.
    // Pointers have a known size at compile time and therefore are stored on the stack at runtime.
    let b = Box::new(5);
    println!("b = {}", b);

    //let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    /*
        The main difference is that here we set y to be an instance of a Box<T> pointing to a
        copied value of x rather than a reference pointing to the value of x. In the last assertion,
        we can use the dereference operator to follow the pointer of the Box<T> in the same way
        that we did when y was a reference.
     */
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    /*
        deref coercion for flexible fn definitions

        When the Deref trait is defined for the types involved, Rust will analyze the types and use
        Deref::deref as many times as necessary to get a reference to match the parameter’s type.
        The number of times that Deref::deref needs to be inserted is resolved at compile time,
        so there is no runtime penalty for taking advantage of deref coercion!
     */
    let hello = |name: &str| println!("Hello, {name}!");
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let a = Rc::new(
        Cons(
            Rc::new(RefCell::new(5)),
            Rc::new(
                Cons(Rc::new(RefCell::new(10)), Rc::new(Nil))
            )
        )
    );
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    let a = Rc::new(Cons(Rc::new(RefCell::new(5)), Rc::new(Cons(Rc::new(RefCell::new(10)), Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("thread"),
            String::from("B"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
        // When the transmitter closes, recv will return an error to signal that no more values will be coming.
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("from"),
            String::from("thread"),
            String::from("C"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    // recv blocks the main thread’s execution and waits until a value is sent down the channel.
    // we’re not calling the recv function explicitly anymore: instead, we’re treating rx as an iterator.
    // For each value received, we’re printing it. When the channel is closed, iteration will end.
    for received in rx {
        println!("Thread A got: {received}");
    }

    // The type of m is Mutex<i32>, not i32, so we must call lock to be able to use the i32 value
    let m = Mutex::new(5);

    {
        /*
            The call to lock would fail if another thread holding the lock panicked.
            In that case, no one would ever be able to get the lock, so we’ve chosen
            to unwrap and have this thread panic if we’re in that situation.
            After we’ve acquired the lock, the return value is a mutable reference to the data inside.
            The call to lock returns a smart pointer called MutexGuard, wrapped in a LockResult that
            we handled with the call to unwrap. The MutexGuard smart pointer implements Deref to
            point at our inner data; the smart pointer also has a Drop implementation that releases
            the lock automatically when a MutexGuard goes out of scope, which happens at the end of
            the inner scope. As a result, we don’t risk forgetting to release the lock and blocking
            the mutex from being used by other threads, because the lock release happens automatically.
         */
        let mut num = m.lock().unwrap();
        *num += 5;
    }

    println!("m = {:?}", m);

    /*
        counter is immutable but we can get a mutable reference to the value inside it. This means
        Mutex<T> provides interior mutability. In the same way we used RefCell<T> to allow us to
        mutate contents inside an Rc<T>, we use Mutex<T> to mutate contents inside an Arc<T>
        Mutex<T> comes with the risk of creating deadlocks. These occur when an operation needs to
        lock two resources and two threads have each acquired one of the locks, causing them to wait
        for each other forever. The standard library API documentation for Mutex<T> and MutexGuard offers useful information.
     */
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    post.add_text("Trying to edit");
    assert_eq!("I ate a salad for lunch today", post.content());

    post.reject();
    assert_eq!("", post.content());

    let mut draft_post = MyPost::new();
    draft_post.add_text("Type System");
    let pending_post = draft_post.request_review();
    let published_post = pending_post.approve();
    assert_eq!("Type System", published_post.content());

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);

    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let Point { x, y } = Point { x: 0, y: 7 };
    assert_eq!(0, x);
    assert_eq!(7, y);

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis: ({x}, {y})")
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }

    // Complex pattern destructuring
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });


    Ok(())
}


struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
        println!("Drawing SelectBox...")
    }
}

/*
    RefCell<T> is useful when you are certain your code follows the borrowing rules but the
    compiler is unable to understand and guarantee that. With references and Box<T>, the borrowing
    rules’ invariants are enforced at compile time. With RefCell<T>, these invariants are enforced
    at runtime. With references, if you break these rules, you’ll get a compiler error. With
    RefCell<T>, if you break these rules, your program will panic and exit. RefCell<T> is only for
    use in single-threaded scenarios. RefCell<T> allows immutable or mutable borrows checked at runtime.

    Mutating the value inside an immutable value is the interior mutability pattern.
 */

use crate::List::{Cons, Nil};
/*
    We use the Rc<T> type when we want to allocate some data on the heap for multiple parts of our
    program to read and we can’t determine at compile time which part will finish using the data last.
    The implementation of the Drop trait decreases the reference count automatically when an Rc<T> value goes out of scope.
    Rc<T> is only for use in single-threaded scenarios and can replace the use of lifetime parameters / moves.
    Rc<T> enables passing references while avoiding lifetime parameters and avoiding moving ownership.
    If you have an Rc<T> that holds a RefCell<T>, you can get a value that can have multiple owners and that you can mutate.
 */
use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil
}

// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
    where
        T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    /*
        There are two input lifetimes, so Rust applies the first lifetime elision rule and gives
        both &self and announcement their own lifetimes. Then, because one of the parameters is
        &self, the return type gets the lifetime of &self, and all lifetimes have been accounted for.
     */
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    /*
        The signature means that the lifetime of the reference returned by the function is
        the same as the smaller of the lifetimes of the values referred to by the function arguments.
        The concrete lifetime that is substituted for 'a is the part of the scope of x that
        overlaps with the scope of y
        Lifetime syntax is about connecting the lifetimes of various parameters and return values of
        functions. Once they’re connected, Rust has enough information to allow memory-safe operations
        and disallow operations that would create dangling pointers or otherwise violate memory safety.
     */
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
/*
    fn some_function<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    { ... }
 */

/*
    The ability to specify a return type only by the trait it implements is especially useful in the
    context of closures and iterators. Closures and iterators create types that only the compiler
    knows or types that are very long to specify. The impl Trait syntax lets you concisely specify
    that a function returns some type that implements the Iterator trait without needing to write out a very long type.
 */
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// The impl Trait syntax works for straightforward cases but is syntax sugar for a longer form known as a trait bound

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify(item1: &impl Summary, item2: &impl Summary)

/*
    Public so that crates depending on this crate can make use of this trait too
    We can’t implement external traits on external types. For example, we can’t implement the
    Display trait on Vec<T> within our aggregator crate, because Display and Vec<T> are both
    defined in the standard library and aren’t local to our aggregator crate. This restriction
    is part of a property called coherence, and more specifically the orphan rule, so named because
    the parent type is not present. This rule ensures that other people’s code can’t break your code
    and vice versa. Without the rule, two crates could implement the same trait for the same type,
    and Rust wouldn’t know which implementation to use.
 */
pub trait Summary {
    // both return a heap String to be owned by the caller

    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("Written by {}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    /*
        We read this definition as: the function is generic over some type T that implements the
        PartialOrd trait. This function has one parameter named list, which is a slice of values of
        type T. The function will return a reference to a value of the same type T.
     */
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

struct Point<M, N> {
    x: M,
    y: N,
}
impl<M, N> Point<M, N> {
    fn x(&self) -> &M {
        &self.x
    }

    fn set_x(&mut self, new_value: M) {
        self.x = new_value
    }

    /*
        The purpose of this example is to demonstrate a situation in which some generic parameters
        are declared with impl and some are declared with the method definition. Here, the generic
        parameters X1 and Y1 are declared after impl because they go with the struct definition.
        The generic parameters X2 and Y2 are declared after fn mixup, because they’re only relevant to the method.
     */
    fn mixup<X, Y>(self, other: Point<X, Y>) -> Point<M, Y> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// We don’t have enough information on what the calling code is actually trying to do,
// so we propagate all the success or error information upward for it to handle appropriately.
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?
        .read_to_string(&mut username)?;

    Ok(username)
}
/*
    Reading a file into a string is a fairly common operation, so the standard library provides the
    convenient fs::read_to_string function that opens the file, creates a new String, reads the
    contents of the file, puts the contents into that String, and returns it.
    fs::read_to_string("hello.txt")
 */

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
//struct Point(i32, i32, i32);
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
