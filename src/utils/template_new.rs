/// Template generation utilities for Solid.js
/// 
/// This module handles the generation of template strings and
/// template function calls for Solid.js JSX transformation

use oxc_ast::ast::{JSXElement, JSXChild};

/// Template generator for converting JSX to Solid.js templates
pub struct TemplateGenerator {
    /// Counter for generating unique template names
    template_counter: usize,
}

impl TemplateGenerator {
    pub fn new() -> Self {
        Self {
            template_counter: 0,
        }
    }

    /// Generate a template string from JSX element
    pub fn generate_template_string(&self, _element: &JSXElement) -> String {
        // TODO: Implement template string generation
        // This will convert JSX elements to HTML template strings
        // Example: <div class="test">content</div> -> "<div class=\"test\">content</div>"
        String::new()
    }

    /// Generate template function call
    pub fn generate_template_call(&mut self, _element: &JSXElement) -> String {
        // TODO: Implement template function call generation
        // This will create calls like: _tmpl$1()
        self.template_counter += 1;
        format!("_tmpl${}()", self.template_counter)
    }

    /// Generate unique template variable name
    pub fn get_next_template_name(&mut self) -> String {
        self.template_counter += 1;
        format!("_tmpl${}", self.template_counter)
    }

    /// Convert JSX children to template content
    pub fn children_to_template(&self, _children: &[JSXChild]) -> String {
        // TODO: Implement children to template conversion
        String::new()
    }

    /// Escape template string for JavaScript
    pub fn escape_template_string(input: &str) -> String {
        input
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t")
    }
}

impl Default for TemplateGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_generator() {
        let mut generator = TemplateGenerator::new();
        assert_eq!(generator.get_next_template_name(), "_tmpl$1");
        assert_eq!(generator.get_next_template_name(), "_tmpl$2");
    }

    #[test]
    fn test_escape_template_string() {
        assert_eq!(
            TemplateGenerator::escape_template_string("hello \"world\""),
            "hello \\\"world\\\""
        );
        assert_eq!(
            TemplateGenerator::escape_template_string("line1\nline2"),
            "line1\\nline2"
        );
    }
}
