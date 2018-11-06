
extern crate eno_rust;

use eno_rust::parser::parse;

fn main() {
    let input = "author: Jane Doe
email: jane@eno-lang.org

-- my_content
Multi-line embedded content (e.g. markdown) here ...
-- my_content

states:
active = #fff
hover = #b6b6b6

# cities
Den Haag: 52.069961, 4.302315
Málaga: 36.721447, -4.421291
서울특별시: 37.566984, 126.977041

# cities expanded < cities
Springfield: 38.790312, -77.186418";
    let zero_indexing = false;
    println!("{:?}", parse(input,  zero_indexing));
}
