use oxc_ast::ast::{Program, Statement, Expression, JSXElement, JSXElementName, Argument};
use oxc_allocator::Allocator;

pub mod jsx;
pub mod components;
pub mod events;
pub mod optimization;

// Re-export main types
pub use jsx::{JSXTransformer, TransformError};
pub use components::{ComponentTransformer, ComponentError};
pub use events::{EventTransformer, EventError};
pub use optimization::{OptimizationPass, OptimizationResult};

#[derive(Debug, Clone)]
pub enum ModuleFormat {
    Esm,
    Cjs,
}

impl Default for ModuleFormat {
    fn default() -> Self {
        ModuleFormat::Esm
    }
}

#[derive(Debug, Clone)]
pub struct DomExpressionsTransformOptions {
    pub generate_ssr: bool,
    pub hydratable: bool,
    pub delegation: bool,
    pub context_to_custom_elements: bool,
    pub static_marker: String,
    pub memo_wrapper: bool,
    pub wrap_conditionals: bool,
}

impl Default for DomExpressionsTransformOptions {
    fn default() -> Self {
        Self {
            generate_ssr: false,
            hydratable: false,
            delegation: true,
            context_to_custom_elements: false,
            static_marker: "$$".to_string(),
            memo_wrapper: true,
            wrap_conditionals: true,
        }
    }
}

/// Main transformer for dom-expressions JSX
pub struct DomExpressionsTransform<'a> {
    options: &'a DomExpressionsTransformOptions,
    template_counter: usize,
    allocator: &'a Allocator,
}

impl<'a> DomExpressionsTransform<'a> {
    pub fn new(options: &'a DomExpressionsTransformOptions, allocator: &'a Allocator) -> Self {
        Self {
            options,
            template_counter: 0,
            allocator,
        }
    }

    /// Main transformation entry point
    pub fn transform_program(&mut self, program: &mut Program) {
        let mut jsx_transformer = JSXTransformer::new(self.allocator);
        
        // Collect all JSX elements and generate templates
        self.collect_jsx_templates(program, &mut jsx_transformer);
        
        // Add template declarations at the beginning of the program
        self.add_template_declarations(program, &jsx_transformer);
        
        println!("Transformation completed with {} templates", jsx_transformer.get_templates().len());
    }

    /// Recursively find and process JSX elements to generate templates
    fn collect_jsx_templates(&mut self, program: &mut Program, jsx_transformer: &mut JSXTransformer) {
        for stmt in &mut program.body {
            self.visit_statement(stmt, jsx_transformer);
        }
    }

    /// Process different statement types
    fn visit_statement(&mut self, stmt: &mut Statement, jsx_transformer: &mut JSXTransformer) {
        match stmt {
            Statement::FunctionDeclaration(func) => {
                if let Some(body) = &mut func.body {
                    for stmt in &mut body.statements {
                        self.visit_statement(stmt, jsx_transformer);
                    }
                }
            }
            Statement::ReturnStatement(ret) => {
                if let Some(expr) = &mut ret.argument {
                    self.visit_expression(expr, jsx_transformer);
                }
            }
            _ => {
                // Handle other statement types as needed
            }
        }
    }

    /// Process expressions to find JSX elements
    fn visit_expression(&mut self, expr: &mut Expression, jsx_transformer: &mut JSXTransformer) {
        match expr {
            Expression::JSXElement(jsx_element) => {
                // Transform JSX element and generate template
                if let Ok(template_call) = jsx_transformer.transform_jsx_element(jsx_element) {
                    println!("Generated template call for JSX: {}", template_call);
                    // TODO: Replace JSX with actual CallExpression in AST
                    // For now, just generate the template
                }
            }
            _ => {
                // Handle other expression types as needed
            }
        }
    }

    /// Add template declarations to the beginning of the program
    fn add_template_declarations(&self, program: &mut Program, jsx_transformer: &JSXTransformer) {
        // For now, we'll print the template declarations
        // TODO: Add actual AST nodes to the program
        
        for (template_name, template_html) in jsx_transformer.get_templates() {
            let declaration = jsx_transformer.create_template_declaration(template_name, template_html);
            println!("Template declaration: {}", declaration);
        }
        
        println!("Added {} template declarations", jsx_transformer.get_templates().len());
    }

    /// Generate unique template name
    fn get_next_template_name(&mut self) -> String {
        self.template_counter += 1;
        format!("_tmpl${}", self.template_counter)
    }

    /// Check if JSX element name represents a dom-expressions component
    fn is_dom_expressions_component(&self, _name: &JSXElementName) -> bool {
        // TODO: Implement dom-expressions component detection
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxc_allocator::Allocator;

    #[test]
    fn test_dom_expressions_transform_creation() {
        let allocator = Allocator::default();
        let options = DomExpressionsTransformOptions::default();
        let transform = DomExpressionsTransform::new(&options, &allocator);
        assert_eq!(transform.template_counter, 0);
    }

    #[test]
    fn test_template_name_generation() {
        let allocator = Allocator::default();
        let options = DomExpressionsTransformOptions::default();
        let mut transform = DomExpressionsTransform::new(&options, &allocator);
        
        let name1 = transform.get_next_template_name();
        assert_eq!(name1, "_tmpl$1");
        
        let name2 = transform.get_next_template_name();
        assert_eq!(name2, "_tmpl$2");
    }
}
