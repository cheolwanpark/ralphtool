# OpenSpec-Ralph Implementation Guide

This document provides the complete implementation reference for integrating OpenSpec with the Ralph autonomous agent loop.

---

## Table of Contents

1. [Ralph Loop Reference](#ralph-loop-reference)
2. [OpenSpec CLI Commands](#openspec-cli-commands)
3. [Orchestrator Script Design](#orchestrator-script-design)
4. [Workflow Examples](#workflow-examples)
5. [File Format Specifications](#file-format-specifications)

---

## Ralph Loop Reference

This section documents the original Ralph loop for reference (the `ralph/` directory will be removed).

### Core Concept

Ralph is an autonomous AI agent loop that:
1. Spawns fresh AI instances (Amp or Claude Code) iteratively
2. Each instance has clean context (no memory of previous iterations)
3. Memory persists through: git commits, `progress.txt`, `prd.json`
4. Loops until all tasks complete or max iterations reached

### Original Architecture

```
ralph.sh (orchestrator)
    │
    ├─ Check/create git branch from prd.json.branchName
    ├─ Archive previous run if branch changed
    │
    └─ For each iteration (1 to MAX_ITERATIONS):
        │
        ├─ Spawn AI tool with prompt.md (Amp) or CLAUDE.md (Claude)
        │
        ├─ AI reads prd.json
        │   └─ Finds first story where passes=false
        │
        ├─ AI reads progress.txt
        │   └─ Checks Codebase Patterns section
        │
        ├─ AI implements ONE story
        │   └─ Runs quality checks (typecheck, tests)
        │
        ├─ AI commits changes
        │   └─ Message: "feat: [Story ID] - [Title]"
        │
        ├─ AI updates prd.json
        │   └─ Sets passes=true for completed story
        │
        ├─ AI appends to progress.txt
        │   └─ Learnings, gotchas, patterns
        │
        └─ Check for <promise>COMPLETE</promise>
            └─ If found: exit 0
            └─ If not: continue to next iteration
```

### Original ralph.sh Script

```bash
#!/bin/bash
# Ralph Wiggum - Long-running AI agent loop
# Usage: ./ralph.sh [--tool amp|claude] [max_iterations]

set -e

# Parse arguments
TOOL="amp"
MAX_ITERATIONS=10

while [[ $# -gt 0 ]]; do
  case $1 in
    --tool)
      TOOL="$2"
      shift 2
      ;;
    --tool=*)
      TOOL="${1#*=}"
      shift
      ;;
    *)
      if [[ "$1" =~ ^[0-9]+$ ]]; then
        MAX_ITERATIONS="$1"
      fi
      shift
      ;;
  esac
done

if [[ "$TOOL" != "amp" && "$TOOL" != "claude" ]]; then
  echo "Error: Invalid tool '$TOOL'. Must be 'amp' or 'claude'."
  exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PRD_FILE="$SCRIPT_DIR/prd.json"
PROGRESS_FILE="$SCRIPT_DIR/progress.txt"

# Initialize progress file if missing
if [ ! -f "$PROGRESS_FILE" ]; then
  echo "# Ralph Progress Log" > "$PROGRESS_FILE"
  echo "Started: $(date)" >> "$PROGRESS_FILE"
  echo "---" >> "$PROGRESS_FILE"
fi

echo "Starting Ralph - Tool: $TOOL - Max iterations: $MAX_ITERATIONS"

for i in $(seq 1 $MAX_ITERATIONS); do
  echo ""
  echo "==============================================================="
  echo "  Ralph Iteration $i of $MAX_ITERATIONS ($TOOL)"
  echo "==============================================================="

  if [[ "$TOOL" == "amp" ]]; then
    OUTPUT=$(cat "$SCRIPT_DIR/prompt.md" | amp --dangerously-allow-all 2>&1 | tee /dev/stderr) || true
  else
    OUTPUT=$(claude --dangerously-skip-permissions --print < "$SCRIPT_DIR/CLAUDE.md" 2>&1 | tee /dev/stderr) || true
  fi

  if echo "$OUTPUT" | grep -q "<promise>COMPLETE</promise>"; then
    echo ""
    echo "Ralph completed all tasks!"
    exit 0
  fi

  sleep 2
done

echo "Ralph reached max iterations without completing."
exit 1
```

### Original prd.json Format

```json
{
  "project": "ProjectName",
  "branchName": "ralph/feature-name",
  "description": "Feature description",
  "userStories": [
    {
      "id": "US-001",
      "title": "Story title",
      "description": "As a [user], I want [feature] so that [benefit]",
      "acceptanceCriteria": [
        "Criterion 1",
        "Criterion 2",
        "Typecheck passes"
      ],
      "priority": 1,
      "passes": false,
      "notes": ""
    }
  ]
}
```

### Original AI Instructions (prompt.md / CLAUDE.md)

The AI is instructed to:

1. Read `prd.json` for task list
2. Read `progress.txt` for context (check Codebase Patterns first)
3. Verify correct git branch
4. Pick highest priority story where `passes: false`
5. Implement that ONE story
6. Run quality checks
7. Update AGENTS.md/CLAUDE.md with patterns
8. Commit with message: `feat: [Story ID] - [Story Title]`
9. Update `prd.json` to set `passes: true`
10. Append progress to `progress.txt`
11. If ALL stories pass, output `<promise>COMPLETE</promise>`

---

## OpenSpec CLI Commands

### Initialization

```bash
# Initialize OpenSpec in project
openspec init

# Configure for Claude Code
openspec init --tools claude

# Configure for multiple tools
openspec init --tools claude,cursor
```

This creates:
```
openspec/
├── specs/           # Source of truth specifications
├── changes/         # Active changes
└── config.yaml      # Project configuration
```

### Creating a Change

```bash
# Interactive creation
openspec new add-dark-mode

# Or use the AI skill
/opsx:new add-dark-mode
```

This creates:
```
openspec/changes/add-dark-mode/
├── .openspec.yaml   # Change metadata (optional)
└── (artifacts created via continue/ff)
```

### Creating Artifacts

**Step-by-step:**
```bash
# Create next artifact based on schema dependencies
/opsx:continue add-dark-mode
```

**All at once:**
```bash
# Create all artifacts: proposal → specs → design → tasks
/opsx:ff add-dark-mode
```

### Checking Status

```bash
# Human-readable status
openspec status --change add-dark-mode

# JSON for scripts/agents
openspec status --change add-dark-mode --json
```

Output:
```
Change: add-dark-mode
Schema: spec-driven

Artifacts:
  ✓ proposal     proposal.md exists
  ✓ specs        specs/ exists
  ◆ design       ready (requires: specs)
  ○ tasks        blocked (requires: design)

Next: Create design using /opsx:continue
```

### Getting Instructions (for AI agent)

```bash
# Get context for implementing tasks
openspec instructions apply --change add-dark-mode --json
```

Returns structured JSON with:
- Proposal content
- Design decisions
- Remaining tasks
- Delta specs to implement
- Project context

### Validation

```bash
# Validate specific change
openspec validate add-dark-mode

# Validate all changes
openspec validate --changes

# Validate with JSON output
openspec validate --all --json
```

### Listing Changes/Specs

```bash
# List active changes
openspec list

# List specs
openspec list --specs

# JSON output
openspec list --json
```

### Viewing Content

```bash
# View change details
openspec show add-dark-mode

# View with JSON
openspec show add-dark-mode --json

# View only delta specs
openspec show add-dark-mode --deltas-only --json
```

### Archiving

```bash
# Interactive archive
openspec archive

# Archive specific change
openspec archive add-dark-mode

# Skip confirmation (for scripts)
openspec archive add-dark-mode --yes

# Skip spec updates (for non-spec changes)
openspec archive update-ci-config --skip-specs
```

---

## Orchestrator Script Design

### New Script: openspec-ralph

```bash
#!/bin/bash
# openspec-ralph - Ralph loop using OpenSpec for task management
# Usage: openspec-ralph <change-name> [--tool amp|claude] [max_iterations]

set -e

# ============================================================
# ARGUMENT PARSING
# ============================================================

CHANGE_NAME=""
TOOL="claude"
MAX_ITERATIONS=50

while [[ $# -gt 0 ]]; do
  case $1 in
    --tool)
      TOOL="$2"
      shift 2
      ;;
    --tool=*)
      TOOL="${1#*=}"
      shift
      ;;
    -*)
      echo "Unknown option: $1"
      exit 1
      ;;
    *)
      if [[ -z "$CHANGE_NAME" ]]; then
        CHANGE_NAME="$1"
      elif [[ "$1" =~ ^[0-9]+$ ]]; then
        MAX_ITERATIONS="$1"
      fi
      shift
      ;;
  esac
done

if [[ -z "$CHANGE_NAME" ]]; then
  echo "Usage: openspec-ralph <change-name> [--tool amp|claude] [max_iterations]"
  echo ""
  echo "Active changes:"
  openspec list 2>/dev/null || echo "  (none)"
  exit 1
fi

if [[ "$TOOL" != "amp" && "$TOOL" != "claude" ]]; then
  echo "Error: Invalid tool '$TOOL'. Must be 'amp' or 'claude'."
  exit 1
fi

# ============================================================
# PATHS AND VALIDATION
# ============================================================

CHANGE_DIR="openspec/changes/$CHANGE_NAME"
TASKS_FILE="$CHANGE_DIR/tasks.md"
DESIGN_FILE="$CHANGE_DIR/design.md"

if [[ ! -d "$CHANGE_DIR" ]]; then
  echo "Error: Change '$CHANGE_NAME' not found."
  echo "Create it with: /opsx:new $CHANGE_NAME"
  exit 1
fi

if [[ ! -f "$TASKS_FILE" ]]; then
  echo "Error: tasks.md not found in $CHANGE_DIR"
  echo "Create artifacts with: /opsx:ff $CHANGE_NAME"
  exit 1
fi

# ============================================================
# GIT BRANCH SETUP
# ============================================================

BRANCH_NAME="ralph/$CHANGE_NAME"
CURRENT_BRANCH=$(git branch --show-current)

if [[ "$CURRENT_BRANCH" != "$BRANCH_NAME" ]]; then
  echo "Switching to branch: $BRANCH_NAME"
  if git show-ref --verify --quiet "refs/heads/$BRANCH_NAME"; then
    git checkout "$BRANCH_NAME"
  else
    git checkout -b "$BRANCH_NAME"
  fi
fi

# ============================================================
# HELPER FUNCTIONS
# ============================================================

get_next_task() {
  # Find first unchecked task (- [ ] pattern)
  # Returns: "LINE_NUM|TASK_TEXT" or empty if none
  grep -n "^[[:space:]]*- \[ \]" "$TASKS_FILE" | head -1 | sed 's/:/ |/'
}

count_remaining_tasks() {
  grep -c "^[[:space:]]*- \[ \]" "$TASKS_FILE" 2>/dev/null || echo "0"
}

count_completed_tasks() {
  grep -c "^[[:space:]]*- \[x\]" "$TASKS_FILE" 2>/dev/null || echo "0"
}

get_current_story() {
  # Find the story heading (### ) above the current task
  local line_num=$1
  head -n "$line_num" "$TASKS_FILE" | grep "^### " | tail -1 | sed 's/^### //'
}

get_current_epic() {
  # Find the epic heading (## ) above the current task
  local line_num=$1
  head -n "$line_num" "$TASKS_FILE" | grep "^## " | tail -1 | sed 's/^## //'
}

# ============================================================
# GENERATE AI PROMPT
# ============================================================

generate_prompt() {
  local task_text="$1"
  local task_line="$2"
  local story=$(get_current_story "$task_line")
  local epic=$(get_current_epic "$task_line")

  # Get OpenSpec instructions
  local instructions=$(openspec instructions apply --change "$CHANGE_NAME" --json 2>/dev/null || echo "{}")

  cat << EOF
# OpenSpec-Ralph Agent Instructions

You are an autonomous coding agent working on change: **$CHANGE_NAME**

## Current Context

- **Epic:** $epic
- **Story:** $story
- **Task:** $task_text

## Your Job

1. **Read the change artifacts:**
   - Proposal: \`$CHANGE_DIR/proposal.md\`
   - Design: \`$CHANGE_DIR/design.md\`
   - Delta specs: \`$CHANGE_DIR/specs/\`

2. **Implement ONLY this one task:**
   \`$task_text\`

3. **Run quality checks:**
   - Typecheck (e.g., \`npm run typecheck\` or \`tsc --noEmit\`)
   - Tests (e.g., \`npm test\`)
   - Lint (e.g., \`npm run lint\`)

4. **If checks pass:**
   - Commit changes with message: \`feat($CHANGE_NAME): $task_text\`
   - Mark the task complete in \`$TASKS_FILE\` by changing \`- [ ]\` to \`- [x]\`

5. **Update learnings:**
   - If you discovered patterns or gotchas, append to the \`## Learnings\` section in \`$DESIGN_FILE\`

6. **Verify completion:**
   - After marking the task done, check if ALL tasks in \`$TASKS_FILE\` are complete
   - If ALL tasks are \`- [x]\`, respond with: \`<promise>COMPLETE</promise>\`
   - If tasks remain, end normally (next iteration will pick up the next task)

## Important Rules

- Work on ONE task only
- Do NOT skip quality checks
- Do NOT commit broken code
- Keep changes focused and minimal
- Follow existing code patterns

## OpenSpec Context

\`\`\`json
$instructions
\`\`\`
EOF
}

# ============================================================
# MAIN LOOP
# ============================================================

echo ""
echo "============================================================"
echo "  OpenSpec-Ralph: $CHANGE_NAME"
echo "  Tool: $TOOL | Max iterations: $MAX_ITERATIONS"
echo "============================================================"
echo ""

COMPLETED=$(count_completed_tasks)
REMAINING=$(count_remaining_tasks)
echo "Progress: $COMPLETED completed, $REMAINING remaining"
echo ""

for i in $(seq 1 $MAX_ITERATIONS); do
  echo ""
  echo "------------------------------------------------------------"
  echo "  Iteration $i of $MAX_ITERATIONS"
  echo "------------------------------------------------------------"

  # Get next task
  NEXT_TASK=$(get_next_task)

  if [[ -z "$NEXT_TASK" ]]; then
    echo ""
    echo "All tasks complete!"
    echo ""
    echo "Next steps:"
    echo "  1. openspec validate $CHANGE_NAME"
    echo "  2. openspec archive $CHANGE_NAME"
    exit 0
  fi

  TASK_LINE=$(echo "$NEXT_TASK" | cut -d'|' -f1 | tr -d ' ')
  TASK_TEXT=$(echo "$NEXT_TASK" | cut -d'|' -f2- | sed 's/^[[:space:]]*- \[ \] //')

  echo "Task: $TASK_TEXT"
  echo ""

  # Generate prompt
  PROMPT=$(generate_prompt "$TASK_TEXT" "$TASK_LINE")

  # Run AI tool
  if [[ "$TOOL" == "amp" ]]; then
    OUTPUT=$(echo "$PROMPT" | amp --dangerously-allow-all 2>&1 | tee /dev/stderr) || true
  else
    OUTPUT=$(echo "$PROMPT" | claude --dangerously-skip-permissions --print 2>&1 | tee /dev/stderr) || true
  fi

  # Check for completion
  if echo "$OUTPUT" | grep -q "<promise>COMPLETE</promise>"; then
    echo ""
    echo "============================================================"
    echo "  All tasks complete!"
    echo "============================================================"
    echo ""
    echo "Next steps:"
    echo "  1. openspec validate $CHANGE_NAME"
    echo "  2. openspec archive $CHANGE_NAME"
    exit 0
  fi

  # Show progress
  COMPLETED=$(count_completed_tasks)
  REMAINING=$(count_remaining_tasks)
  echo ""
  echo "Progress: $COMPLETED completed, $REMAINING remaining"

  sleep 2
done

echo ""
echo "Max iterations ($MAX_ITERATIONS) reached."
echo "Remaining tasks: $(count_remaining_tasks)"
exit 1
```

### Script Features

| Feature | Description |
|---------|-------------|
| **Task parsing** | Reads `tasks.md` for checkbox state |
| **Hierarchy awareness** | Extracts Epic/Story context for the AI |
| **Dynamic prompts** | Uses `openspec instructions` for context |
| **Branch management** | Creates/switches to `ralph/<change-name>` |
| **Progress tracking** | Shows completed/remaining counts |
| **Graceful completion** | Suggests next steps on finish |

---

## Workflow Examples

### Example 1: New Feature Development

```bash
# 1. Initialize OpenSpec (if not done)
openspec init --tools claude

# 2. Create a new change
/opsx:new add-priority-feature

# 3. Create all artifacts
/opsx:ff add-priority-feature
# This creates: proposal.md, specs/, design.md, tasks.md

# 4. Review and adjust tasks.md as needed
# (ensure proper Epic > Story > Task hierarchy)

# 5. Run the Ralph loop
./openspec-ralph add-priority-feature --tool claude 30

# 6. After completion, validate
openspec validate add-priority-feature

# 7. Archive the change
openspec archive add-priority-feature
```

### Example 2: tasks.md Structure

```markdown
# Tasks

## 1. Database Layer
Foundation for priority storage.

### Story 1.1: Priority Column
- [ ] 1.1.1 Add priority column to tasks table (enum: high|medium|low, default: medium)
- [ ] 1.1.2 Create database migration script
- [ ] 1.1.3 Run migration and verify

### Story 1.2: Priority Queries
- [ ] 1.2.1 Add getPriorityTasks query function
- [ ] 1.2.2 Update existing task queries to include priority

## 2. Backend Logic
Server-side priority handling.

### Story 2.1: Task Service Updates
- [ ] 2.1.1 Add priority field to TaskDTO
- [ ] 2.1.2 Update createTask to accept priority
- [ ] 2.1.3 Update updateTask to modify priority
- [ ] 2.1.4 Add filterByPriority method

## 3. User Interface
Priority visualization and controls.

### Story 3.1: Priority Badge Component
- [ ] 3.1.1 Create PriorityBadge component with color variants
- [ ] 3.1.2 Add priority badge to TaskCard component
- [ ] 3.1.3 Verify in browser

### Story 3.2: Priority Selector
- [ ] 3.2.1 Create PrioritySelector dropdown component
- [ ] 3.2.2 Add selector to TaskEditModal
- [ ] 3.2.3 Wire up to updateTask API
- [ ] 3.2.4 Verify in browser

### Story 3.3: Priority Filter
- [ ] 3.3.1 Add priority filter dropdown to TaskList header
- [ ] 3.3.2 Implement filter logic with URL param persistence
- [ ] 3.3.3 Add empty state for no matching tasks
- [ ] 3.3.4 Verify in browser
```

### Example 3: Delta Specs with Scenarios

```markdown
# Delta for Tasks

## ADDED Requirements

### Requirement: Task Priority
Tasks MUST support priority levels to help users focus on important work.

#### Scenario: Default priority assignment
- GIVEN a new task is created
- WHEN no priority is specified
- THEN the task priority is set to "medium"

#### Scenario: Priority modification
- GIVEN an existing task
- WHEN the user changes the priority
- THEN the new priority is saved immediately
- AND the UI reflects the change without refresh

#### Scenario: Priority filtering
- GIVEN tasks with mixed priorities
- WHEN the user selects "High" from the priority filter
- THEN only high-priority tasks are displayed
- AND the filter selection persists in the URL

#### Scenario: Priority visual indication
- GIVEN a task with high priority
- WHEN viewing the task list
- THEN a red priority badge is visible on the task card
```

### Example 4: Design.md with Learnings

```markdown
# Design: Add Priority Feature

## Technical Approach

Priority stored as enum in database with three levels.
React context not needed - prop drilling is fine for this scope.

## Architecture Decisions

### Decision: Enum over numeric priority
Using string enum ('high'|'medium'|'low') instead of numeric (1,2,3) because:
- More readable in database queries
- Self-documenting in code
- No ambiguity about sort order

### Decision: URL params for filter state
Filter persists in URL so users can:
- Share filtered views
- Use browser back/forward
- Bookmark specific filters

## Learnings

### 2025-01-24 - Task 1.1.1
- Database migration required `IF NOT EXISTS` for idempotency
- The `tasks` table is in the `app` schema, not `public`

### 2025-01-24 - Task 3.1.1
- Existing Badge component supports `variant` prop for colors
- Color tokens are in `src/styles/tokens.css`

### 2025-01-25 - Task 3.2.2
- TaskEditModal uses react-hook-form, not useState
- Must use `register` and `setValue` for new fields
```

---

## File Format Specifications

### tasks.md Format

```markdown
# Tasks

## [Epic Number]. [Epic Title]
[Optional epic description]

### Story [Epic].[Story]: [Story Title]
- [ ] [Epic].[Story].[Task] [Task description]
- [ ] [Epic].[Story].[Task] [Task description]
- [x] [Epic].[Story].[Task] [Completed task]

### Story [Epic].[Story]: [Another Story]
- [ ] [Epic].[Story].[Task] [Task description]
```

**Parsing rules:**
- `## ` = Epic heading
- `### ` or `### Story` = Story heading
- `- [ ] ` = Incomplete task
- `- [x] ` = Complete task
- Numbering format: `Epic.Story.Task` (e.g., `1.2.3`)

### design.md Learnings Section

```markdown
## Learnings

### [Date] - Task [Number]
- [Learning 1]
- [Learning 2]

### [Date] - Task [Number]
- [Pattern discovered]
- [Gotcha encountered]
```

### Delta Spec Format

```markdown
# Delta for [Domain]

## ADDED Requirements

### Requirement: [Name]
[Description using MUST/SHALL/SHOULD]

#### Scenario: [Scenario name]
- GIVEN [precondition]
- WHEN [action]
- THEN [expected result]
- AND [additional result]

## MODIFIED Requirements

### Requirement: [Name]
[New description]
(Previously: [old description])

## REMOVED Requirements

### Requirement: [Name]
(Deprecated because: [reason])
```

---

## Migration from Ralph

If you have existing `prd.json` files:

### 1. Convert prd.json to tasks.md

```bash
# Use jq to extract stories
cat prd.json | jq -r '.userStories[] | "- [ ] \(.id) \(.title)"'
```

Or manually structure into Epic > Story > Task hierarchy.

### 2. Convert acceptanceCriteria to Delta Specs

```json
// From prd.json
"acceptanceCriteria": [
  "Filter dropdown has options: All | High | Medium | Low",
  "Filter persists in URL params"
]
```

```markdown
<!-- To delta spec -->
#### Scenario: Priority filtering options
- GIVEN the task list view
- WHEN viewing the priority filter
- THEN options include: All, High, Medium, Low

#### Scenario: Filter persistence
- GIVEN a priority filter is selected
- WHEN the page is refreshed
- THEN the filter selection is preserved via URL params
```

### 3. Convert progress.txt patterns to specs

```
## Codebase Patterns
- Use `sql<number>` template for aggregations
```

```markdown
<!-- To specs/database/spec.md -->
### Requirement: Query Templates
Database queries SHOULD use numbered sql templates for aggregations.

#### Scenario: Aggregation query
- GIVEN a query that aggregates data
- WHEN writing the query
- THEN use `sql<number>` template syntax
```

---

## Summary

The OpenSpec-Ralph integration provides:

1. **Structured task management** via `tasks.md` with Epic > Story > Task hierarchy
2. **Rich context** from OpenSpec artifacts (proposal, design, specs)
3. **Verifiable requirements** through Given/When/Then scenarios in delta specs
4. **CLI-based state management** - no direct file editing needed
5. **Formal archive** that updates source of truth specs
6. **Dynamic AI instructions** via `openspec instructions` command

The new `openspec-ralph` script replaces `ralph.sh` while maintaining the core loop concept with improved structure and verification capabilities.
