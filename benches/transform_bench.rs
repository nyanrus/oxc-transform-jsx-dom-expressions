use criterion::{black_box, criterion_group, criterion_main, Criterion};
use oxc_transform_solid::{SolidJsTransformer, SolidTransformOptions};
use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_span::SourceType;

fn parse_jsx_code(code: &str) -> oxc_ast::ast::Program {
    let allocator = Allocator::default();
    let source_type = SourceType::default().with_typescript(false).with_jsx(true);
    
    let mut parser = Parser::new(&allocator, code, source_type);
    parser.parse().program
}

fn benchmark_simple_jsx(c: &mut Criterion) {
    let jsx_code = r#"
        function App() {
            return <div>Hello World</div>;
        }
    "#;

    c.bench_function("transform_simple_jsx", |b| {
        b.iter(|| {
            let mut program = black_box(parse_jsx_code(jsx_code));
            let mut transformer = SolidJsTransformer::new();
            transformer.transform_program(&mut program);
        })
    });
}

fn benchmark_complex_jsx(c: &mut Criterion) {
    let jsx_code = r#"
        function ComplexApp() {
            const [count, setCount] = createSignal(0);
            return (
                <div class="container">
                    <h1>Counter: {count()}</h1>
                    <button onClick={() => setCount(count() + 1)}>
                        Increment
                    </button>
                    <Show when={count() > 5}>
                        <p>Count is greater than 5!</p>
                    </Show>
                    <For each={Array.from({length: count()}, (_, i) => i)}>
                        {(item) => <div>Item {item}</div>}
                    </For>
                </div>
            );
        }
    "#;

    c.bench_function("transform_complex_jsx", |b| {
        b.iter(|| {
            let mut program = black_box(parse_jsx_code(jsx_code));
            let mut transformer = SolidJsTransformer::new();
            transformer.transform_program(&mut program);
        })
    });
}

fn benchmark_nested_components(c: &mut Criterion) {
    let jsx_code = r#"
        function NestedApp() {
            return (
                <div>
                    <Header>
                        <Navigation>
                            <NavItem href="/home">Home</NavItem>
                            <NavItem href="/about">About</NavItem>
                            <NavItem href="/contact">Contact</NavItem>
                        </Navigation>
                    </Header>
                    <Main>
                        <Article>
                            <h2>Article Title</h2>
                            <p>Article content goes here...</p>
                            <Comments>
                                <Comment author="User1">Great article!</Comment>
                                <Comment author="User2">Thanks for sharing.</Comment>
                            </Comments>
                        </Article>
                    </Main>
                    <Footer>
                        <p>&copy; 2025 My App</p>
                    </Footer>
                </div>
            );
        }
    "#;

    c.bench_function("transform_nested_components", |b| {
        b.iter(|| {
            let mut program = black_box(parse_jsx_code(jsx_code));
            let mut transformer = SolidJsTransformer::new();
            transformer.transform_program(&mut program);
        })
    });
}

fn benchmark_with_optimizations(c: &mut Criterion) {
    let jsx_code = r#"
        function OptimizedApp() {
            return (
                <div class="static-class" id="static-id">
                    <h1>Static Title</h1>
                    <p>Static paragraph with no dynamic content.</p>
                    <ul>
                        <li>Static item 1</li>
                        <li>Static item 2</li>
                        <li>Static item 3</li>
                    </ul>
                </div>
            );
        }
    "#;

    let options = SolidTransformOptions {
        development: false,
        hydratable: false,
        ..Default::default()
    };

    c.bench_function("transform_with_optimizations", |b| {
        b.iter(|| {
            let mut program = black_box(parse_jsx_code(jsx_code));
            let mut transformer = SolidJsTransformer::with_options(options.clone());
            transformer.transform_program(&mut program);
        })
    });
}

fn benchmark_parser_only(c: &mut Criterion) {
    let jsx_code = r#"
        function App() {
            return <div>Hello World</div>;
        }
    "#;

    c.bench_function("parse_jsx_only", |b| {
        b.iter(|| {
            black_box(parse_jsx_code(jsx_code));
        })
    });
}

criterion_group!(
    benches,
    benchmark_simple_jsx,
    benchmark_complex_jsx,
    benchmark_nested_components,
    benchmark_with_optimizations,
    benchmark_parser_only
);
criterion_main!(benches);
