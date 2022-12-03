use std::io::Write;

use serde::Serialize;

use crate::visitor::*;

#[derive(Debug, Serialize)]
pub struct LogMessage {
    pub message: String,
    pub log_entity: Option<LogEntity>,
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
    pub fn write_json(&self, out: &mut dyn Write) {
        if let Some(log_entity) = &self.log_entity {
            write!(out, "Log Entity = ").unwrap();
            let mut json_printer = JsonPrinterVisitor::create(out, 2);
            log_entity.accept(&mut json_printer);
            writeln!(out, "").unwrap();
        }
    }
}

impl AstNode for LogEntity {
    fn accept(&self, visitor: &mut dyn AstVisitor) {
        match self {
            LogEntity::Object(_, attributes) => {
                visitor.visit_log_entity_object_entry(self);
                attributes.accept(visitor);
                visitor.visit_log_entity_object_exit(self);
            }
            LogEntity::Text(_) => visitor.visit_log_entity_text(self),
            LogEntity::Number(_) => visitor.visit_log_entity_number(self),
            LogEntity::Null => visitor.visit_log_entity_null(self),
        }
    }
}

impl AstNode for AttributeList {
    fn accept(&self, visitor: &mut dyn AstVisitor) {
        visitor.visit_attribute_list_entry(self);
        let mut is_first = true;
        for attribute in &self.attributes {
            if !is_first {
                visitor.visit_attribute_list_between(self);
            }
            attribute.accept(visitor);
            is_first = false;
        }
        visitor.visit_attribute_list_exit(self);
    }
}

impl AstNode for Attribute {
    fn accept(&self, visitor: &mut dyn AstVisitor) {
        visitor.visit_attribute(self);
        self.value.accept(visitor);
    }
}
