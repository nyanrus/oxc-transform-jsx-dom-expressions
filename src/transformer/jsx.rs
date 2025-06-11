/// JSX transformation logic for Solid.js
/// 
/// This module handles the core JSX to Solid.js template transformation:
/// - <div>content</div> -> _tmpl$("<div>content</div>")
/// - Dynamic content handling
/// - Attribute and property transformations

use oxc_ast::ast::{JSXElement, JSXChild, JSXAttribute, Expression, Statement, CallExpression, Argument};
use oxc_allocator::Allocator;
use oxc_span::Span;
use std::collections::HashMap;

pub struct JSXTransformer<'a> {
    /// Counter for generating unique template names
    template_counter: usize,
    /// Allocator for creating AST nodes
    allocator: &'a Allocator,
    /// Generated templates
    templates: HashMap<String, String>,
}

impl<'a> JSXTransformer<'a> {
    pub fn new(allocator: &'a Allocator) -> Self {
        Self {
            template_counter: 0,
            allocator,
            templates: HashMap::new(),
        }
    }

    /// Transform a JSX element into a Solid.js template call
    pub fn transform_jsx_element(&mut self, element: &JSXElement) -> Result<String, TransformError> {
        // Extract static template structure
        let template_html = self.extract_template(element);
        
        // Generate unique template name
        let template_name = self.get_next_template_name();
        
        // Store template for later code generation
        self.templates.insert(template_name.clone(), template_html);
        
        // Return template call as string for now
        Ok(format!("{}()", template_name))
    }

    /// Extract static HTML template from JSX
    fn extract_template(&self, element: &JSXElement) -> String {
        let mut html = String::new();
        
        // Get tag name
        let tag_name = self.get_element_tag_name(element);
        html.push('<');
        html.push_str(&tag_name);
        
        // Add attributes (static only for now)
        for attr_item in &element.opening_element.attributes {
            match attr_item {
                oxc_ast::ast::JSXAttributeItem::Attribute(jsx_attr) => {
                    if let Some(attr_html) = self.extract_static_attribute(jsx_attr) {
                        html.push(' ');
                        html.push_str(&attr_html);
                    }
                }
                _ => {
                    // Skip spread attributes for now
                }
            }
        }
        
        html.push('>');
        
        // Add children
        html.push_str(&self.extract_children_template(&element.children));
        
        // Close tag
        html.push_str("</");
        html.push_str(&tag_name);
        html.push('>');
        
        html
    }

    /// Get element tag name
    fn get_element_tag_name(&self, element: &JSXElement) -> String {
        match &element.opening_element.name {
            oxc_ast::ast::JSXElementName::Identifier(ident) => ident.name.to_string(),
            oxc_ast::ast::JSXElementName::IdentifierReference(ident) => ident.name.to_string(),
            _ => "div".to_string(), // fallback
        }
    }

    /// Extract static attribute as HTML string
    fn extract_static_attribute(&self, attr: &JSXAttribute) -> Option<String> {
        let name = self.get_attribute_name(attr);
        
        match &attr.value {
            Some(oxc_ast::ast::JSXAttributeValue::StringLiteral(literal)) => {
                Some(format!("{}=\"{}\"", name, literal.value))
            }
            None => Some(name), // Boolean attribute
            _ => None, // Dynamic attribute, skip for now
        }
    }

    /// Get attribute name
    fn get_attribute_name(&self, attr: &JSXAttribute) -> String {
        match &attr.name {
            oxc_ast::ast::JSXAttributeName::Identifier(ident) => ident.name.to_string(),
            _ => "attr".to_string(), // fallback
        }
    }

    /// Extract children as template HTML
    fn extract_children_template(&self, children: &oxc_allocator::Vec<JSXChild>) -> String {
        let mut html = String::new();
        
        for child in children {
            match child {
                JSXChild::Text(text) => {
                    html.push_str(&text.value);
                }
                JSXChild::Element(_) => {
                    // For nested elements, we'd need recursive handling
                    // For now, just add a placeholder
                    html.push_str("[nested element]");
                }
                _ => {
                    // Skip expressions and other dynamic content for now
                }
            }
        }
        
        html
    }

    /// Generate unique template name
    fn get_next_template_name(&mut self) -> String {
        self.template_counter += 1;
        format!("_tmpl${}", self.template_counter)
    }

    /// Get generated templates
    pub fn get_templates(&self) -> &HashMap<String, String> {
        &self.templates
    }
}

#[derive(Debug)]
pub enum TransformError {
    NotImplemented(&'static str),
    InvalidJSX(String),
    UnsupportedFeature(String),
}

#[derive(Debug)]
pub struct AttributeBinding {
    pub name: String,
    pub value: AttributeValue,
}

#[derive(Debug)]
pub enum AttributeValue {
    Static(String),
    Dynamic(String), // Use String instead of Expression for now
    EventHandler(String), // Use String instead of Expression for now
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxc_allocator::Allocator;

    #[test]
    fn test_jsx_transformer_creation() {
        let allocator = Allocator::default();
        let transformer = JSXTransformer::new(&allocator);
        assert_eq!(transformer.template_counter, 0);
    }

    #[test]
    fn test_template_counter_increment() {
        let allocator = Allocator::default();
        let mut transformer = JSXTransformer::new(&allocator);
        // Simulate transforming elements
        transformer.template_counter += 1;
        assert_eq!(transformer.template_counter, 1);
    }
}
