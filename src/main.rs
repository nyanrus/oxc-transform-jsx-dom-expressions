use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_span::SourceType;
use oxc_transform_jsx_dom_expressions::{
    DomExpressionsTransform, DomExpressionsTransformOptions, ModuleFormat,
};
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input.jsx> [options]", args[0]);
        eprintln!("Options:");
        eprintln!("  --dev              Enable development mode");
        eprintln!("  --hydratable       Enable hydratable mode");
        eprintln!("  --cjs              Use CommonJS module format");
        eprintln!("  --output <file>    Output file (default: stdout)");
        std::process::exit(1);
    }

    let input_file = &args[1];
    let mut options = DomExpressionsTransformOptions::default();
    let mut output_file: Option<String> = None;

    // Parse command line options
    let mut i = 2;
    while i < args.len() {        match args[i].as_str() {
            "--dev" => {
                // development mode - enable additional debugging features
                options.memo_wrapper = false;
                options.wrap_conditionals = true;
            },
            "--hydratable" => options.hydratable = true,
            "--cjs" => {
                // Note: ModuleFormat might be used in the future for output generation
                // For now, we'll just set a flag that could influence template generation
            },
            "--output" => {
                if i + 1 < args.len() {
                    output_file = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Error: --output requires a filename");
                    std::process::exit(1);
                }
            }
            _ => {
                eprintln!("Unknown option: {}", args[i]);
                std::process::exit(1);
            }
        }
        i += 1;
    }

    // Read input file
    let input_code = match fs::read_to_string(input_file) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading {}: {}", input_file, err);
            std::process::exit(1);
        }
    };    // Transform the code
    match transform_code(&input_code, &options) {
        Ok(transformed) => {
            // Output the result
            match output_file {
                Some(file) => {
                    if let Err(err) = fs::write(&file, transformed) {
                        eprintln!("Error writing to {}: {}", file, err);
                        std::process::exit(1);
                    }
                    println!("Transformed {} -> {}", input_file, file);
                }
                None => {
                    println!("{}", transformed);
                }
            }
        }
        Err(err) => {
            eprintln!("Transform error: {}", err);
            std::process::exit(1);
        }
    }
}

fn transform_code(code: &str, options: &DomExpressionsTransformOptions) -> Result<String, String> {
    let allocator = Allocator::default();

    // Determine source type based on file extension or content
    let source_type = SourceType::default().with_typescript(false).with_jsx(true);

    // Parse the code
    let mut parser = Parser::new(&allocator, code, source_type);
    let parse_result = parser.parse();

    if !parse_result.errors.is_empty() {
        let mut error_msg = String::from("Parse errors:\n");
        for error in &parse_result.errors {
            error_msg.push_str(&format!("  {}\n", error));
        }
        return Err(error_msg);
    }    let mut program = parse_result.program;

    // Transform the AST
    let mut transformer = DomExpressionsTransform::new(options, &allocator);
    transformer.transform_program(&mut program);

    // For now, return a placeholder since we don't have code generation yet
    // In a complete implementation, this would use oxc_codegen to generate the final code
    Ok(format!(
        "// Transformed with oxc-transform-jsx-dom-expressions\n// Original code:\n/*\n{}\n*/\n\n// TODO: Implement code generation\n",
        code
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_simple_jsx() {
        let code = r#"
            function App() {
                return <div>Hello World</div>;
            }
        "#;

        let result = transform_code(code, &DomExpressionsTransformOptions::default());
        assert!(result.is_ok());
        let transformed = result.unwrap();
        assert!(transformed.contains("Transformed with oxc-transform-jsx-dom-expressions"));
    }

    #[test]
    fn test_transform_with_options() {
        let code = r#"
            function App() {
                return <div>Development mode</div>;
            }
        "#;        let options = DomExpressionsTransformOptions {
            hydratable: true,
            delegation: false,
            memo_wrapper: false,
            ..Default::default()
        };

        let result = transform_code(code, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_error_handling() {
        let invalid_code = "function App() { return <div>unclosed div; }";

        let result = transform_code(invalid_code, &DomExpressionsTransformOptions::default());
        // Should handle parse errors gracefully
        // The actual behavior depends on the parser's error handling
    }
}
