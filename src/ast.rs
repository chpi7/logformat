use serde::{Serialize};
use std::io::{Write};

#[derive(Debug, Serialize)]
pub enum LogEntity {
    Object(String, AttributeList),
    Text(String),
    Number(f32),
}

#[derive(Debug, Serialize)]
pub struct AttributeList {
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Serialize)]
pub struct Attribute {
    pub key: String,
    pub value: LogEntity,
}

impl LogEntity {
    pub fn to_json(&self, indent: u32, indent_level: u32, out: &mut dyn Write) {
        let indent_str = " ".repeat((indent * indent_level) as usize);
        match self {
            LogEntity::Object(_, attributes) => {
                out.write("{\n".as_bytes()).unwrap();
                let attribute_list = &attributes.attributes;
                for (idx, attribute) in attribute_list.into_iter().enumerate() {
                    let is_last = idx == attribute_list.len() - 1;
                    attribute.to_json(indent, indent_level + 1, out, is_last);
                }
                out.write(indent_str.as_bytes()).unwrap();
                out.write("}".as_bytes()).unwrap();
            },
            LogEntity::Text(text) => {
                write!(out, "\"{}\"", text).unwrap();
            },
            LogEntity::Number(value) => {
                write!(out, "{}", value).unwrap();
            },
        }
    }
}

impl Attribute {
    pub fn to_json(&self, indent: u32, indent_level: u32, out: &mut dyn Write, is_last: bool) {
        let indent_str = " ".repeat((indent * indent_level) as usize);
        out.write(indent_str.as_bytes()).unwrap();
        write!(out, "{} = ", self.key).unwrap();
        self.value.to_json(indent, indent_level, out);
        if is_last {
            write!(out, "\n").unwrap();
        } else {
            write!(out, ",\n").unwrap();
        }
    }
}