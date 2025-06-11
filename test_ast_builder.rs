use oxc_ast::AstBuilder;
use oxc_allocator::Allocator;
use oxc_span::SPAN;

fn main() {
    let allocator = Allocator::default();
    let ast_builder = AstBuilder::new(&allocator);
    
    // Create a call expression: _tmpl$1()
    let identifier = ast_builder.identifier_name(SPAN, "_tmpl$1");
    let callee = ast_builder.identifier_reference_expression(identifier);
    let arguments = ast_builder.vec();
    let call_expr = ast_builder.call_expression(SPAN, callee, None, arguments, false);
    
    println!("Created call expression: {:?}", call_expr);
}
