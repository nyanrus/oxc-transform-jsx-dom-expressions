# SUMMARY BOT INSTRUCTIONS

## MISSION
Extract OXC API information and project state using chunked reading to prevent context overflow and enable resumable workflows across AI resets.

## CORE PRINCIPLES
1. **NEVER** read full large files directly
2. **ALWAYS** use chunked reading with shell commands
3. **MAINTAIN** persistent state across sessions
4. **UPDATE** memory files incrementally
5. **ENABLE** resumable workflows after AI resets

## TOOL USAGE INSTRUCTIONS

**IMPORTANT**: For proper tool usage and XML formatting, read `bot_instructions/CLINE_INSTRUCTIONS.md` first.

Use XML-style tool formatting for all operations. The tools you'll primarily use:
- `execute_command` - For PowerShell commands and chunked reading
- `read_file` - Only for small summary files 
- `write_to_file` - For creating deliverable summary files
- `replace_in_file` - For updating state files

## STARTUP PROTOCOL

### Step 1: Check for Implementation Bot Request
Always check first if Implementation Bot has requested more documentation.

### Step 2: Read current state and initialize if needed

### Step 3: Process files in small chunks using PowerShell commands

## CHUNKED READING STRATEGY

### Configuration
- **Chunk Size**: 50-100 lines maximum
- **Context Window**: Monitor token usage, stop at 80% capacity
- **Memory Updates**: After each chunk, append findings to memory file

### PowerShell Core (pwsh) Chunking Commands

Use `pwsh` shell for all PowerShell commands:

#### For HTML Documentation:
```pwsh
# Read first chunk
Get-Content target\doc\oxc_ast\struct.AstBuilder.html | Select-Object -First 50

# Read subsequent chunks
Get-Content target\doc\oxc_ast\struct.AstBuilder.html | Select-Object -Skip 50 -First 50
Get-Content target\doc\oxc_ast\struct.AstBuilder.html | Select-Object -Skip 100 -First 50

# Search for specific patterns
Select-String -Path target\doc\oxc_ast\struct.AstBuilder.html -Pattern "pub fn" -Context 1,3
Select-String -Path target\doc\oxc_ast\struct.AstBuilder.html -Pattern "impl.*Builder" -Context 0,5
```

#### For Source Code (when needed):
```pwsh
# Small chunks with line numbers
Get-Content src\transformer\mod.rs | Select-Object -First 30 | ForEach-Object -Begin {$i=1} -Process {"$i`: $_"; $i++}
Get-Content src\transformer\mod.rs | Select-Object -Skip 30 -First 30 | ForEach-Object -Begin {$i=31} -Process {"$i`: $_"; $i++}

# Extract specific sections
Select-String -Path src\transformer\*.rs -Pattern "struct.*Transform" -Context 0,10
Select-String -Path src\transformer\*.rs -Pattern "impl.*Transform" -Context 0,20
```

#### For Compilation Errors:
```pwsh
# Get concise error information
cargo check --message-format=short 2>&1 | Select-Object -First 20
cargo check 2>&1 | Select-String -Pattern "error\[E" -Context 0,3
```

## STATE MANAGEMENT

### Progress Tracking
Update `bot_instructions/state/SUMMARY_STATE.md` after each major chunk

### Memory Accumulation
Always APPEND to `bot_instructions/state/SUMMARY_MEMORY.md`

### Chunk Progress Tracking
Update `bot_instructions/state/CHUNK_PROGRESS.md`

## OUTPUT DELIVERABLES

### For Implementation Bot:
1. `OXC_API_SUMMARY.md` - Essential API patterns and usage
2. `PROJECT_CURRENT_STATE.md` - Compilation errors and missing pieces
3. `IMPLEMENTATION_ROADMAP.md` - Prioritized tasks for fixing issues

### File Size Limits:
- Each deliverable file: < 2KB
- Focus on essential information only
- Use bullet points and code snippets
- Include specific line numbers and file references

## SUCCESS CRITERIA
- [ ] OXC AstBuilder API fully documented
- [ ] Transform patterns identified
- [ ] Current compilation errors catalogued
- [ ] Implementation roadmap created
- [ ] All state files updated for resumability
- [ ] Deliverable files ready for Implementation Bot
