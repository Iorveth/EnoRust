extern crate eno_rust;
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
    eno_rust::parser::parse(input, locale, zero_indexing);
}
