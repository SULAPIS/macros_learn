use proc_macro::TokenStream;

use crate::json_schema::{get_string_literal, StructsTemplate};
mod json_schema;

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    println!("{:#?}", input);
    "fn hello() { println!(\"hello\"); }".parse().unwrap()
}

#[proc_macro]
pub fn generate(input: TokenStream) -> TokenStream {
    let filename = get_string_literal(input).unwrap();
    let res = StructsTemplate::render(&filename).unwrap();
    res.parse().unwrap()
}
