mod lexer;
mod ast;
mod parser;

use crate::parser::*;
use std::io::stdout;

fn main() {
    println!("Hello, world!");

    let test_input = "  MyClass (name = test123, other = 123, reference = MyOtherClass(id = 123, age = 23, name = heyho), reference = MyOtherClass(id = 123, age = 23, name = heyho))";
    let mut parser = Parser::create(test_input);
    let result = parser.parse_log_entity().expect("Could not parse input!");
    result.to_json(2, 0, &mut stdout());
    // println!("{:?}", result);
}
