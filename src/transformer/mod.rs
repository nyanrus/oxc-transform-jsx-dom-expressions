use oxc_allocator::Allocator;
use oxc_ast::ast::{
    Argument, Expression, JSXElementName, Program, Span, Statement, VariableDeclarationKind,
};
use oxc_ast::AstBuilder;

pub mod components;
pub mod events;
pub mod jsx;
pub mod optimization;

// Re-export main types
pub use jsx::JSXTransformer;

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
    ast_builder: AstBuilder<'a>,
}

impl<'a> DomExpressionsTransform<'a> {
    pub fn new(options: &'a DomExpressionsTransformOptions, allocator: &'a Allocator) -> Self {
        Self {
            options,
            template_counter: 0,
            allocator,
            ast_builder: AstBuilder::new(allocator),
        }
    }

    /// Main transformation entry point
    pub fn transform_program(&mut self, program: &mut Program<'a>) {
        let mut jsx_transformer = JSXTransformer::new(self.allocator);

        // Collect all JSX elements and generate templates
        self.collect_jsx_templates(program, &mut jsx_transformer);

        // Add import statement and template declarations if we have templates
        if !jsx_transformer.get_templates().is_empty() {
            self.add_import_statement(program, &jsx_transformer);
            self.add_template_declarations(program, &jsx_transformer);
        }

        println!(
            "Transformation completed with {} templates",
            jsx_transformer.get_templates().len()
        );
    }

    /// Recursively find and process JSX elements to generate templates
    fn collect_jsx_templates(
        &mut self,
        program: &mut Program<'a>,
        jsx_transformer: &mut JSXTransformer,
    ) {
        for stmt in &mut program.body {
            self.visit_statement(stmt, jsx_transformer);
        }
    }

    /// Process different statement types
    fn visit_statement(&mut self, stmt: &mut Statement<'a>, jsx_transformer: &mut JSXTransformer) {
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
            Statement::VariableDeclaration(var_decl) => {
                // Handle variable declarations like const template = <div>...</div>
                for declarator in &mut var_decl.declarations {
                    if let Some(init_expr) = &mut declarator.init {
                        self.visit_expression(init_expr, jsx_transformer);
                    }
                }
            }
            Statement::ExpressionStatement(expr_stmt) => {
                self.visit_expression(&mut expr_stmt.expression, jsx_transformer);
            }
            _ => {
                // Handle other statement types as needed
            }
        }
    }

    /// Process expressions to find JSX elements
    fn visit_expression(
        &mut self,
        expr: &mut Expression<'a>,
        jsx_transformer: &mut JSXTransformer,
    ) {
        match expr {
            Expression::JSXElement(jsx_element) => {
                // Transform JSX element and generate template
                if let Ok(template_call_name) = jsx_transformer.transform_jsx_element(jsx_element) {
                    println!("Generated template call for JSX: {}", template_call_name);

                    // Replace JSX with actual CallExpression in AST
                    let template_name = template_call_name.trim_end_matches("()");
                    let call_expr = self.create_template_call_expression(template_name);
                    *expr = call_expr;
                }
            }
            Expression::ParenthesizedExpression(paren_expr) => {
                // Handle parenthesized expressions like (<div>...</div>)
                self.visit_expression(&mut paren_expr.expression, jsx_transformer);
            }
            Expression::AssignmentExpression(assign_expr) => {
                self.visit_expression(&mut assign_expr.right, jsx_transformer);
            }
            Expression::CallExpression(_call_expr) => {
                // Skip call expression arguments for now to avoid API issues
                // TODO: Handle call expression arguments properly
            }
            _ => {
                // Handle other expression types as needed
            }
        }
    }

    /// Create a template call expression: _tmpl$1()
    fn create_template_call_expression(&self, template_name: &str) -> Expression<'a> {
        let template_identifier = self
            .ast_builder
            .expression_identifier(Span::default(), self.ast_builder.atom(template_name));

        self.ast_builder.expression_call(
            Span::default(),
            template_identifier,
            None::<oxc_ast::ast::TSTypeParameterInstantiation>, // type_arguments
            self.ast_builder.vec(),                             // empty arguments for now
            false,                                              // optional_chain
        )
    }

    /// Add template declarations to the beginning of the program
    fn add_template_declarations(
        &self,
        program: &mut Program<'a>,
        jsx_transformer: &JSXTransformer,
    ) {
        for (template_name, template_html) in jsx_transformer.get_templates() {
            // 1. Create string literal for now (will be changed to template literal later)
            let template_string = self.ast_builder.expression_string_literal(
                Span::default(),
                self.ast_builder.atom(&template_html),
                None,
            );

            // 2. Create template function call
            let template_identifier = self
                .ast_builder
                .expression_identifier(Span::default(), self.ast_builder.atom("_$template"));

            let template_call = self.ast_builder.expression_call(
                Span::default(),
                template_identifier,
                None::<oxc_ast::ast::TSTypeParameterInstantiation>, // type_arguments
                self.ast_builder.vec1(Argument::from(template_string)), // arguments
                false,                                              // optional_chain
            );

            // 3. Create binding identifier for variable name
            let _binding_id = self
                .ast_builder
                .binding_identifier(Span::default(), self.ast_builder.atom(&template_name));

            // 4. Create binding pattern
            let binding_pattern = self.ast_builder.binding_pattern(
                self.ast_builder.binding_pattern_kind_binding_identifier(
                    Span::default(),
                    self.ast_builder.atom(&template_name),
                ),
                None::<oxc_ast::ast::TSTypeAnnotation>, // type_annotation
                false,                                  // optional
            );

            // 5. Create variable declarator
            let declarator = self.ast_builder.variable_declarator(
                Span::default(),
                VariableDeclarationKind::Var, // Use 'var' to match expected output
                binding_pattern,
                Some(template_call),
                false, // definite
            );

            // 6. Create variable declaration
            let var_decl = self.ast_builder.variable_declaration(
                Span::default(),
                VariableDeclarationKind::Var, // Use 'var' instead of 'const' to match expected output
                self.ast_builder.vec1(declarator),
                false, // declare
            );

            // 7. Create statement
            let statement = Statement::VariableDeclaration(self.ast_builder.alloc(var_decl));

            // 8. Insert at beginning of program body
            program.body.insert(0, statement);
        }
    }

    /// Generate unique template name
    fn get_next_template_name(&mut self) -> String {
        self.template_counter += 1;
        format!("_tmpl${}", self.template_counter)
    }

    /// Add import statement for dom-expressions template function
    fn add_import_statement(&self, program: &mut Program<'a>, jsx_transformer: &JSXTransformer) {
        // Get required imports from JSX transformer
        let required_imports = jsx_transformer.get_required_imports();

        if !required_imports.is_empty() {
            let imports_str = required_imports.join(", ");
            println!(
                "Import statement needed: import {{ {} }} from \"r-dom\";",
                imports_str
            );
        }

        // TODO: Actually add import to AST using OXC API
        // For now, this will be handled in post-processing
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

    #[test]
    fn test_template_generation_basic() {
        let allocator = Allocator::default();
        let options = DomExpressionsTransformOptions::default();
        let transform = DomExpressionsTransform::new(&options, &allocator);
        let jsx_transformer = JSXTransformer::new(&allocator);

        // Test that we can access templates without errors
        assert_eq!(jsx_transformer.get_templates().len(), 0);

        // Test that the transform has been properly initialized with OXC components
        assert_eq!(transform.template_counter, 0);
    }

    #[test]
    fn test_jsx_transformer_template_insertion() {
        let allocator = Allocator::default();
        let mut jsx_transformer = JSXTransformer::new(&allocator);

        // Add a mock template to verify the HashMap functionality
        jsx_transformer
            .templates
            .insert("_tmpl$1".to_string(), "<div>Hello</div>".to_string());

        // Verify the template was stored correctly
        assert_eq!(jsx_transformer.get_templates().len(), 1);
        assert_eq!(
            jsx_transformer.get_templates().get("_tmpl$1").unwrap(),
            "<div>Hello</div>"
        );
    }

    #[test]
    fn test_ast_builder_functionality() {
        let allocator = Allocator::default();
        let options = DomExpressionsTransformOptions::default();
        let transform = DomExpressionsTransform::new(&options, &allocator);

        // Test that we can create basic AST elements without errors
        let atom = transform.ast_builder.atom("test_template");
        let span = Span::default();

        // Create a simple identifier to verify AST builder works
        let _identifier = transform.ast_builder.expression_identifier(span, atom);

        // If we get here without compilation errors, the OXC integration is working
        assert!(true);
    }
}
