use oxc_allocator::Allocator;
use oxc_ast::ast::{Expression, JSXElement, JSXChild, JSXElementName, JSXAttribute};
use std::collections::HashMap;

/// JSX transformation logic for dom-expressions
///
/// This module handles the core JSX to dom-expressions template transformation:
/// - <div>content</div> -> _tmpl$1() with template declaration
/// - Dynamic content handling
/// - Attribute and property transformations
pub struct JSXTransformer<'a> {
    /// Counter for generating unique template names
    template_counter: usize,
    /// Allocator for creating AST nodes
    allocator: &'a Allocator,
    /// Generated templates: template_name -> template_html
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

    /// Get current template name (the last one generated)
    pub fn get_current_template_name(&self) -> String {
        if self.template_counter == 0 {
            "_tmpl$1".to_string()
        } else {
            format!("_tmpl${}", self.template_counter)
        }
    }
}

#[derive(Debug)]
pub enum TransformError {
    NotImplemented(&'static str),
    InvalidJSX(String),
    UnsupportedFeature(String),
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
        
        let template_name1 = transformer.get_next_template_name();
        assert_eq!(template_name1, "_tmpl$1");
        
        let template_name2 = transformer.get_next_template_name();
        assert_eq!(template_name2, "_tmpl$2");
    }
}
