/*
    Macros are expanded before the compiler interprets the meaning of the code, so a macro can,
    for example, implement a trait on a given type. A function can’t, because it gets called at
    runtime and a trait needs to be implemented at compile time.
    Rust doesn’t have reflection capabilities, so it can’t look up the type’s name at runtime.
    We need a macro to generate code at compile time.

    Procedural macros need to be in their own crate. Eventually, this restriction might be lifted.
    The convention for structuring crates and macro crates is as follows: for a crate named foo, a
    custom derive procedural macro crate is called foo_derive.
 */

pub trait HelloMacro {
    fn hello_macro();
}