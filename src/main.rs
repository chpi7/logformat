mod lexer;
mod ast;
mod parser;

use crate::parser::*;
use std::io::{stdout, stdin, Result, BufRead};
use std::io;
use serde_json::{StreamDeserializer, Value};

fn main() {

    let target_field_name = "value";

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let stream = serde_json::Deserializer::from_reader(stdin.lock());
    let values = stream.into_iter::<Value>();

    for value in values {
        if let Ok(mut value) = value {
            let pretty_print_value = &value[target_field_name];
            let mut parser = Parser::create(pretty_print_value.as_str().unwrap());
            let log_entity = parser.parse_log_entity().unwrap();

            value[target_field_name] = Value::Null;

            println!("Deserialized object {}", value);
            log_entity.pretty_print(2, 0, &mut stdout)
        }
    }

    // println!("Hello, world!");

    // let test_input = "  MyClass (name = test123, other = 123, reference = MyOtherClass(id = 123, age = 23, name = Name(value = test)), reference = MyOtherClass(id = 123, age = 23, name = heyho))";
    // let mut parser = Parser::create(test_input);
    // let result = parser.parse_log_entity().expect("Could not parse input!");
    // result.pretty_print(2, 0, &mut stdout());
    // println!("{:?}", result);
}
