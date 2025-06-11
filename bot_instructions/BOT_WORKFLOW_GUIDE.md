# BOT WORKFLOW GUIDE

## OVERVIEW
This guide explains how to use the Summary Bot and Implementation Bot in sequence to work on the OXC JSX transformer project without context overflow issues.

## WORKFLOW PHASES

### Phase 1: SUMMARY BOT
**Goal**: Extract OXC API information and analyze current project state

#### When to Use Summary Bot:
- Starting a new development session
- Need to understand OXC APIs
- Analyze compilation errors
- Implementation Bot requests more documentation

#### Summary Bot Process:
1. **Startup**: Check for existing state, initialize if needed
2. **Chunked Reading**: Process large files in small chunks (50-100 lines)
3. **API Extraction**: Find OXC method signatures and patterns
4. **Error Analysis**: Get concise compilation error information
5. **Documentation**: Create small summary files for Implementation Bot
6. **Handoff**: Signal completion with HANDOFF_READY.flag

#### Summary Bot Deliverables:
- `OXC_API_SUMMARY.md` - Essential API patterns and usage
- `PROJECT_CURRENT_STATE.md` - Compilation errors and issues
- `IMPLEMENTATION_ROADMAP.md` - Prioritized tasks
- `SUMMARY_MEMORY.md` - Complete session memory

### Phase 2: IMPLEMENTATION BOT  
**Goal**: Fix compilation errors and implement features using only summary files

#### When to Use Implementation Bot:
- After Summary Bot completes documentation
- Fix specific compilation errors
- Implement new features with known APIs
- Clean up code and run tests

#### Implementation Bot Process:
1. **Startup**: Verify Summary Bot completion, read summary files only
2. **Error Fixing**: Make targeted fixes based on summary information
3. **Feature Implementation**: Add new methods using OXC API patterns
4. **Testing**: Verify compilation and run tests
5. **Progress Tracking**: Update status files
6. **Handoff**: Request more documentation if needed

## DETAILED WORKFLOW

### Starting a New Session

#### Step 1: Run Summary Bot
```bash
# User message: "I need to start working on the OXC transformer. Please run as Summary Bot."

# Summary Bot actions:
cat bot_instructions/state/SUMMARY_STATE.md  # Check existing state
cargo check --message-format=short           # Get current errors
head -n 50 target/doc/oxc_ast/struct.AstBuilder.html  # Start API analysis
```

#### Step 2: Summary Bot Completion
Summary Bot will create:
- Small documentation files (< 2KB each)
- Progress tracking files
- HANDOFF_READY.flag when complete

#### Step 3: Switch to Implementation Bot
```bash
# User switches bots
# Implementation Bot starts with:
cat bot_instructions/state/OXC_API_SUMMARY.md      # Read API docs
cat bot_instructions/state/PROJECT_CURRENT_STATE.md # Read errors
# Then makes targeted fixes
```

### Handling Context Overflow

#### If Summary Bot Hits Context Limit:
```bash
# Emergency save current progress
echo "## EMERGENCY SAVE [$(date)]" >> bot_instructions/state/SUMMARY_MEMORY.md
echo "Context approaching limit, resuming from chunk 5" >> bot_instructions/state/SUMMARY_STATE.md

# User gets message: "Context limit approaching. Progress saved. Please restart Summary Bot."
```

#### Recovery Process:
```bash
# Restarted Summary Bot reads state first:
cat bot_instructions/state/SUMMARY_STATE.md
tail -20 bot_instructions/state/SUMMARY_MEMORY.md
# Then resumes from last incomplete chunk
```

### Implementation Bot Requesting More Documentation

#### When Implementation Bot Needs Help:
```bash
# Implementation Bot creates detailed request:
cat > bot_instructions/state/SUMMARY_BOT_REQUEST.md << 'EOF'
# SUMMARY BOT REQUEST FROM IMPLEMENTATION BOT

## Missing Documentation Needed:
- JSX attribute handling APIs
- Error handling patterns

## Current Implementation State:
- Last working: DomExpressionsTransform constructor fixed
- Current blocking issue: Don't know how to create JSXAttribute nodes
- Next planned task: Implement transform_jsx_attributes() method

## What Implementation Bot Will Do Next:
When you provide the JSX attribute APIs, I will:
1. Implement transform_jsx_attributes() method
2. Add attribute validation logic  
3. Handle dynamic vs static attributes
4. Test the implementation

EOF

# Then tells user:
# "I need additional OXC API documentation. Please switch to Summary Bot to gather JSX attribute handling APIs."
```

#### Summary Bot Handling Request:
```bash
# Summary Bot reads the request first:
cat bot_instructions/state/SUMMARY_BOT_REQUEST.md

# Then focuses on the specific missing documentation:
grep -A 5 "JSXAttribute" target/doc/oxc_ast/*.html
grep -A 3 "attribute" target/doc/oxc_ast/*.html

# Updates API summary with new findings:
echo "## JSX Attribute APIs" >> bot_instructions/state/OXC_API_SUMMARY.md
echo "- JSXAttribute::new(name, value)" >> bot_instructions/state/OXC_API_SUMMARY.md
```

## BOT COMMUNICATION PROTOCOL

### Summary Bot → Implementation Bot
**Files**: 
- `OXC_API_SUMMARY.md` - How to use OXC APIs
- `PROJECT_CURRENT_STATE.md` - What needs to be fixed
- `IMPLEMENTATION_ROADMAP.md` - Task priorities
- `HANDOFF_READY.flag` - Signal completion

**Message**: "OXC API documentation complete. Ready for Implementation Bot."

### Implementation Bot → Summary Bot  
**Files**:
- `SUMMARY_BOT_REQUEST.md` - Detailed request for missing documentation
- `IMPLEMENTATION_STATUS.md` - Current progress and blocking issues

**Message**: "Need additional documentation. Please switch to Summary Bot."

## SUCCESS CRITERIA

### Summary Bot Success:
- [ ] OXC API patterns documented
- [ ] Current compilation errors identified
- [ ] Implementation roadmap created
- [ ] All deliverable files < 2KB
- [ ] HANDOFF_READY.flag created
- [ ] No context overflow during session

### Implementation Bot Success:
- [ ] All compilation errors resolved
- [ ] New features implemented using documented APIs
- [ ] Code changes tested and verified
- [ ] Progress documented in status files
- [ ] Ready for next development cycle

## TROUBLESHOOTING

### Summary Bot Issues:
**Problem**: Context overflow during documentation reading
**Solution**: Use smaller chunks (25-30 lines instead of 50)

**Problem**: Can't find specific OXC APIs
**Solution**: Use targeted searches instead of reading full files
```bash
grep -A 3 "specific_method" target/doc/oxc_*/*.html
```

### Implementation Bot Issues:
**Problem**: Missing API information in summary files
**Solution**: Create detailed SUMMARY_BOT_REQUEST.md and switch bots

**Problem**: Compilation errors after changes
**Solution**: Use cargo check --message-format=short for concise errors

### Cross-Bot Issues:
**Problem**: State files getting too large
**Solution**: Archive old sessions, keep only recent memory
```bash
mv bot_instructions/state/SUMMARY_MEMORY.md bot_instructions/archive/session_$(date +%Y%m%d).md
```

## BEST PRACTICES

### For Summary Bot:
1. Always check existing state before starting
2. Use chunked reading, never read full large files
3. Focus on essential API patterns, not implementation details
4. Keep deliverable files small and focused
5. Save progress frequently to prevent loss

### For Implementation Bot:
1. Never read source files directly
2. Make minimal, targeted changes
3. Test after each change
4. Document all changes in status files
5. Request help when missing information

### For Both Bots:
1. Update progress files after major actions
2. Use consistent file naming and formats
3. Keep context usage below 80% when possible
4. Focus on one task at a time
5. Maintain clean separation of concerns
