# BOT COMMANDS REFERENCE

## SUMMARY BOT COMMANDS

### State Management
```pwsh
# Check current state
Get-Content bot_instructions\state\SUMMARY_STATE.md

# Initialize new session
if (-not (Test-Path "bot_instructions\state")) { New-Item -ItemType Directory -Path "bot_instructions\state" }
"## Summary Session Started: $(Get-Date)" | Out-File -FilePath "bot_instructions\state\SUMMARY_STATE.md"

# Update progress
"- [x] Completed OXC AstBuilder analysis" | Add-Content bot_instructions\state\SUMMARY_STATE.md
```

### Chunked Reading
```pwsh
# HTML documentation (50-line chunks)
Get-Content target\doc\oxc_ast\struct.AstBuilder.html | Select-Object -First 50
Get-Content target\doc\oxc_ast\struct.AstBuilder.html | Select-Object -Skip 50 -First 50
Get-Content target\doc\oxc_ast\struct.AstBuilder.html | Select-Object -Skip 100 -First 50

# Search for specific patterns
Select-String -Path target\doc\oxc_ast\struct.AstBuilder.html -Pattern "pub fn" -Context 0,3
Select-String -Path target\doc\oxc_ast\struct.AstBuilder.html -Pattern "impl.*Builder" -Context 0,5

# Source code analysis (small chunks)
Select-String -Path src\transformer\*.rs -Pattern "struct.*Transform" -Context 0,10
Select-String -Path src\transformer\mod.rs -Pattern "impl.*Transform" -AllMatches
```

### Error Analysis
```pwsh
# Get compilation errors concisely
cargo check --message-format=short 2>&1 | Select-Object -First 20
cargo check 2>&1 | Select-String -Pattern "error\[E" -Context 0,3

# Find specific error patterns
Select-String -Path . -Pattern "error.*missing.*field" -Include "*.rs" -Recurse
```

### Memory Updates
```pwsh
# Append findings to memory
"## $(Get-Date) OXC AstBuilder Methods Found:" | Add-Content bot_instructions\state\SUMMARY_MEMORY.md
"- ``new(allocator: &'a Allocator) -> AstBuilder<'a>``" | Add-Content bot_instructions\state\SUMMARY_MEMORY.md

# Mark chunk complete
(Get-Content bot_instructions\state\CHUNK_PROGRESS.md) -replace '- \[ \] Lines 1-50', '- [x] Lines 1-50' | Set-Content bot_instructions\state\CHUNK_PROGRESS.md
```

## IMPLEMENTATION BOT COMMANDS

### Startup
```pwsh
# Check handoff from Summary Bot
if (Test-Path "bot_instructions\state\HANDOFF_READY.flag") {
    Remove-Item "bot_instructions\state\HANDOFF_READY.flag"
    Write-Host "Starting implementation..."
}

# Read summary files
Get-Content bot_instructions\state\OXC_API_SUMMARY.md
Get-Content bot_instructions\state\PROJECT_CURRENT_STATE.md
```

### Targeted Fixes
```pwsh
# Fix missing field error using replace_in_file
# (This is a file operation, not PowerShell-specific)
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

### Testing
```pwsh
# Quick compilation check
cargo check --message-format=short

# Run specific tests
cargo test jsx_transformer_tests
cargo test --lib

# Status check
if (cargo check --quiet) { Write-Host "✓ Compiles" } else { Write-Host "✗ Errors" }
```

### Progress Tracking
```pwsh
# Update implementation status
"- [x] Fixed E0063: missing ast_builder field" | Add-Content bot_instructions\state\IMPLEMENTATION_STATUS.md
"## Changes Made:" | Add-Content bot_instructions\state\IMPLEMENTATION_STATUS.md
"- Added ast_builder field initialization" | Add-Content bot_instructions\state\IMPLEMENTATION_STATUS.md
```

## EMERGENCY COMMANDS

### Context Overflow Prevention
```pwsh
# Save current progress immediately
"## EMERGENCY SAVE $(Get-Date)" | Add-Content bot_instructions\state\SUMMARY_MEMORY.md
"Context approaching limit, resuming from chunk X" | Add-Content bot_instructions\state\SUMMARY_STATE.md

# Quick status check
Get-ChildItem bot_instructions\state\*.md | Select-Object Name, Length
Get-ChildItem bot_instructions\state\ | Select-Object Name, LastWriteTime
```

### Bot Handoff
```pwsh
# Summary Bot ready for handoff
"READY FOR IMPLEMENTATION BOT" | Out-File bot_instructions\state\HANDOFF_READY.flag

# Implementation Bot needs more documentation
@"
# SUMMARY BOT REQUEST
## Missing Documentation:
- JSX attribute handling APIs
- Error handling patterns
## Current Implementation State:
- Last working: DomExpressionsTransform constructor
- Next task: JSX attribute transformation
"@ | Out-File bot_instructions\state\SUMMARY_BOT_REQUEST.md
```

## USEFUL SEARCH PATTERNS

### Finding OXC APIs
```pwsh
# Constructor patterns
Select-String -Path target\doc\oxc_*\struct.*.html -Pattern "new.*Allocator" -Context 0,3

# Method signatures
Select-String -Path target\doc\oxc_*\struct.*.html -Pattern "pub fn" -Context 0,2

# Transform patterns
Get-ChildItem target\doc -Recurse -Filter "*.html" | Where-Object {$_.Name -like "*transform*"}
```

### Project Analysis
```pwsh
# Find all structs in transformer
Select-String -Path src\transformer\*.rs -Pattern "struct.*" -AllMatches

# Find all impl blocks
Select-String -Path src\transformer\*.rs -Pattern "impl.*" -AllMatches

# Check dependencies
Select-String -Path src\transformer\*.rs -Pattern "use oxc_" -AllMatches
```

### Error Hunting
```pwsh
# Find specific error codes
Select-String -Path . -Pattern "E[0-9]{4}" -Include "*.rs" -Recurse

# Find missing field errors
Select-String -Path . -Pattern "missing field" -Include "*.rs" -Recurse

# Find unused import warnings
Select-String -Path . -Pattern "unused.*import" -Include "*.rs" -Recurse
```

## FILE SIZE MONITORING

### Check before reading
```pwsh
# Check file size before reading
(Get-Content target\doc\oxc_ast\struct.AstBuilder.html).Count

# Count lines in chunks
(Get-Content file.html | Select-Object -First 50).Count
```

### Memory file maintenance
```pwsh
# Keep memory files manageable
Get-Content bot_instructions\state\SUMMARY_MEMORY.md | Select-Object -Last 100 | Set-Content temp.md
Move-Item temp.md bot_instructions\state\SUMMARY_MEMORY.md

# Check total state size
Get-ChildItem bot_instructions\state\ | Measure-Object -Property Length -Sum
