// The proc_macro crate comes with Rust and is the compiler’s API that allows us to
// read and manipulate Rust code from our code.
use proc_macro::TokenStream;
// The syn crate parses Rust code from a string into a data structure that we can perform operations on
use syn;
// The quote crate turns syn data structures back into Rust code. These crates make it much simpler to
// parse any sort of Rust code we might want to handle: writing a full parser for Rust code is no simple task.
use quote::quote;


/*
    It’s necessary for our procedural macro to panic on errors because proc_macro_derive functions
    must return TokenStream rather than Result to conform to the procedural macro API. We’ve
    simplified this example by using unwrap; in production code, you should provide more specific
    error messages about what went wrong by using panic! or expect.
 */
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree that we can manipulate
    let ast = syn::parse(input).unwrap();
    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
