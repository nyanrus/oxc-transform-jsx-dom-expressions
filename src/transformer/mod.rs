#![allow(unused_imports)]

pub mod jsx;
pub mod components;
pub mod events;
pub mod optimization;

// Re-export main types that will be used in the future
pub use jsx::{JSXTransformer, TransformError};
pub use components::{ComponentTransformer, ComponentError};
pub use events::{EventTransformer, EventError};
pub use optimization::{OptimizationPass, OptimizationResult};

// Main transformer implementation
use crate::SolidTransformOptions;
use oxc_ast::ast::{Program, Statement, Expression, JSXElement, JSXElementName};
use oxc_allocator::Allocator;

pub struct SolidTransform<'a> {
    options: &'a SolidTransformOptions,
    template_counter: usize,
    allocator: &'a Allocator,
}

impl<'a> SolidTransform<'a> {
    pub fn new(options: &'a SolidTransformOptions, allocator: &'a Allocator) -> Self {
        Self {
            options,
            template_counter: 0,
            allocator,
        }
    }

    pub fn transform_program(&mut self, program: &mut Program) {
        // Main transformation entry point
        self.visit_program(program);
    }

    fn visit_program(&mut self, program: &mut Program) {
        let mut jsx_transformer = JSXTransformer::new(self.allocator);
        
        for stmt in &mut program.body {
            self.visit_statement(stmt, &mut jsx_transformer);
        }
        
        // Add template declarations at the top of the program
        self.add_template_declarations(program, &jsx_transformer);
    }

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

    fn visit_expression(&mut self, expr: &mut Expression, jsx_transformer: &mut JSXTransformer) {
        match expr {
            Expression::JSXElement(jsx_element) => {
                // Transform JSX element and extract template info
                match jsx_transformer.transform_jsx_element(jsx_element) {
                    Ok(template_call) => {
                        // Mark that transformation occurred
                        self.template_counter += 1;
                        // In a complete implementation, we would replace the expression
                        println!("Transformed JSX element to: {}", template_call);
                    }
                    Err(e) => {
                        eprintln!("Failed to transform JSX element: {:?}", e);
                    }
                }
            }
            Expression::JSXFragment(_) => {
                // Handle JSX fragments
                self.template_counter += 1;
                println!("Transformed JSX fragment");
            }
            _ => {
                // Handle other expression types
            }
        }
    }

    fn add_template_declarations(&self, _program: &mut Program, jsx_transformer: &JSXTransformer) {
        // Add template declarations like:
        // const _tmpl$1 = /*#__PURE__*/template(`<div>Hello World</div>`);
        
        let templates = jsx_transformer.get_templates();
        for (template_name, template_html) in templates {
            // In a complete implementation, we would create and insert
            // variable declarations for each template
            // For now, we just track that templates were generated
            eprintln!("Generated template: {} = `{}`", template_name, template_html);
        }
    }

    fn get_next_template_name(&mut self) -> String {
        self.template_counter += 1;
        format!("_tmpl${}", self.template_counter)
    }

    fn is_solid_component(&self, _name: &JSXElementName) -> bool {
        // TODO: Determine if a JSX element is a Solid.js component
        // This includes components like <Show>, <For>, <Switch>, etc.
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SolidTransformOptions;
    use oxc_allocator::Allocator;

    #[test]
    fn test_solid_transform_creation() {
        let options = SolidTransformOptions::default();
        let allocator = Allocator::default();
        let transform = SolidTransform::new(&options, &allocator);
        assert_eq!(transform.template_counter, 0);
    }

    #[test]
    fn test_template_name_generation() {
        let options = SolidTransformOptions::default();
        let allocator = Allocator::default();
        let mut transform = SolidTransform::new(&options, &allocator);
        
        assert_eq!(transform.get_next_template_name(), "_tmpl$1");
        assert_eq!(transform.get_next_template_name(), "_tmpl$2");
        assert_eq!(transform.get_next_template_name(), "_tmpl$3");
    }
}
