# IMPLEMENTATION BOT INSTRUCTIONS

## MISSION
Fix compilation errors and implement features using ONLY summary files from Summary Bot, making surgical code changes without context overflow.

## CORE PRINCIPLES
1. **NEVER** read source code files directly
2. **ONLY** read summary files created by Summary Bot
3. **MAKE** minimal, targeted changes
4. **TEST** after each change
5. **UPDATE** progress in state files

## TOOL USAGE INSTRUCTIONS

**IMPORTANT**: For proper tool usage and XML formatting, read `bot_instructions/CLINE_INSTRUCTIONS.md` first.

Use XML-style tool formatting for all operations. The tools you'll primarily use:
- `read_file` - Only for summary files from Summary Bot
- `replace_in_file` - For surgical code changes
- `write_to_file` - For creating small new files
- `execute_command` - For compilation and testing

## STARTUP PROTOCOL

### Step 1: Check Summary Bot Deliverables
```bash
# Verify Summary Bot completed its work
if [ -f "bot_instructions/state/HANDOFF_READY.flag" ]; then
    echo "Summary Bot work complete, starting implementation"
    rm bot_instructions/state/HANDOFF_READY.flag
else
    echo "ERROR: Summary Bot must complete first"
    exit 1
fi
```

### Step 2: Read Summary Files ONLY
```bash
# Read all summary files (these should be small)
cat bot_instructions/state/OXC_API_SUMMARY.md
cat bot_instructions/state/PROJECT_CURRENT_STATE.md
cat bot_instructions/state/IMPLEMENTATION_ROADMAP.md
```

### Step 3: Initialize Implementation State
```bash
# Create implementation tracking
echo "## Implementation Session Started: $(date)" > bot_instructions/state/IMPLEMENTATION_STATUS.md
echo "### Roadmap Tasks:" >> bot_instructions/state/IMPLEMENTATION_STATUS.md
echo "- [ ] Fix compilation errors" >> bot_instructions/state/IMPLEMENTATION_STATUS.md
echo "- [ ] Implement missing methods" >> bot_instructions/state/IMPLEMENTATION_STATUS.md
echo "- [ ] Add tests" >> bot_instructions/state/IMPLEMENTATION_STATUS.md
```

## ALLOWED OPERATIONS

### File Modifications:
```bash
# ALLOWED - Targeted modifications
replace_in_file  # For specific SEARCH/REPLACE operations
write_to_file    # For new small files only (< 500 lines)

# FORBIDDEN - Context overflow risks
read_file        # On any source files
```

### Testing & Verification:
```bash
# ALLOWED - Compilation and testing
cargo check                    # Verify compilation
cargo check --message-format=short  # Get concise errors
cargo test specific_test       # Run specific tests
cargo test --lib              # Test library only
```

### Information Gathering:
```bash
# ALLOWED - Targeted searches only
grep -n "specific_pattern" src/file.rs  # Find specific lines
rg "error.*missing.*field" . --type rust  # Find specific errors
head -n 5 src/file.rs         # Read only first few lines if needed
```

## IMPLEMENTATION WORKFLOW

### Phase 1: Fix Compilation Errors
Based on `PROJECT_CURRENT_STATE.md`:

1. **Read Error Summary**:
   ```bash
   # Get error details from summary (no direct file reading)
   grep -A 3 "Error.*E0063" bot_instructions/state/PROJECT_CURRENT_STATE.md
   ```

2. **Make Targeted Fix**:
   ```bash
   # Example: Fix missing field error
   replace_in_file src/transformer/mod.rs '
   ------- SEARCH
   Self {
       options,
       template_counter: 0,
       allocator,
   }
   =======
   Self {
       options,
       template_counter: 0,
       allocator,
       ast_builder: AstBuilder::new(allocator),
   }
   +++++++ REPLACE
   '
   ```

3. **Verify Fix**:
   ```bash
   cargo check --message-format=short
   ```

4. **Update Progress**:
   ```bash
   echo "- [x] Fixed E0063: missing ast_builder field" >> bot_instructions/state/IMPLEMENTATION_STATUS.md
   ```

### Phase 2: Implement Missing Features
Based on `IMPLEMENTATION_ROADMAP.md`:

1. **Read Requirements** (from summary only):
   ```bash
   grep -A 5 "Missing Methods" bot_instructions/state/IMPLEMENTATION_ROADMAP.md
   ```

2. **Implement Method** (using OXC API patterns from summary):
   ```bash
   # Add missing method based on API summary
   replace_in_file src/transformer/jsx.rs '
   ------- SEARCH
   impl JSXTransformer {
   =======
   impl JSXTransformer {
       pub fn new(allocator: &Allocator) -> Self {
           Self {
               allocator,
               templates: Vec::new(),
           }
       }
   +++++++ REPLACE
   '
   ```

3. **Test Implementation**:
   ```bash
   cargo test jsx_transformer_tests
   ```

### Phase 3: Clean Up
1. **Remove Unused Imports**:
   ```bash
   # Based on warnings from summary
   replace_in_file src/transformer/mod.rs '
   ------- SEARCH
   use oxc_ast::ast::{
       Argument, Expression, JSXElement, JSXElementName, Program, Statement, StringLiteral,
       VariableDeclarationKind,
   };
   =======
   use oxc_ast::ast::{
       Expression, JSXElementName, Program, Statement,
   };
   +++++++ REPLACE
   '
   ```

2. **Final Verification**:
   ```bash
   cargo test
   cargo check
   ```

## INFORMATION SOURCES

### Primary Sources (Summary Bot Output):
- `OXC_API_SUMMARY.md` - How to use OXC APIs correctly
- `PROJECT_CURRENT_STATE.md` - Current compilation errors and issues
- `IMPLEMENTATION_ROADMAP.md` - Prioritized tasks and requirements

### Reference Files (Read-Only):
- `bot_instructions/state/SUMMARY_MEMORY.md` - Complete API documentation
- `bot_instructions/state/CHUNK_PROGRESS.md` - What was analyzed

### NEVER Read:
- Source files (src/*.rs) except through targeted grep
- Large documentation files
- Any file > 100 lines

## ERROR HANDLING

### If Compilation Fails:
1. **Get Concise Error**:
   ```bash
   cargo check --message-format=short 2>&1 | head -10
   ```

2. **Update Error Log**:
   ```bash
   echo "## New Error: $(date)" >> bot_instructions/state/IMPLEMENTATION_STATUS.md
   echo "Error details: [paste concise error]" >> bot_instructions/state/IMPLEMENTATION_STATUS.md
   ```

3. **Make Targeted Fix** (using summary info only)

4. **Re-test**

### If Missing API Information:
1. **Check Summary Memory**:
   ```bash
   grep -A 5 "needed_api_name" bot_instructions/state/SUMMARY_MEMORY.md
   ```

2. **If Not Found, Document Need**:
   ```bash
   echo "MISSING: Need API info for X" >> bot_instructions/state/IMPLEMENTATION_STATUS.md
   echo "HANDOFF TO SUMMARY BOT NEEDED" >> bot_instructions/state/IMPLEMENTATION_STATUS.md
   ```

## PROGRESS TRACKING

### Update After Each Task:
```bash
# Mark task complete
sed -i 's/- \[ \] Fix compilation errors/- [x] Fix compilation errors/' bot_instructions/state/IMPLEMENTATION_STATUS.md

# Add completion timestamp
echo "- Completed at: $(date)" >> bot_instructions/state/IMPLEMENTATION_STATUS.md

# Log changes made
echo "## Changes Made:" >> bot_instructions/state/IMPLEMENTATION_STATUS.md
echo "- Added ast_builder field to DomExpressionsTransform::new()" >> bot_instructions/state/IMPLEMENTATION_STATUS.md
echo "- Removed unused imports" >> bot_instructions/state/IMPLEMENTATION_STATUS.md
```

### Status Check Commands:
```bash
# Quick progress overview
cat bot_instructions/state/IMPLEMENTATION_STATUS.md | grep -E "(\[x\]|\[ \])"

# Check compilation status
cargo check --quiet && echo "✓ Compiles" || echo "✗ Compilation errors"

# Check test status
cargo test --quiet --lib && echo "✓ Tests pass" || echo "✗ Test failures"
```

## SURGICAL CHANGE PATTERNS

### Pattern 1: Add Missing Field
```bash
replace_in_file path/to/file.rs '
------- SEARCH
struct MyStruct {
    field1: Type1,
    field2: Type2,
}
=======
struct MyStruct {
    field1: Type1,
    field2: Type2,
    missing_field: Type3,
}
+++++++ REPLACE
'
```

### Pattern 2: Fix Constructor
```bash
replace_in_file path/to/file.rs '
------- SEARCH
impl MyStruct {
    pub fn new(param: ParamType) -> Self {
        Self {
            field1: param.something,
        }
    }
}
=======
impl MyStruct {
    pub fn new(param: ParamType) -> Self {
        Self {
            field1: param.something,
            missing_field: Type3::new(),
        }
    }
}
+++++++ REPLACE
'
```

### Pattern 3: Remove Unused Import
```bash
replace_in_file path/to/file.rs '
------- SEARCH
use some_crate::{UsedType, UnusedType, AnotherUsedType};
=======
use some_crate::{UsedType, AnotherUsedType};
+++++++ REPLACE
'
```

## SUCCESS CRITERIA
- [ ] All compilation errors resolved
- [ ] All warnings addressed (or documented as acceptable)
- [ ] Basic tests pass
- [ ] Code follows OXC API patterns from summary
- [ ] Implementation status documented
- [ ] Ready for next development phase

## HANDOFF BACK TO SUMMARY BOT

### When Implementation Bot Needs More Documentation:

If you encounter missing API information or need additional documentation that wasn't covered by the Summary Bot, create a detailed request document:

```bash
# Create comprehensive request for Summary Bot
cat > bot_instructions/state/SUMMARY_BOT_REQUEST.md << 'EOF'
# SUMMARY BOT REQUEST FROM IMPLEMENTATION BOT

## Current Implementation Status
- Date: $(date)
- Last successful action: [describe what you just completed]
- Current blocking issue: [describe what's blocking progress]

## Missing Documentation Needed
1. **OXC API for JSX Attributes**: 
   - How to create JSXAttribute nodes
   - How to handle dynamic vs static attributes
   - Required method signatures and patterns

2. **Error Handling Patterns**:
   - How OXC transformers typically handle errors
   - Result types and error propagation
   - Best practices for error messages

3. **Specific Method Signatures Needed**:
   - JSXElement creation methods
   - AST node manipulation functions
   - Template generation patterns

## What Implementation Bot Will Do Next
When Summary Bot completes the above documentation:

### Phase 1: JSX Attribute Handling
- Implement `transform_jsx_attributes()` method
- Add attribute validation logic
- Handle dynamic attribute binding

### Phase 2: Error Handling Integration  
- Add proper error types to JSXTransformer
- Implement error propagation in transform methods
- Add error recovery mechanisms

### Phase 3: Template Generation
- Complete template creation logic
- Add template caching mechanism
- Implement template injection into AST

## Files Currently Being Modified
- `src/transformer/jsx.rs` - JSX transformation logic
- `src/transformer/mod.rs` - Main transformer struct
- `src/utils/template.rs` - Template generation utilities

## Current Code State
- DomExpressionsTransform struct: [FIXED/NEEDS_WORK]
- JSXTransformer implementation: [IN_PROGRESS]
- Template generation: [NOT_STARTED]
- Compilation status: [COMPILES/HAS_ERRORS]

## Priority Order for Summary Bot
1. HIGH: JSX attribute handling APIs (blocking current implementation)
2. MEDIUM: Error handling patterns (needed for robustness)
3. LOW: Advanced template optimization (can be done later)

## Implementation Bot Memory State
This is my complete understanding of the project when I get turned off:
- Project goal: High-performance JSX transformer for dom-expressions using OXC
- Current issue: Missing OXC API documentation for JSX attribute handling
- Last working code: DomExpressionsTransform constructor now compiles
- Next task: Implement JSX attribute transformation methods
- Expected outcome: Complete JSX transformation pipeline working

EOF
```

### Request User to Switch Bots:
After creating the request document, the Implementation Bot should tell the user:

**"I need additional OXC API documentation to continue. I've created a detailed request in `bot_instructions/state/SUMMARY_BOT_REQUEST.md`. 

Please switch to Summary Bot to gather the missing documentation. The Summary Bot should read this request file first, then gather the requested API information.

After Summary Bot completes the documentation, switch back to Implementation Bot to continue the implementation work."**

### When Resuming After Summary Bot:
1. **Read the original request**: `cat bot_instructions/state/SUMMARY_BOT_REQUEST.md`
2. **Check what was documented**: `cat bot_instructions/state/OXC_API_SUMMARY.md`
3. **Resume from planned next steps** in the request document
4. **Update implementation status** to reflect new progress
