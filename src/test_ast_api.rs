// Test file to understand OXC AST API
use oxc_ast::AstBuilder;
use oxc_allocator::Allocator;
use oxc_span::SPAN;
use oxc_ast::ast::*;

#[allow(dead_code)]
pub fn test_create_call_expression() {
    let allocator = Allocator::default();
    let ast_builder = AstBuilder::new(&allocator);
    
    // Create a call expression: _tmpl$1()
    let identifier = ast_builder.identifier_name(SPAN, "_tmpl$1");
    let callee = ast_builder.identifier_reference_expression(identifier);
    let arguments = ast_builder.vec();
    let call_expr = ast_builder.call_expression(SPAN, callee, None, arguments, false);
    
    println!("Created call expression: {:?}", call_expr);
}

#[allow(dead_code)]
pub fn test_create_variable_declaration() {
    let allocator = Allocator::default();
    let ast_builder = AstBuilder::new(&allocator);
    
    // Create: const _tmpl$1 = template('...');
    let template_name = ast_builder.identifier_name(SPAN, "_tmpl$1");
    let template_call = create_template_call(&ast_builder, "<div>Hello</div>");
    
    let variable_declarator = ast_builder.variable_declarator(
        SPAN,
        VariableDeclarationKind::Const,
        ast_builder.binding_pattern_kind_binding_identifier(SPAN, template_name),
        Some(template_call),
        false,
    );
    
    let declarations = ast_builder.vec1(variable_declarator);
    let var_decl = ast_builder.variable_declaration(SPAN, VariableDeclarationKind::Const, declarations, false);
    
    println!("Created variable declaration: {:?}", var_decl);
}

fn create_template_call(ast_builder: &AstBuilder, template_html: &str) -> Expression {
    // Create: template('...')
    let template_name = ast_builder.identifier_name(SPAN, "template");
    let callee = ast_builder.identifier_reference_expression(template_name);
    
    let template_string = ast_builder.literal_string_expression(StringLiteral::new(SPAN, template_html.into()));
    let arguments = ast_builder.vec1(Argument::from(template_string));
    
    ast_builder.call_expression(SPAN, callee, None, arguments, false)
}
