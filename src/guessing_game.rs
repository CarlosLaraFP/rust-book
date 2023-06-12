use std::io;
//use anyhow::anyhow; // Only use anyhow! macro if no default implementation is already provided by the crate
/*
    The logic behind the anyhow crate is that it provides its own error type. This type has
    pretty-printing properties and can easily be converted from other errors, like std::io::Error.
    It's easy to add anyhow to our project. All we have to do is place it as the return type of the
    main function. Because most error types can be converted to anyhow::Error, we can use ? syntax
    to remove the expect calls from our code. Also, note that we can use the anyhow! macro to
    produce an anyhow::Error on the fly that contains the provided error message. Now every panic
    message caused by an I/O error being returned from within our program will be displayed user-friendly.
 */
use rand::Rng; // The Rng trait defines methods that random number generators implement
use std::cmp::Ordering; // enum with the variants Less, Greater, and Equal. These are the three outcomes that are possible when you compare two values.

const RANGE_UPPER_LIMIT: i32 = 100;

pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    /*
        This public method is necessary because the value field of the Guess struct is private.
        It’s important that the value field be private so code using the Guess struct is not allowed
        to set value directly: code outside the module must use the Guess::new function to create an
        instance of Guess, thereby ensuring there’s no way for a Guess to have a value that hasn’t
        been checked by the conditions in the Guess::new function.
     */
    pub fn value(&self) -> i32 {
        self.value
    }
}

pub fn start_game() -> anyhow::Result<()> {
    println!("Guess the number!");

    // Q# style syntax for range where = indicates inclusive
    // the rand::thread_rng function that gives us the particular random number generator:
    // one that is local to the current thread of execution and is seeded by the operating system
    let secret_number: i32 = rand::thread_rng().gen_range(1..=RANGE_UPPER_LIMIT);

    loop {
        println!("Please input your guess.");

        // string type provided by the standard library that is a growable, UTF-8 encoded bit of text
        let mut guess = String::new();

        // The stdin function returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for the terminal
        // Result is an enum, which is a type that can be in one of multiple possible states. We call each possible state a variant (Ok and Err).
        // Returns the number of bytes in the user’s input
        io::stdin().read_line(&mut guess)?;

        // Shadowing and trimming the newline character \n that gets appended when the user hits enter after typing their input
        // Without anyhow, the trait `From<ParseIntError>` is not implemented for `std::io::Error`
        let guess = Guess::new(guess.trim().parse::<i32>()?);

        match guess.value.cmp(&secret_number) {
            Ordering::Less => println!("Too small. Try a bigger number!"),
            Ordering::Greater => println!("Too big. Try a small number!"),
            Ordering::Equal => {
                println!("You win!");
                break
            }
        }
    }

    Ok(())
}