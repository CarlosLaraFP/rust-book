#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShirtColor {
    Red,
    Blue,
}

pub struct Inventory {
    pub shirts: Vec<ShirtColor>,
}

impl Inventory {
    /*
        The unwrap_or_else method on Option<T> is defined by the standard library.
        It takes one argument: a closure without any arguments that returns a value T
        (the same type stored in the Some variant of the Option<T>, in this case ShirtColor).
        If the Option<T> is the Some variant, unwrap_or_else returns the value from within the Some.
        If the Option<T> is the None variant, unwrap_or_else calls the closure and returns the value
        returned by the closure. If the closure had parameters, they would appear between the two
        vertical bars. The closure captures an immutable reference to the self Inventory instance
        and passes it with the code we specify to the unwrap_or_else method.
        Functions, on the other hand, are not able to capture their environment in this way.
     */
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

