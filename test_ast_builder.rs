use oxc_ast::AstBuilder;
use oxc_allocator::Allocator;

fn main() {
    let allocator = Allocator::default();
    let ast_builder = AstBuilder::new(&allocator);
    
    // List some available methods
    println!("AstBuilder methods:");
    // Try to find correct method names by compilation error
}
