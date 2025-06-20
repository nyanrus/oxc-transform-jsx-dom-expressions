# Development Continuation Guide & Roadmap

## Development Phase Overview

The project is currently 20% complete and aims to reach 100% completion through the following 4 phases.

### Phase 1: Dynamic Attribute Processing (20% → 45%)
**Estimated Duration**: 2-3 weeks  
**Priority**: 🔴 Highest  
**Impact**: Significant improvement in attributeExpressions test cases

### Phase 2: Text Interpolation and Content Processing (45% → 65%)
**Estimated Duration**: 2-3 weeks  
**Priority**: 🟠 High  
**Impact**: Complete support for textInterpolation test cases

### Phase 3: Event System (65% → 85%)
**Estimated Duration**: 3-4 weeks  
**Priority**: 🟡 Medium  
**Impact**: Complete support for eventExpressions test cases

### Phase 4: Optimization and Completion (85% → 100%)
**Estimated Duration**: 2-3 weeks  
**Priority**: 🟢 Medium  
**Impact**: Performance improvement and stability assurance

---

## Phase 1: Dynamic Attribute Processing

### 1.1 JSX Expression Container Support

**Implementation Target**: Extend `extract_template()` function in `src/transformer/jsx.rs`

**Required Implementation:**

```rust
// 1. Expression Container detection
JSXAttributeValue::ExpressionContainer(expr_container) => {
    match &expr_container.expression {
        JSXExpression::Expression(expr) => {
            // Process as dynamic attribute
            self.handle_dynamic_attribute(attr_name, expr);
        }
    }
}
```

**Patterns to Support:**
- `id={variable}` → Variable reference
- `className={computedClass}` → Computed expression
- `disabled={isDisabled}` → Boolean expression
- `style={{color: 'red'}}` → Object expression

**Implementation Steps:**
1. **Expression analyzer implementation** (Create new `src/utils/expression_analyzer.rs`)
2. **Dynamic attribute collector implementation**
3. **Runtime function call generation**
4. **Template separation logic**

### 1.2 Runtime Function Integration

**New Implementation**: `src/utils/runtime_imports.rs`

```rust
pub struct RuntimeImportManager {
    imports: HashSet<String>,
}

impl RuntimeImportManager {
    pub fn add_import(&mut self, func_name: &str) {
        match func_name {
            "setAttribute" => self.imports.insert("setAttribute as _$setAttribute".to_string()),
            "effect" => self.imports.insert("effect as _$effect".to_string()),
            "className" => self.imports.insert("className as _$className".to_string()),
            // Other functions...
        };
    }
    
    pub fn generate_import_statements(&self) -> Vec<String> {
        // Generate import statements
    }
}
```

**Runtime Functions to Support:**
- `_$setAttribute(el, name, value)` - Dynamic attribute setting
- `_$effect(() => ...)` - Reactive updates
- `_$className(el, classes)` - Class name setting
- `_$style(el, styles)` - Style setting

### 1.3 Template Separation Algorithm

**Core Concept**: Separation of static and dynamic parts

**Implementation Example:**
```rust
pub struct TemplateSegment {
    pub static_html: String,
    pub dynamic_bindings: Vec<DynamicBinding>,
}

pub struct DynamicBinding {
    pub binding_type: BindingType,
    pub target_path: String,  // e.g., "firstChild.nextSibling"
    pub expression: String,
    pub runtime_func: String,
}

pub enum BindingType {
    Attribute { name: String },
    Property { name: String },
    TextContent,
    Style { property: Option<String> },
    Class,
}
```

**Transformation Example:**
```jsx
// Input
<div id={dynamicId} class="static">Content</div>

// Output
var _tmpl$ = /*#__PURE__*/ _$template(`<div class="static">Content</div>`);
const result = (() => {
  var _el$ = _tmpl$();
  _$effect(() => _$setAttribute(_el$, "id", dynamicId));
  return _el$;
})();
```

### 1.4 Test Targets and Verification Method

**Target Test Fixtures:**
- `tests/__dom_fixtures__/attributeExpressions/` (272 lines of test cases)
- Important patterns:
  - Basic dynamic attributes: `id={id}`, `title={welcoming()}`
  - Spread attributes: `{...results}`, `{...props}`
  - Composite attributes: `style={{color: red}}`, `classList={{selected: true}}`

**Implementation Verification Steps:**
```bash
# 1. Single case test
cargo run -- tests/__dom_fixtures__/attributeExpressions/code.js --output actual.js
diff actual.js tests/__dom_fixtures__/attributeExpressions/output.js

# 2. Regression test
cargo test test_attribute_expressions

# 3. Benchmark
cargo bench attribute_bench
```

---

## Phase 2: Text Interpolation and Content Processing

### 2.1 JSX Expression in Children Support

**Implementation Target**: Extended JSXChild processing

**Required Implementation:**
```rust
JSXChild::ExpressionContainer(expr_container) => {
    match &expr_container.expression {
        JSXExpression::Expression(expr) => {
            // Process as text interpolation
            self.handle_text_interpolation(expr);
        }
    }
}
```

**Supported Patterns:**
- `<span>Hello {name}</span>` → Single interpolation
- `<span>{greeting} {name}</span>` → Multiple interpolation
- `<span>{getValue()}</span>` → Function call interpolation
- `<span>{condition ? 'A' : 'B'}</span>` → Conditional interpolation

### 2.2 Text Insertion Point Generation

**Core Implementation**: `src/utils/text_insertion.rs`

```rust
pub struct TextInsertionPoint {
    pub position: InsertionPosition,
    pub expression: String,
    pub is_static: bool,
}

pub enum InsertionPosition {
    Before(String),  // Before DOM element
    After(String),   // After DOM element
    Replace(String), // Element replacement
    Append,          // Append to end
}
```

**HTML Generation Algorithm:**
```rust
// Input: <span>Hello {name}</span>
// 1. Detect static part: "Hello "
// 2. Detect dynamic part: {name}
// 3. Generate template: `<span>Hello </span>`
// 4. Record insertion point: After("Hello "), expression: "name"
```

### 2.3 _$insert Function Integration

**Code to Generate:**
```javascript
var _tmpl$ = /*#__PURE__*/ _$template(`<span>Hello </span>`);
const result = (() => {
  var _el$ = _tmpl$();
  _$insert(_el$, name, null);
  return _el$;
})();
```

**_$insert Call Patterns:**
- `_$insert(parent, content, before)` - Basic insertion
- `_$insert(parent, () => expression, before)` - Reactive insertion

### 2.4 Complex Text Processing

**Whitespace Normalization:**
```rust
fn normalize_jsx_text(text: &str) -> String {
    // Normalization following JSX whitespace rules
    // Remove leading/trailing whitespace
    // Convert consecutive whitespace to single space
    // But preserve intentional whitespace
}
```

**Escape Processing:**
```rust
fn escape_jsx_text(text: &str) -> String {
    text.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
}
```

**Target Test Fixtures:**
- `tests/__dom_fixtures__/textInterpolation/` (94 lines of test cases)

---

## Phase 3: Event System

### 3.1 Event Handler Detection

**Implementation Target**: Event attribute classification and processing

```rust
pub enum EventHandlerType {
    Delegated(String),    // onClick → "click"
    Bound(String),        // onchange → "change"  
    Custom(String),       // on:custom → "custom"
    Capture(String),      // oncapture:click → "click"
}

fn classify_event_attribute(attr_name: &str) -> Option<EventHandlerType> {
    if attr_name.starts_with("on:") {
        Some(EventHandlerType::Custom(attr_name[3..].to_string()))
    } else if attr_name.starts_with("oncapture:") {
        Some(EventHandlerType::Capture(attr_name[10..].to_string()))
    } else if attr_name.starts_with("on") && is_delegated_event(&attr_name[2..]) {
        Some(EventHandlerType::Delegated(attr_name[2..].to_lowercase()))
    } else if attr_name.starts_with("on") {
        Some(EventHandlerType::Bound(attr_name[2..].to_lowercase()))
    } else {
        None
    }
}
```

### 3.2 Event Delegation System

**Implementation Strategy:**
- `onClick`, `onInput` etc. are delegation targets → `_$delegateEvents()`
- `onChange`, `onFocus` etc. are direct binding → `addEventListener()`

**Delegated Event Generation:**
```rust
// For onClick={handler}
element_var.$$click = handler;

// For onClick={[handler, data]}  
element_var.$$click = handler;
element_var.$$clickData = data;
```

**Delegated Event Registration:**
```javascript
_$delegateEvents(["click", "input"]); // Added to end of file
```

### 3.3 Custom Event Processing

**`on:*` Format Processing:**
```rust
// on:customEvent={handler}
_$addEventListener(element, "customEvent", handler);

// on:event={{handleEvent: fn, once: true}}
_$addEventListener(element, "event", {handleEvent: fn, once: true});
```

### 3.4 Event Handler Optimization

**Static Analysis Optimization:**
- Function reference case: Direct assignment
- Inline function case: Wrapper generation
- Array format case: Data binding

**Target Test Fixtures:**
- `tests/__dom_fixtures__/eventExpressions/` (38 lines)
- `tests/__dom_hydratable_fixtures__/eventExpressions/` (29 lines)

---

## Phase 4: Optimization and Completion

### 4.1 Static Analysis Engine

**Implementation**: `src/transformer/optimization.rs`

```rust
pub struct StaticAnalyzer {
    scope_tracker: ScopeTracker,
    constant_tracker: ConstantTracker,
}

impl StaticAnalyzer {
    pub fn analyze_expression(&self, expr: &Expression) -> AnalysisResult {
        // Static analysis of expressions
        // - Whether it's a constant expression
        // - Whether it has side effects  
        // - Dependency tracking
    }
    
    pub fn optimize_template(&self, template: &Template) -> OptimizedTemplate {
        // Template-level optimization
        // - Remove unnecessary _$effect()
        // - Pre-calculate static values
        // - Merge duplicate processing
    }
}
```

### 4.2 Code Generation Optimization

**Optimization Patterns:**
1. **Static Value Pre-calculation**: `{1 + 2}` → `3`
2. **Unnecessary Effect Removal**: Eliminate reactive processing for static values
3. **Template Consolidation**: Reuse templates with identical patterns
4. **Inlining**: Inline expansion of simple function calls

### 4.3 Error Handling and Diagnostics

**Implementation**: `src/diagnostics/`

```rust
pub struct TransformDiagnostics {
    errors: Vec<TransformError>,
    warnings: Vec<TransformWarning>,
}

pub struct TransformError {
    pub span: Span,
    pub message: String,
    pub suggestion: Option<String>,
}
```

**Error Cases:**
- Unsupported JSX syntax
- Invalid attribute combinations
- Missing runtime functions

### 4.4 Performance Optimization

**Target Numbers:**
- 5-10x faster than Babel plugins
- 30-50% memory usage reduction
- Output size optimization

**Benchmark Implementation:**
```rust
// benches/transform_bench.rs
#[bench]
fn bench_large_component(b: &mut Bencher) {
    let large_jsx = include_str!("../test-data/large-component.jsx");
    b.iter(|| transform_code(large_jsx, &options));
}
```

---

## Implementation Hints and Reference Information

### OXC API Usage Patterns

**Expression Processing:**
```rust
match expr {
    Expression::Identifier(ident) => {
        // Variable reference: ident.name
    }
    Expression::CallExpression(call) => {
        // Function call
    }
    Expression::ObjectExpression(obj) => {
        // Object literal
    }
    Expression::ArrayExpression(arr) => {
        // Array literal
    }
}
```

**AST Construction Patterns:**
```rust
// IIFE (Immediately Invoked Function Expression) generation
let iife = self.ast_builder.expression_call(
    span,
    self.ast_builder.expression_arrow_function_expression(
        span,
        false, // expression
        false, // async
        None,  // type_parameters
        self.ast_builder.formal_parameters(span, params, None),
        None,  // return_type
        body,
    ),
    None, // type_arguments
    arguments,
    false, // optional
);
```

### dom-expressions Transformation Patterns

**Basic Templates:**
```javascript
// Pattern 1: Static elements
var _tmpl$ = /*#__PURE__*/ _$template(`<div>Static</div>`);
const result = _tmpl$();

// Pattern 2: Dynamic attributes
var _tmpl$2 = /*#__PURE__*/ _$template(`<div>Content</div>`);
const result2 = (() => {
  var _el$ = _tmpl$2();
  _$effect(() => _$setAttribute(_el$, "id", dynamicId));
  return _el$;
})();

// Pattern 3: Text interpolation
var _tmpl$3 = /*#__PURE__*/ _$template(`<div></div>`);
const result3 = (() => {
  var _el$ = _tmpl$3();
  _$insert(_el$, textContent);
  return _el$;
})();
```

### Test Fixture Support Strategy

**Gradual Approach:**
1. **Single Feature Tests**: Tests focused on one feature
2. **Combination Tests**: Tests combining multiple features  
3. **Edge Case Tests**: Boundary cases and error cases
4. **Performance Tests**: Performance tests with large components

**Test Execution Commands:**
```bash
# Test specific fixtures
cargo test --test fixture_test -- simpleElements

# Run benchmarks
cargo bench

# Memory profiling
cargo test --release --features profiling
```

### Recommended Development Environment Setup

**Required Tools:**
```bash
# Rust toolchain
rustup install stable
rustup component add clippy rustfmt

# Development support tools
cargo install cargo-watch    # File change monitoring
cargo install cargo-expand   # Macro expansion verification
cargo install flamegraph     # Performance analysis
```

**VS Code Extensions:**
- rust-analyzer: Rust language support
- CodeLLDB: Debug support
- Better TOML: Cargo.toml editing support

**Debug Configuration:**
```json
// .vscode/launch.json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug Transform",
            "cargo": {
                "args": ["build", "--bin=oxc-transform-jsx-dom-expressions"],
                "filter": {
                    "name": "oxc-transform-jsx-dom-expressions",
                    "kind": "bin"
                }
            },
            "args": ["test_simple.jsx"],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```

---

## Detailed Task Lists for Each Phase

### Phase 1 Tasks (Dynamic Attribute Processing)

**Week 1:**
- [ ] Implement Expression Container analyzer
- [ ] Support basic dynamic attributes (`id={var}`)
- [ ] Create unit tests

**Week 2:**
- [ ] Style object processing (`style={{color: 'red'}}`)
- [ ] ClassList processing (`classList={{active: true}}`)
- [ ] Runtime function integration

**Week 3:**
- [ ] Spread attribute processing (`{...props}`)
- [ ] Complex case support
- [ ] Regression tests, performance measurement

### Phase 2 Tasks (Text Interpolation)

**Week 1:**
- [ ] JSX Expression in Children detection
- [ ] Single interpolation support (`{name}`)
- [ ] Text insertion point generation

**Week 2:**
- [ ] Multiple interpolation support (`{a} {b}`)
- [ ] Whitespace normalization processing
- [ ] HTML escape processing

**Week 3:**
- [ ] Conditional interpolation (`{cond ? 'A' : 'B'}`)
- [ ] Complex expression support
- [ ] Text node optimization

### Phase 3 Tasks (Event System)

**Week 1:**
- [ ] Implement event attribute classifier
- [ ] Basic event handler support

**Week 2:**
- [ ] Implement event delegation system
- [ ] Custom event support (`on:*`)

**Week 3:**
- [ ] Event data binding
- [ ] Capture event support

**Week 4:**
- [ ] Event handler optimization
- [ ] Edge case support

### Phase 4 Tasks (Optimization and Completion)

**Week 1:**
- [ ] Implement static analysis engine
- [ ] Basic optimization rules

**Week 2:**
- [ ] Enhance error handling and diagnostics
- [ ] Performance measurement and improvement

**Week 3:**
- [ ] Complete documentation
- [ ] Fix remaining bugs
- [ ] Release preparation

---

**Roadmap Last Updated**: June 19, 2025  
**Total Development Duration Estimate**: 10-13 weeks  
**Target Completion Date**: End of September 2025