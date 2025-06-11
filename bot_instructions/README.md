# BOT INSTRUCTIONS SYSTEM

## OVERVIEW
This directory contains instructions for using specialized AI bots to work on the OXC JSX transformer project without context overflow issues.

## QUICK START

### 1. Start with Summary Bot
Tell the AI: **"Please follow the Summary Bot instructions to analyze the OXC documentation and current project state."**

### 2. Switch to Implementation Bot  
After Summary Bot completes: **"Please follow the Implementation Bot instructions to fix the compilation errors."**

### 3. Request More Documentation (if needed)
If Implementation Bot needs more info: **"Please create a Summary Bot request and switch back to Summary Bot."**

## FILES IN THIS DIRECTORY

### Main Instructions
- **`SUMMARY_BOT_INSTRUCTIONS.md`** - Complete instructions for the Summary Bot
- **`IMPLEMENTATION_BOT_INSTRUCTIONS.md`** - Complete instructions for the Implementation Bot
- **`BOT_WORKFLOW_GUIDE.md`** - How to use both bots together
- **`BOT_COMMANDS_REFERENCE.md`** - Quick reference of useful commands

### State Management
The `bot_instructions/state/` directory (created automatically) contains:
- **`SUMMARY_STATE.md`** - Current Summary Bot progress
- **`SUMMARY_MEMORY.md`** - Accumulated OXC API knowledge
- **`OXC_API_SUMMARY.md`** - Essential API patterns (Summary Bot → Implementation Bot)
- **`PROJECT_CURRENT_STATE.md`** - Compilation errors and issues  
- **`IMPLEMENTATION_STATUS.md`** - Implementation Bot progress
- **`SUMMARY_BOT_REQUEST.md`** - Implementation Bot → Summary Bot requests

## KEY FEATURES

### Context Overflow Prevention
- **Chunked Reading**: Never read large files completely
- **Persistent State**: Resumable across AI resets
- **Small Deliverables**: All summary files < 2KB
- **Targeted Operations**: Surgical code changes only

### Bot Specialization
- **Summary Bot**: OXC documentation analysis, error identification
- **Implementation Bot**: Code fixes, feature implementation, testing
- **Clear Separation**: No overlap in responsibilities

### Communication Protocol
- **File-based Handoffs**: Clear deliverable specifications
- **Progress Tracking**: Resumable workflows
- **Request System**: Implementation Bot can ask for more documentation

## CURRENT PROJECT STATUS

### Known Issues (as of creation):
1. **Compilation Error E0063**: Missing `ast_builder` field in `DomExpressionsTransform::new()`
2. **Unused Imports**: Multiple unused imports causing warnings
3. **Missing Implementation**: JSX transformation logic incomplete

### Next Steps:
1. Run Summary Bot to analyze OXC AstBuilder APIs
2. Run Implementation Bot to fix the missing field error
3. Continue with JSX transformation implementation

## BOT USAGE EXAMPLES

### Starting Fresh Session
```
User: "I need to work on the OXC transformer. Please act as Summary Bot and analyze the current state."

Summary Bot will:
1. Check for existing state files
2. Analyze compilation errors with cargo check
3. Extract OXC API documentation in chunks
4. Create summary files for Implementation Bot
5. Signal completion with HANDOFF_READY.flag
```

### Fixing Compilation Errors
```
User: "Summary Bot is complete. Please act as Implementation Bot and fix the compilation errors."

Implementation Bot will:
1. Read only the summary files (never source code directly)
2. Make targeted fixes based on summary information
3. Test after each change
4. Update progress tracking
5. Request more documentation if needed
```

### Requesting More Documentation
```
Implementation Bot: "I need additional OXC API documentation for JSX attributes. I've created bot_instructions/state/SUMMARY_BOT_REQUEST.md. Please switch to Summary Bot."

User switches to Summary Bot, which will:
1. Read the specific request
2. Focus on the missing documentation
3. Update API summaries
4. Signal completion for Implementation Bot to resume
```

## TROUBLESHOOTING

### If AI Gets Confused
- **Always start with**: "Please read bot_instructions/[SUMMARY_BOT|IMPLEMENTATION_BOT]_INSTRUCTIONS.md"
- **Check state**: "Please check bot_instructions/state/ for existing progress"
- **Emergency reset**: Delete state files and start fresh

### If Context Overflow Occurs
- **Summary Bot**: Will save progress and request restart
- **Implementation Bot**: Will create detailed request for Summary Bot
- **Recovery**: State files enable resuming from last position

### If Compilation Fails
- **Never read source files directly** 
- **Use cargo check --message-format=short** for concise errors
- **Make minimal changes** based on summary information only

## SUCCESS METRICS

### Summary Bot Success:
- [ ] OXC API patterns documented
- [ ] Compilation errors identified  
- [ ] Implementation roadmap created
- [ ] All files < 2KB
- [ ] No context overflow

### Implementation Bot Success:
- [ ] Compilation errors fixed
- [ ] Features implemented
- [ ] Tests passing
- [ ] Progress documented
- [ ] Ready for next cycle

## MAINTENANCE

### Cleaning Up State Files
```bash
# Archive old sessions
mkdir -p bot_instructions/archive
mv bot_instructions/state/SUMMARY_MEMORY.md bot_instructions/archive/session_$(date +%Y%m%d).md

# Start fresh
rm -rf bot_instructions/state/*
```

### Updating Instructions
When updating bot instructions:
1. Update the relevant instruction file
2. Test with a simple task
3. Update this README if workflow changes
4. Document any new patterns in BOT_COMMANDS_REFERENCE.md

## NOTES FOR DEVELOPERS

This bot instruction system is designed to:
- **Prevent AI context overflow** during large project analysis
- **Maintain continuity** across AI resets and sessions  
- **Enable specialized workflows** for different types of tasks
- **Provide clear handoff protocols** between different AI capabilities

The key insight is that modern AI systems work better with:
- **Clear, specific instructions** rather than general guidance
- **Persistent state management** to handle context limitations
- **Specialized roles** rather than trying to do everything at once
- **File-based communication** for complex handoffs

This approach should work for any large Rust project that needs both documentation analysis and implementation work.
