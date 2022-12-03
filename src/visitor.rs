use std::io::Write;

use crate::ast::*;

pub trait AstVisitor {
    fn visit_log_message(&mut self, node: &LogMessage);
    fn visit_attribute_list_entry(&mut self, node: &AttributeList);
    fn visit_attribute_list_between(&mut self, node: &AttributeList);
    fn visit_attribute_list_exit(&mut self, node: &AttributeList);
    fn visit_attribute(&mut self, node: &Attribute);
    fn visit_log_entity_object_entry(&mut self, node: &LogEntity);
    fn visit_log_entity_object_exit(&mut self, node: &LogEntity);
    fn visit_log_entity_text(&mut self, node: &LogEntity);
    fn visit_log_entity_number(&mut self, node: &LogEntity);
    fn visit_log_entity_null(&mut self, node: &LogEntity);
}

pub trait AstNode {
    fn accept(&self, visitor: &mut dyn AstVisitor);
}

pub struct JsonPrinterVisitor<'a> {
    out: &'a mut dyn Write,
    indent_size: usize,
    current_indent_level: usize,
}

impl JsonPrinterVisitor<'_> {
    pub fn create(out: &mut dyn Write, indent_size: usize) -> JsonPrinterVisitor {
        JsonPrinterVisitor {
            out,
            indent_size,
            current_indent_level: 0,
        }
    }

    fn write_indent(&mut self) {
        let indent_str = " ".repeat(self.indent_size * self.current_indent_level);
        self.out.write(indent_str.as_bytes()).unwrap();
    }

    fn write_newline(&mut self) {
        writeln!(self.out, "").unwrap();
    }
}

impl AstVisitor for JsonPrinterVisitor<'_> {
    fn visit_attribute(&mut self, node: &Attribute) {
        self.write_indent();
        write!(self.out, "\"{}\": ", node.key).unwrap();
    }

    fn visit_attribute_list_entry(&mut self, _: &AttributeList) {
        self.write_newline();
    }

    fn visit_attribute_list_between(&mut self, _: &AttributeList) {
        write!(self.out, ", ").unwrap();
        self.write_newline();
    }

    fn visit_attribute_list_exit(&mut self, _: &AttributeList) {
        self.write_newline();
    }

    fn visit_log_entity_object_entry(&mut self, _: &LogEntity) {
        write!(self.out, "{{").unwrap();
        self.current_indent_level += 1;
    }

    fn visit_log_entity_object_exit(&mut self, _: &LogEntity) {
        self.current_indent_level -= 1;
        self.write_indent();
        write!(self.out, "}}").unwrap();
    }

    fn visit_log_entity_text(&mut self, node: &LogEntity) {
        if let LogEntity::Text(text) = node {
            write!(self.out, "\"{}\"", text).unwrap();
        }
    }

    fn visit_log_entity_number(&mut self, node: &LogEntity) {
        if let LogEntity::Number(num) = node {
            write!(self.out, "{}", num).unwrap();
        }
    }

    fn visit_log_entity_null(&mut self, _: &LogEntity) {
        write!(self.out, "null").unwrap();
    }

    fn visit_log_message(&mut self, _: &LogMessage) {
        self.write_newline();
    }
}
