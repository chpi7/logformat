use serde::Serialize;
use std::io::Write;

#[derive(Debug, Serialize)]
pub struct LogMessage {
    pub message: String,
    pub log_entity: LogEntity
}

#[derive(Debug, Serialize)]
pub enum LogEntity {
    Object(String, AttributeList),
    Text(String),
    Number(f32),
    Null,
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

impl LogMessage {
    pub fn pretty_print(&self, indent: u32, out: &mut dyn Write) {
        // out.write(self.message.as_bytes()).unwrap();
        // write!(out, "\n").unwrap();
        self.log_entity.pretty_print(indent, 0, out);
    }
}

impl LogEntity {

    pub fn pretty_print(&self, indent: u32, indent_level: u32, out: &mut dyn Write) {
        let indent_str = " ".repeat((indent * indent_level) as usize);
        match self {
            LogEntity::Object(name, attributes) => {
                out.write(name.as_bytes()).unwrap();
                out.write("(\n".as_bytes()).unwrap();
                let attribute_list = &attributes.attributes;
                for (idx, attribute) in attribute_list.into_iter().enumerate() {
                    let is_last = idx == attribute_list.len() - 1;
                    attribute.pretty_print(indent, indent_level + 1, out, is_last);
                }
                out.write(indent_str.as_bytes()).unwrap();
                out.write(")".as_bytes()).unwrap();
            }
            LogEntity::Text(text) => {
                write!(out, "\"{}\"", text).unwrap();
            }
            LogEntity::Number(value) => {
                write!(out, "{}", value).unwrap();
            }
            LogEntity::Null => {}
        }
        if indent_level == 0 {
            writeln!(out, "").unwrap();
        }
    }

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
            }
            LogEntity::Text(text) => {
                write!(out, "\"{}\"", text).unwrap();
            }
            LogEntity::Number(value) => {
                write!(out, "{}", value).unwrap();
            }
            LogEntity::Null => {}
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

    pub fn pretty_print(&self, indent: u32, indent_level: u32, out: &mut dyn Write, is_last: bool) {
        let indent_str = " ".repeat((indent * indent_level) as usize);
        out.write(indent_str.as_bytes()).unwrap();
        write!(out, "{} = ", self.key).unwrap();
        self.value.pretty_print(indent, indent_level, out);
        if is_last {
            write!(out, "\n").unwrap();
        } else {
            write!(out, ",\n").unwrap();
        }
    }
}
