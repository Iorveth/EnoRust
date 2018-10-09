extern crate EnoRust;
#[macro_use]
extern crate runtime_fmt;
//use std::fs::File;
//use std::io::prelude::*;
//extern crate regex;
//use parser::parse;
fn main() {
    let input = "Greeting: Hello World!";
    let locale = "en";
    let zero_indexing = false;
    EnoRust::parser::parse(input, locale, zero_indexing);
}
