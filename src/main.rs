use logformat::{parser::*, visitor::{JsonPrinterVisitor, AstNode}};
use serde_json::{Value};
use std::io;

fn main() {
    let target_field_name = "message";

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let stream = serde_json::Deserializer::from_reader(stdin.lock());
    let values = stream.into_iter::<Value>();

    

    for value in values {
        if let Ok(mut value) = value {
            let pretty_print_value = &value[target_field_name];

            let mut parser = Parser::create(pretty_print_value.as_str().unwrap());
            let log_message = parser.parse_log_message().unwrap();
            // println!("{:?}", log_message);

            value[target_field_name] = Value::String(log_message.message.clone());

            print!(">>> ");
            log_message.print_json(&mut stdout);
        }
    }
}
