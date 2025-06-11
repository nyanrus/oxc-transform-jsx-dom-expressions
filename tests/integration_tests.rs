use oxc_transform_solid::{SolidJsTransformer, SolidTransformOptions, ModuleFormat};
use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_span::SourceType;
use oxc_codegen::{CodeGenerator, CodegenOptions};

fn parse_and_transform(code: &str, options: Option<SolidTransformOptions>) -> String {
    let allocator = Allocator::default();
    let source_type = SourceType::default().with_typescript(false).with_jsx(true);
    
    let mut parser = Parser::new(&allocator, code, source_type);
    let mut program = parser.parse().program;
    
    let mut transformer = match options {
        Some(opts) => SolidJsTransformer::with_options(opts),
        None => SolidJsTransformer::new(),
    };
    
    transformer.transform_program_with_allocator(&mut program, &allocator);
    
    // Generate code from transformed AST
    let mut codegen = CodeGenerator::new();
    codegen.build(&program).source_text
}

#[test]
fn test_simple_jsx_element() {
    let input = r#"
        function App() {
            return <div>Hello World</div>;
        }
    "#;
    
    let output = parse_and_transform(input, None);
    // For now, just ensure it doesn't panic
    assert!(!output.is_empty());
}

#[test]
fn test_jsx_with_attributes() {
    let input = r#"
        function App() {
            return <div class="container" id="main">Content</div>;
        }
    "#;
    
    let output = parse_and_transform(input, None);
    assert!(!output.is_empty());
}

#[test]
fn test_jsx_with_dynamic_content() {
    let input = r#"
        function App() {
            const name = "World";
            return <div>Hello {name}!</div>;
        }
    "#;
    
    let output = parse_and_transform(input, None);
    assert!(!output.is_empty());
}

#[test]
fn test_jsx_with_event_handlers() {
    let input = r#"
        function App() {
            const handleClick = () => console.log("clicked");
            return <button onClick={handleClick}>Click me</button>;
        }
    "#;
    
    let output = parse_and_transform(input, None);
    assert!(!output.is_empty());
}

#[test]
fn test_solid_components() {
    let input = r#"
        function App() {
            const [show, setShow] = createSignal(true);
            return (
                <Show when={show()}>
                    <div>Shown content</div>
                </Show>
            );
        }
    "#;
    
    let output = parse_and_transform(input, None);
    assert!(!output.is_empty());
}

#[test]
fn test_for_component() {
    let input = r#"
        function App() {
            const items = [1, 2, 3];
            return (
                <For each={items}>
                    {(item) => <div>{item}</div>}
                </For>
            );
        }
    "#;
    
    let output = parse_and_transform(input, None);
    assert!(!output.is_empty());
}

#[test]
fn test_nested_components() {
    let input = r#"
        function App() {
            return (
                <div>
                    <Header>
                        <h1>Title</h1>
                    </Header>
                    <Main>
                        <p>Content</p>
                    </Main>
                </div>
            );
        }
    "#;
    
    let output = parse_and_transform(input, None);
    assert!(!output.is_empty());
}

#[test]
fn test_fragment() {
    let input = r#"
        function App() {
            return (
                <>
                    <div>First</div>
                    <div>Second</div>
                </>
            );
        }
    "#;
    
    let output = parse_and_transform(input, None);
    assert!(!output.is_empty());
}

#[test]
fn test_development_mode() {
    let input = r#"
        function App() {
            return <div>Development mode</div>;
        }
    "#;
    
    let options = SolidTransformOptions {
        development: true,
        ..Default::default()
    };
    
    let output = parse_and_transform(input, Some(options));
    assert!(!output.is_empty());
}

#[test]
fn test_hydratable_mode() {
    let input = r#"
        function App() {
            return <div>Hydratable content</div>;
        }
    "#;
    
    let options = SolidTransformOptions {
        hydratable: true,
        ..Default::default()
    };
    
    let output = parse_and_transform(input, Some(options));
    assert!(!output.is_empty());
}

#[test]
fn test_cjs_module_format() {
    let input = r#"
        function App() {
            return <div>CommonJS module</div>;
        }
    "#;
    
    let options = SolidTransformOptions {
        module_format: ModuleFormat::Cjs,
        ..Default::default()
    };
    
    let output = parse_and_transform(input, Some(options));
    assert!(!output.is_empty());
}

#[test]
fn test_complex_jsx_structure() {
    let input = r#"
        function ComplexApp() {
            const [count, setCount] = createSignal(0);
            const [todos, setTodos] = createSignal([]);
            
            return (
                <div class="app">
                    <header>
                        <h1>My App</h1>
                        <nav>
                            <a href="/home">Home</a>
                            <a href="/about">About</a>
                        </nav>
                    </header>
                    
                    <main>
                        <section class="counter">
                            <h2>Counter: {count()}</h2>
                            <button onClick={() => setCount(count() + 1)}>
                                Increment
                            </button>
                            <Show when={count() > 5}>
                                <p>High count!</p>
                            </Show>
                        </section>
                        
                        <section class="todos">
                            <h2>Todos</h2>
                            <For each={todos()}>
                                {(todo, index) => (
                                    <div class="todo" data-index={index()}>
                                        <span>{todo.text}</span>
                                        <button onClick={() => removeTodo(index())}>
                                            Remove
                                        </button>
                                    </div>
                                )}
                            </For>
                        </section>
                    </main>
                    
                    <footer>
                        <p>&copy; 2025 My App</p>
                    </footer>
                </div>
            );
        }
    "#;
    
    let output = parse_and_transform(input, None);
    assert!(!output.is_empty());
}
