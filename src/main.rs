use logformat::{parser::*, visitor::{JsonPrinterVisitor, AstNode}};
use serde_json::{Value};
use std::io;
use std::io::Write;

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

            println!("--------------------------------------");
            match parser.parse_log_message() {
                Ok(log_message) => {
                    let modified_message = log_message.message.trim().to_string();
                    value[target_field_name] = Value::String(modified_message);
                    let value_json = serde_json::to_string_pretty(&value).expect("Could not re-serialize json log message.");
                    writeln!(stdout, "{}", value_json).unwrap();
                    log_message.write_json(&mut stdout);
                },
                Err(err) => {
                    println!("Could not parse log message: {}", err);
                    let value_json = serde_json::to_string_pretty(&value).expect("Could not re-serialize json log message.");
                    write!(&mut stdout, "{}", value_json).unwrap();
                },
            }
        }
    }
}
