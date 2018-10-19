extern crate eno_rust;
#[macro_use]
extern crate runtime_fmt;

use eno_rust::parser::parse;

fn main() {
    let input = "Greeting: Hello World!";
    let locale = "en";
    let zero_indexing = false;
    println!("{:?}", parse(input, locale, zero_indexing));
}
