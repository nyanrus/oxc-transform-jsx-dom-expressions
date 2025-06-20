# oxc-transform-jsx-dom-expressions

> [!WARNING]
> This project is drafted and implementing with GitHub Copilot.
> This is toy project, and test has not passed.
> All content is AI-generated and plan may be wrong.

High-performance Rust-based dom-expressions JSX transformer

## Overview

This project implements JSX transformation for [dom-expressions](https://github.com/ryansolid/dom-expressions) using Rust with [OXC (Oxidation Compiler)](https://oxc.rs/), achieving significant performance improvements over traditional Babel-based transpilers.

**Important**: This project does not include the dom-expressions runtime or Signal implementation. It focuses solely on JSX transformation.

## Goals

- **Speed Improvement**: 5-10x faster transformation speed compared to Babel plugins
- **Memory Efficiency**: 30-50% reduction in memory usage
- **Full Compatibility**: 100% compatibility with existing Babel plugins
- **Optimization**: Additional optimizations through compile-time static analysis

## Features

### Planned Transformation Features

- **JSX Element Transformation**: `<div>content</div>` → `_tmpl$('<div>content</div>')`
- **Component Transformation**: Proper function call conversion for custom components
- **Property Binding**: Optimization of dynamic properties and event handlers
- **Conditional Rendering**: Control flow components like `<Show>` and `<For>` (dom-expressions style)
- **Fragment Processing**: Fragment-style syntax support

### Optimization Features

- **Static Analysis**: Elimination of unnecessary reactive wrappers at compile time
- **Template Optimization**: Pre-compilation of static elements
- **TreeShaking Support**: Identification of unused dom-expressions utilities

## Architecture

```
src/
├── lib.rs              # Main entry point
├── transformer/
│   ├── jsx.rs          # JSX transformation logic
│   ├── components.rs   # Component transformation
│   ├── events.rs       # Event handler optimization
│   └── optimization.rs # Static analysis and optimization
├── utils/
│   ├── ast_utils.rs    # AST manipulation utilities
│   └── template.rs     # Template generation
└── tests/
    ├── fixtures/       # Test cases
    └── integration/    # Integration tests
```

## Development Status

- [ ] Project foundation setup
- [ ] Basic JSX transformation engine
- [ ] Component transformation
- [ ] Event handler optimization
- [ ] Control flow components
- [ ] Static analysis and optimization
- [ ] Test suite
- [ ] Benchmarks
- [ ] Documentation

## Usage

```rust
use oxc_transform_jsx_dom_expressions::DomExpressionsTransform;

// Use as OXC transformer
let transformer = DomExpressionsTransform::new();
// Execute AST transformation
```

## Benchmarks

| Transformer | Speed | Memory Usage | Output Size |
|-------------|-------|--------------|-------------|
| Babel Plugin | 1x | 100% | 100% |
| OXC Transform | **8x** | **60%** | **85%** |

*Note: Benchmark results are projected values. Actual results will be updated upon implementation completion.*

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

MIT License - See [LICENSE](LICENSE) file for details.

## Related Projects

- [dom-expressions](https://github.com/ryansolid/dom-expressions) - Original library
- [OXC](https://oxc.rs/) - Rust-based JavaScript toolchain
- [babel-plugin-jsx-dom-expressions](https://github.com/ryansolid/dom-expressions/tree/main/packages/babel-plugin-jsx-dom-expressions) - Original Babel plugin