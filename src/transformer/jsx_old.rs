use oxc_allocator::Allocator;
/// JSX transformation logic for dom-expressions
///
/// This module handles the core JSX to dom-expressions template transformation:
/// - <div>content</div> -> _tmpl$("<div>content</div>")
/// - Dynamic content handling
/// - Attribute and property transformations
use oxc_ast::ast::{
    Argument, CallExpression, Expression, JSXAttribute, JSXChild, JSXElement, Statement,
};
use oxc_ast::AstBuilder;
use oxc_ast::NONE;
use oxc_span::Span;
use std::collections::HashMap;

// Add new imports for AST node creation
use oxc_ast::ast::Atom;
use oxc_ast::ast::{BindingIdentifier, IdentifierReference};
use std::cell::Cell;

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

    /// Get stored templates for declaration generation
    pub fn get_templates(&self) -> &HashMap<String, String> {
        &self.templates
    }

    /// Generate a template declaration as a string
    pub fn create_template_declaration(&self, template_name: &str, template_html: &str) -> String {
        format!("const {} = /*#__PURE__*/template(`{}`);", template_name, template_html)
    }

    /// Extract template HTML from a JSX element
    fn extract_template(&self, element: &JSXElement) -> String {
        // For now, handle simple cases
        match &element.opening_element.name {
            JSXElementName::Identifier(ident) => {
                let tag_name = &ident.name;
                
                // Handle children - for simple static content
                let content = if element.children.is_empty() {
                    String::new()
                } else {
                    // For now, just extract text content
                    element.children.iter()
                        .filter_map(|child| {
                            match child {
                                JSXChild::Text(text) => Some(text.value.to_string()),
                                _ => None, // TODO: Handle other child types
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("")
                };

                if element.opening_element.self_closing {
                    format!("<{} />", tag_name)
                } else {
                    format!("<{}>{}</{}>", tag_name, content, tag_name)
                }
            }
            _ => "<!-- unsupported JSX -->".to_string(), // TODO: Handle other element types
        }
    }

    /// Generate next template name
    fn get_next_template_name(&mut self) -> String {
        self.template_counter += 1;
        format!("_tmpl${}", self.template_counter)
    }

    /// Transform a JSX element into a dom-expressions template call
    pub fn transform_jsx_element(
        &mut self,
        element: &JSXElement,
    ) -> Result<String, TransformError> {
        // Extract static template structure
        let template_html = self.extract_template(element);

        // Generate unique template name
        let template_name = self.get_next_template_name();

        // Store template for later code generation
        self.templates.insert(template_name.clone(), template_html);

        // Return template call as string for now
        Ok(format!("{}()", template_name))
    }

    /// Create AST node for template declaration
    /// const _tmpl$1 = /*#__PURE__*/template(`<div>Hello World</div>`);
    pub fn create_template_declaration(&self, template_name: &str, template_html: &str) -> String {
        format!(
            "const {} = /*#__PURE__*/template(`{}`);",
            template_name, template_html
        )
    }    /// Create a CallExpression AST node for template call: _tmpl$1()
    /// Temporarily disabled due to complex OXC API issues - will be implemented in next phase
    pub fn create_template_call(&self, _template_name: &str) -> Expression<'a> {
        // TODO: Implement proper AST node creation
        // For now, this is a placeholder that should be replaced with proper AST building
        // The actual transformation is happening in visit_expression in mod.rs
        // where JSXElement is replaced with a CallExpression

        // Return a placeholder - this will be replaced with proper implementation
        todo!("CallExpression creation needs proper AST builder integration")
    }

    /// Transform JSX element to template call expression (replaces the JSX in AST)
    /// Currently generates templates but AST replacement is pending proper OXC integration
    pub fn transform_jsx_to_call(
        &mut self,
        element: &JSXElement,
    ) -> Result<String, TransformError> {
        // Extract static template structure
        let template_html = self.extract_template(element);

        // Generate unique template name
        let template_name = self.get_next_template_name();

        // Store template for later code generation
        self.templates.insert(template_name.clone(), template_html);

        // For now, return template call as string representation
        // Later this will return Expression<'a> when AST building is properly implemented
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
            _ => None,          // Dynamic attribute, skip for now
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

    /// Get current template name (the last one generated)
    pub fn get_current_template_name(&self) -> String {
        format!("_tmpl${}", self.template_counter)
    }

    /// Get generated templates
    pub fn get_templates(&self) -> &HashMap<String, String> {
        &self.templates
    }

    /// Replace JSXElement with CallExpression in the AST
    /// This method is temporarily disabled due to AST complexity
    pub fn replace_jsx_element_with_call(&self, _element: &JSXElement, _callee: &str) {
        // TODO: Implement proper AST replacement when OXC API issues are resolved
        todo!("AST replacement needs proper implementation")
    }

    /// Convert JSXAttribute to Argument for CallExpression
    /// This method is temporarily disabled due to AST complexity
    fn attribute_to_argument(&self, _attr: &JSXAttribute) {
        // TODO: Implement proper argument conversion when OXC API issues are resolved
        todo!("Argument creation needs proper implementation")
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
    Dynamic(String),      // Use String instead of Expression for now
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
