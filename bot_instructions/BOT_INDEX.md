# BOT INSTRUCTION INDEX

## 🤖 BOT ROLE ASSIGNMENT

### If you are told "You are Summary Bot":
**READ THIS FILE**: `bot_instructions/SUMMARY_BOT_INSTRUCTIONS.md`
- Your role: Analyze OXC documentation and project state using chunked reading
- Never read large files completely
- Create small summary files for Implementation Bot
- Use PowerShell commands compatible with Windows

### If you are told "You are Implementation Bot":
**READ THIS FILE**: `bot_instructions/IMPLEMENTATION_BOT_INSTRUCTIONS.md`
- Your role: Fix compilation errors and implement features
- Only read summary files created by Summary Bot
- Make surgical code changes using replace_in_file
- Test after each change

## 📋 QUICK REFERENCE

### Summary Bot Commands (PowerShell Core):
```pwsh
# Check state
Get-Content bot_instructions\state\SUMMARY_STATE.md

# Chunked reading
Get-Content target\doc\oxc_ast\struct.AstBuilder.html | Select-Object -First 50
Get-Content target\doc\oxc_ast\struct.AstBuilder.html | Select-Object -Skip 50 -First 50

# Error analysis  
cargo check --message-format=short | Select-Object -First 20
```

### Implementation Bot Commands (PowerShell Core):
```pwsh
# Read summaries only
Get-Content bot_instructions\state\OXC_API_SUMMARY.md
Get-Content bot_instructions\state\PROJECT_CURRENT_STATE.md

# Test compilation
cargo check --message-format=short
cargo test --lib
```

## 🔧 TOOL USAGE
**IMPORTANT**: For proper XML tool syntax, both bots must first read `bot_instructions/CLINE_INSTRUCTIONS.md`

## 🔄 BOT HANDOFF PROTOCOL

### Summary Bot → Implementation Bot:
1. Summary Bot creates deliverable files
2. Summary Bot creates `bot_instructions\state\HANDOFF_READY.flag`
3. User says: **"You are Implementation Bot"**
4. Implementation Bot reads `bot_instructions\IMPLEMENTATION_BOT_INSTRUCTIONS.md`

### Implementation Bot → Summary Bot:
1. Implementation Bot creates `bot_instructions\state\SUMMARY_BOT_REQUEST.md`
2. Implementation Bot tells user: **"Please switch to Summary Bot"**  
3. User says: **"You are Summary Bot"**
4. Summary Bot reads `bot_instructions\SUMMARY_BOT_INSTRUCTIONS.md`
5. Summary Bot reads the request file first

## 📁 STATE FILES LOCATION
All state files are in: `bot_instructions\state\`
- Progress tracking
- API documentation
- Error analysis
- Communication between bots

## 🆘 EMERGENCY COMMANDS
```pwsh
# Check what files exist
Get-ChildItem bot_instructions\state\

# File sizes
Get-ChildItem bot_instructions\state\ | Select-Object Name, Length

# Quick status
if (Test-Path "bot_instructions\state\HANDOFF_READY.flag") { "Implementation Bot Ready" } else { "Summary Bot Working" }
```

## 🎯 CURRENT PROJECT STATUS
- **Compilation Error**: Missing `ast_builder` field in DomExpressionsTransform
- **Goal**: High-performance JSX transformer for dom-expressions using OXC
- **Next Step**: Summary Bot analyze OXC APIs, then Implementation Bot fix error

---
**IMPORTANT**: Always read your specific instruction file based on your assigned role!
