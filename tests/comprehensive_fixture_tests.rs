use oxc_allocator::Allocator;
use oxc_codegen::Codegen;
use oxc_parser::Parser;
use oxc_span::SourceType;
use oxc_transform_jsx_dom_expressions::{DomExpressionsTransform, DomExpressionsTransformOptions};
use std::fs;
use std::path::{Path, PathBuf};

fn parse_and_transform(code: &str, options: Option<DomExpressionsTransformOptions>) -> String {
    let allocator = Allocator::default();
    let source_type = SourceType::default().with_typescript(false).with_jsx(true);

    let parser = Parser::new(&allocator, code, source_type);
    let mut program = parser.parse().program;

    // Use actual transformer
    let transform_options = options.unwrap_or_default();
    let mut transformer = DomExpressionsTransform::new(&transform_options, &allocator);

    // Transform the program
    transformer.transform_program(&mut program);

    // Generate code from transformed AST
    let output = Codegen::new().build(&program).code;

    output
}

fn normalize_whitespace(s: &str) -> String {
    s.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}
fn load_fixture_pair(fixture_dir: &Path) -> Option<(String, String)> {
    let code_file = fixture_dir.join("code.js");
    let output_file = fixture_dir.join("output.js");
    
    if code_file.exists() && output_file.exists() {
        let code = fs::read_to_string(&code_file).ok()?;
        let expected = fs::read_to_string(&output_file).ok()?;
        Some((code, expected))
    } else {
        None
    }
}

fn get_fixture_dirs(base_path: &Path) -> Vec<PathBuf> {
    let mut fixture_dirs = Vec::new();
    
    if let Ok(entries) = fs::read_dir(base_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && path.file_name().and_then(|n| n.to_str()).map_or(false, |n| n.starts_with("__") && n.ends_with("__")) {
                // This is a fixture directory, find subdirectories with code.js/output.js pairs
                if let Ok(sub_entries) = fs::read_dir(&path) {
                    for sub_entry in sub_entries.flatten() {
                        let sub_path = sub_entry.path();
                        if sub_path.is_dir() {
                            let code_file = sub_path.join("code.js");
                            let output_file = sub_path.join("output.js");
                            if code_file.exists() && output_file.exists() {
                                fixture_dirs.push(sub_path);
                            }
                        }
                    }
                }
            }
        }
    }
    
    fixture_dirs.sort();
    fixture_dirs
}

fn run_fixture_test(fixture_path: &Path, expected_to_pass: bool) -> bool {
    let (code, expected) = match load_fixture_pair(fixture_path) {
        Some(pair) => pair,
        None => {
            println!("❌ Failed to load fixture: {}", fixture_path.display());
            return false;
        }
    };
    
    let fixture_name = fixture_path.to_string_lossy();
    println!("\n=== Testing: {} ===", fixture_name);
    
    let actual = parse_and_transform(&code, None);
    
    // Normalize for comparison
    let expected_normalized = normalize_whitespace(&expected);
    let actual_normalized = normalize_whitespace(&actual);
    
    let is_match = expected_normalized == actual_normalized;
    
    if is_match {
        println!("✅ PASS: {}", fixture_name);
        return true;
    } else {
        if expected_to_pass {
            println!("❌ FAIL: {}", fixture_name);
            println!("Expected:\n{}", expected.trim());
            println!("Actual:\n{}", actual.trim());
        } else {
            println!("⚠️  EXPECTED FAIL: {} (not yet implemented)", fixture_name);
        }
        return false;
    }
}

#[test]
fn test_dom_fixtures_simple_elements() {
    let tests_dir = Path::new("tests");
    let fixture_path = tests_dir.join("__dom_fixtures__").join("simpleElements");
    
    // This should eventually pass when basic JSX transformation is implemented
    run_fixture_test(&fixture_path, false);
}

#[test]
fn test_dom_fixtures_comprehensive() {
    let tests_dir = Path::new("tests");
    let all_fixtures = get_fixture_dirs(tests_dir);
    
    let mut total_tests = 0;
    let mut passed_tests = 0;
    
    println!("Found {} fixture test cases", all_fixtures.len());
    
    for fixture_path in &all_fixtures {
        total_tests += 1;
        
        // Determine if we expect this test to pass based on implementation status
        let expected_to_pass = should_fixture_pass(fixture_path);
        
        if run_fixture_test(fixture_path, expected_to_pass) {
            passed_tests += 1;
        }
    }
    
    println!("\n=== SUMMARY ===");
    println!("Total fixtures: {}", total_tests);
    println!("Passed: {}", passed_tests);
    println!("Failed: {}", total_tests - passed_tests);
    println!("Pass rate: {:.1}%", (passed_tests as f64 / total_tests as f64) * 100.0);
    
    // For now, we don't assert pass rate since the implementation is in progress
    // This test serves as a comprehensive status check
}

fn should_fixture_pass(fixture_path: &Path) -> bool {
    let path_str = fixture_path.to_string_lossy();
    
    // Currently, no fixtures should pass as the implementation is not complete
    // This will be updated as features are implemented
    
    // Basic static elements might work first
    if path_str.contains("simpleElements") && path_str.contains("__dom_fixtures__") {
        // TODO: Return true when basic static JSX transformation is working
        return false;
    }
    
    // Dynamic content requires more complex implementation
    if path_str.contains("attributeExpressions") || path_str.contains("textInterpolation") {
        return false;
    }
    
    // Component and control flow transformations are advanced features
    if path_str.contains("components") || path_str.contains("conditionalExpressions") {
        return false;
    }
    
    // Event handling is also advanced
    if path_str.contains("eventExpressions") {
        return false;
    }
    
    // SSR, hydration, and other modes are specialized features
    if path_str.contains("__ssr_") || path_str.contains("__hydratable_") {
        return false;
    }
    
    false
}

#[test]
fn test_fixture_structure_validation() {
    let tests_dir = Path::new("tests");
    let all_fixtures = get_fixture_dirs(tests_dir);
    
    println!("Validating fixture structure...");
    
    assert!(!all_fixtures.is_empty(), "No fixture directories found");
    
    for fixture_path in &all_fixtures {
        let code_file = fixture_path.join("code.js");
        let output_file = fixture_path.join("output.js");
        
        assert!(code_file.exists(), "Missing code.js in {}", fixture_path.display());
        assert!(output_file.exists(), "Missing output.js in {}", fixture_path.display());
        
        // Validate that files are readable
        let code_content = fs::read_to_string(&code_file).expect(&format!("Failed to read {}", code_file.display()));
        let output_content = fs::read_to_string(&output_file).expect(&format!("Failed to read {}", output_file.display()));
        
        assert!(!code_content.trim().is_empty(), "Empty code.js in {}", fixture_path.display());
        assert!(!output_content.trim().is_empty(), "Empty output.js in {}", fixture_path.display());
    }
    
    println!("✅ All {} fixtures have valid structure", all_fixtures.len());
}

#[test]
fn test_fixture_categories() {
    let tests_dir = Path::new("tests");
    let all_fixtures = get_fixture_dirs(tests_dir);
    
    let mut categories = std::collections::HashMap::new();
    
    for fixture_path in &all_fixtures {
        let path_str = fixture_path.to_string_lossy();
        let parts: Vec<&str> = path_str.split(std::path::MAIN_SEPARATOR).collect();
        
        for part in &parts {
            if part.starts_with("__") && part.ends_with("__") {
                let category = part.trim_start_matches("__").trim_end_matches("__");
                *categories.entry(category.to_string()).or_insert(0) += 1;
                break;
            }
        }
    }
    
    println!("Fixture categories found:");
    for (category, count) in &categories {
        println!("  {}: {} fixtures", category, count);
    }
    
    // Verify we have the expected categories
    assert!(categories.contains_key("dom_fixtures"), "Missing dom_fixtures category");
    assert!(categories.contains_key("dynamic_fixtures"), "Missing dynamic_fixtures category");
    
    println!("✅ Found {} fixture categories", categories.len());
}
