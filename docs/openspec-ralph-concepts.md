# OpenSpec-Ralph Integration: Concepts

This document covers the conceptual mapping between Ralph and OpenSpec, including improved approaches to story organization and verification.

---

## Philosophy Comparison

| Aspect | Ralph | OpenSpec | Integration Approach |
|--------|-------|----------|---------------------|
| **Structure** | Flat JSON task list | Hierarchical artifacts + specs | Use OpenSpec structure with Ralph execution model |
| **Memory** | `progress.txt` + git | Specs as source of truth | Specs persist patterns, design.md captures learnings |
| **Verification** | Acceptance criteria strings | Given/When/Then scenarios | Scenarios become executable verification |
| **Scope** | Single feature per run | Multiple parallel changes | One OpenSpec change = one Ralph run |
| **Archive** | Date-prefixed folders | Formal delta merge to specs | Archive preserves full context + updates specs |

---

## Component Mapping

### Core Files

| Ralph Component | OpenSpec Equivalent | Notes |
|-----------------|---------------------|-------|
| `prd.json` | `tasks.md` + `proposal.md` | Proposal captures intent; tasks.md has checkboxes |
| `prd.json.branchName` | Change folder name | `changes/add-dark-mode/` implies branch `ralph/add-dark-mode` |
| `prd.json.userStories[]` | Hierarchical tasks in `tasks.md` | See [Story Organization](#story-organization) |
| `prd.json.passes` | Checkbox state `- [ ]` / `- [x]` | Parse markdown to determine completion |
| `prd.json.priority` | Task order (top-to-bottom) | First unchecked = next task |
| `prd.json.acceptanceCriteria` | Delta spec scenarios | Given/When/Then format is verifiable |
| `progress.txt` | `design.md` + git commits | Learnings go to design, patterns go to specs |
| `progress.txt` (Codebase Patterns) | `openspec/specs/` | Patterns become formal requirements |
| `prompt.md` / `CLAUDE.md` | Generated via `openspec instructions` | Dynamic context from artifacts |
| `AGENTS.md` | Domain specs | `specs/auth/spec.md` instead of scattered AGENTS.md |
| `ralph.sh` | New `openspec-ralph` orchestrator | See implementation doc |

### State Tracking

**Ralph approach:**
```json
{
  "userStories": [
    { "id": "US-001", "passes": false },
    { "id": "US-002", "passes": true }
  ]
}
```

**OpenSpec approach:**
```markdown
## 1. Database Layer
- [x] 1.1 Add priority column to tasks table
- [ ] 1.2 Create migration script

## 2. Backend Logic
- [ ] 2.1 Update task service with priority support
```

**Parsing logic:**
- `- [ ]` = incomplete (equivalent to `passes: false`)
- `- [x]` = complete (equivalent to `passes: true`)
- First unchecked task = next work item
- Top-to-bottom order = priority

---

## Story Organization

### The Problem with 1:1 Task=Story

Ralph's original model: one user story = one iteration. This works but has limitations:

1. **Context loss**: Each iteration forgets the bigger picture
2. **No hierarchy**: All stories appear equal regardless of complexity
3. **Rigid granularity**: Must split everything to context-window size

### Improved Hierarchy: Epic > Story > Task

OpenSpec naturally supports hierarchy through `tasks.md` structure:

```markdown
# Tasks

## Epic 1: Theme Infrastructure
The foundational work needed for theming support.

### Story 1.1: Create Theme Context
- [ ] 1.1.1 Define ThemeContext with light/dark state
- [ ] 1.1.2 Add ThemeProvider wrapper component
- [ ] 1.1.3 Create useTheme hook

### Story 1.2: CSS Variable System
- [ ] 1.2.1 Define color tokens as CSS custom properties
- [ ] 1.2.2 Create dark theme color mappings
- [ ] 1.2.3 Add theme class to document root

## Epic 2: User Interface
User-facing theme controls.

### Story 2.1: Theme Toggle Component
- [ ] 2.1.1 Create ThemeToggle button component
- [ ] 2.1.2 Add toggle to settings page
- [ ] 2.1.3 Persist preference to localStorage
```

### Execution Model

**Per-iteration granularity: Task level (numbered items)**

Each Ralph iteration works on ONE task (e.g., `1.1.1`), not an entire story. This provides:

- **Smaller units**: Tasks fit comfortably in one context window
- **Clear progress**: Visible checkmarks at fine granularity
- **Context preservation**: Story/Epic headings provide context even with fresh memory

**Story-level verification**: After all tasks in a story complete, verify the story's scenarios.

### Mapping to OpenSpec Artifacts

| Level | OpenSpec Location | Purpose |
|-------|-------------------|---------|
| **Epic** | `## Heading` in tasks.md | High-level grouping |
| **Story** | `### Subheading` in tasks.md | Cohesive unit of value |
| **Task** | `- [ ] Numbered item` | Single iteration work unit |
| **Scenario** | Delta specs | Verification criteria |

---

## Verification with OpenSpec Scenarios

### The Problem with Acceptance Criteria

Ralph's acceptance criteria are plain strings:
```json
"acceptanceCriteria": [
  "Filter dropdown has options: All | High | Medium | Low",
  "Typecheck passes"
]
```

These are:
- Not structured
- Not directly executable
- Dependent on human interpretation

### OpenSpec Scenarios as Verification

Delta specs use Given/When/Then format:

```markdown
# Delta for UI

## ADDED Requirements

### Requirement: Theme Toggle
The application MUST allow users to switch between light and dark themes.

#### Scenario: Toggle from light to dark
- GIVEN the application is in light theme
- WHEN the user clicks the theme toggle
- THEN the theme switches to dark
- AND the preference is saved to localStorage
- AND all components update their colors

#### Scenario: System preference detection
- GIVEN no saved theme preference
- WHEN the application loads
- THEN the theme matches the OS preference
```

### Making Scenarios Executable

Scenarios can map to automated tests:

```typescript
// Generated from scenario "Toggle from light to dark"
describe('Theme Toggle', () => {
  it('should switch from light to dark', () => {
    // GIVEN the application is in light theme
    cy.visit('/');
    cy.get('html').should('have.class', 'light');

    // WHEN the user clicks the theme toggle
    cy.get('[data-testid="theme-toggle"]').click();

    // THEN the theme switches to dark
    cy.get('html').should('have.class', 'dark');

    // AND the preference is saved to localStorage
    cy.window().its('localStorage.theme').should('eq', 'dark');
  });
});
```

### Verification Workflow

```
┌─────────────────────────────────────────────────────────────┐
│                    VERIFICATION FLOW                         │
│                                                              │
│  1. TASK COMPLETION                                          │
│     └─ Check off task: - [x] 1.1.1 Create ThemeContext       │
│                                                              │
│  2. STORY COMPLETION (all tasks in story done)               │
│     └─ Run scenario tests for Story 1.1                      │
│     └─ Verify scenarios pass                                 │
│                                                              │
│  3. EPIC COMPLETION (all stories in epic done)               │
│     └─ Run integration tests                                 │
│     └─ Browser verification                                  │
│                                                              │
│  4. CHANGE COMPLETION (all epics done)                       │
│     └─ openspec validate                                     │
│     └─ Full test suite                                       │
│     └─ openspec archive                                      │
└─────────────────────────────────────────────────────────────┘
```

### Scenario-to-Test Mapping

Add a `tests` field to track which scenarios have automated tests:

```markdown
#### Scenario: Toggle from light to dark
- GIVEN the application is in light theme
- WHEN the user clicks the theme toggle
- THEN the theme switches to dark

**Test:** `src/tests/theme.test.ts:15` (cypress)
```

Or in the design.md:

```markdown
## Test Mapping

| Scenario | Test File | Type |
|----------|-----------|------|
| Toggle from light to dark | `tests/theme.cy.ts:15` | e2e |
| System preference detection | `tests/theme.cy.ts:32` | e2e |
| Persist theme preference | `tests/theme.unit.ts:8` | unit |
```

---

## Memory and Learning

### Ralph's Memory Model

```
progress.txt (append-only)
    │
    ├── ## Codebase Patterns (top section)
    │   └── General reusable patterns
    │
    └── ## [Date] - [Story ID]
        └── Per-story learnings
```

### OpenSpec's Memory Model

```
openspec/
├── specs/                      # Permanent patterns (source of truth)
│   ├── auth/spec.md           # Auth patterns
│   └── ui/spec.md             # UI patterns
│
└── changes/add-dark-mode/
    ├── design.md              # Implementation learnings
    │   └── ## Learnings       # Gotchas discovered
    └── specs/                 # Delta specs (merged on archive)
        └── ui/spec.md         # New UI requirements
```

### Migration of Patterns

| Ralph Location | OpenSpec Destination |
|----------------|---------------------|
| `progress.txt` Codebase Patterns | `openspec/specs/` as requirements |
| Per-story learnings | `design.md` ## Learnings section |
| AGENTS.md patterns | Domain specs or project CLAUDE.md |

**Example transformation:**

Ralph `progress.txt`:
```
## Codebase Patterns
- Use `sql<number>` template for aggregations
- Always use `IF NOT EXISTS` for migrations
```

OpenSpec `specs/database/spec.md`:
```markdown
### Requirement: Migration Safety
All database migrations MUST use idempotent patterns.

#### Scenario: Table creation
- GIVEN a migration that creates a table
- WHEN the migration runs multiple times
- THEN no error occurs
- AND the table exists with correct schema

**Implementation note:** Use `IF NOT EXISTS` clauses.
```

---

## Branching Strategy

### Ralph's Approach

```json
{
  "branchName": "ralph/task-priority"
}
```

The `ralph.sh` script checks out or creates this branch.

### OpenSpec Integration

The change folder name becomes the branch:

```
openspec/changes/add-dark-mode/
                 └─────────────┘
                       │
                       ▼
              Branch: ralph/add-dark-mode
```

**Convention:** `ralph/<change-name>` as git branch.

**Detection:** Parse the change folder name from the active change.

---

## Archive and Completion

### Ralph Archive

```
archive/
└── 2025-01-24-task-priority/
    ├── prd.json
    └── progress.txt
```

Just copies files. No spec updates.

### OpenSpec Archive

```bash
openspec archive add-dark-mode
```

This:
1. **Validates** the change
2. **Merges delta specs** into main specs
3. **Moves** change folder to `changes/archive/2025-01-24-add-dark-mode/`
4. **Preserves** full context (proposal, design, tasks, specs)

**Key difference:** OpenSpec archive updates the source of truth specs. Patterns discovered during implementation become permanent requirements.

---

## Summary: The Integrated Model

```
┌────────────────────────────────────────────────────────────────┐
│                 OPENSPEC-RALPH INTEGRATION                      │
│                                                                 │
│  PLANNING PHASE                                                 │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  /opsx:new add-dark-mode                                  │  │
│  │      └─ Creates openspec/changes/add-dark-mode/           │  │
│  │                                                           │  │
│  │  /opsx:ff (or /opsx:continue)                            │  │
│  │      └─ Creates: proposal.md, specs/, design.md, tasks.md │  │
│  └──────────────────────────────────────────────────────────┘  │
│                              │                                  │
│                              ▼                                  │
│  EXECUTION PHASE (Ralph Loop)                                   │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  openspec-ralph add-dark-mode                             │  │
│  │      │                                                    │  │
│  │      ├─ Parse tasks.md for first unchecked task           │  │
│  │      ├─ Get context via: openspec instructions apply      │  │
│  │      ├─ Spawn AI instance (claude/amp)                    │  │
│  │      ├─ AI implements task, runs checks                   │  │
│  │      ├─ AI marks task complete: - [x]                     │  │
│  │      ├─ AI updates design.md with learnings               │  │
│  │      ├─ AI commits changes                                │  │
│  │      └─ Loop until all tasks checked                      │  │
│  └──────────────────────────────────────────────────────────┘  │
│                              │                                  │
│                              ▼                                  │
│  VERIFICATION PHASE                                             │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  After each story completes:                              │  │
│  │      └─ Run scenario tests                                │  │
│  │                                                           │  │
│  │  After all tasks complete:                                │  │
│  │      └─ openspec validate add-dark-mode                   │  │
│  │      └─ Full test suite                                   │  │
│  └──────────────────────────────────────────────────────────┘  │
│                              │                                  │
│                              ▼                                  │
│  ARCHIVE PHASE                                                  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  openspec archive add-dark-mode                           │  │
│  │      └─ Merges delta specs to main specs                  │  │
│  │      └─ Moves to archive with full context                │  │
│  └──────────────────────────────────────────────────────────┘  │
│                                                                 │
└────────────────────────────────────────────────────────────────┘
```

---

## Next Steps

See [OpenSpec-Ralph Implementation](./openspec-ralph-implementation.md) for:
- Complete Ralph loop reference (from original ralph/ directory)
- OpenSpec CLI commands for the workflow
- New orchestrator script design
- Practical examples
