/*
    To run the binary crate from the add directory, we can specify which package in the workspace we
    want to run by using the -p argument and the package name with cargo run.
    cargo run -p adder
 */

/*
    Notice that the workspace has only one Cargo.lock file at the top level, rather than having a
    Cargo.lock in each crate’s directory. This ensures that all crates are using the same version
    of all dependencies. If we add the rand package to the adder/Cargo.toml and add_one/Cargo.toml
    files, Cargo will resolve both of those to one version of rand and record that in the one
    Cargo.lock. Making all crates in the workspace use the same dependencies means the crates
    will always be compatible with each other.
 */
use add_one;

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
}
