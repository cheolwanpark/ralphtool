# OpenSpec-Ralph Verification Strategy

This document defines how to make Ralph loop verification more robust using OpenSpec's structured scenarios.

---

## The Verification Problem

### Ralph's Original Approach

Ralph uses plain-text acceptance criteria:

```json
"acceptanceCriteria": [
  "Filter dropdown has options: All | High | Medium | Low",
  "Typecheck passes"
]
```

**Problems:**
1. **Not machine-readable** - AI interprets freely
2. **Not executable** - No direct mapping to tests
3. **Inconsistent verification** - Each iteration may verify differently
4. **No traceability** - Can't track which tests cover which criteria

### The Solution: Scenario-Based Verification

OpenSpec scenarios provide structured, verifiable requirements:

```markdown
#### Scenario: Priority filter options
- GIVEN the task list page
- WHEN viewing the priority filter dropdown
- THEN the options are: All, High, Medium, Low
- AND "All" is selected by default
```

This format:
- Is parseable
- Maps to tests
- Provides clear verification steps
- Enables traceability

---

## Verification Layers

```
┌─────────────────────────────────────────────────────────────┐
│                    VERIFICATION PYRAMID                      │
│                                                              │
│                         ┌─────┐                              │
│                         │ E2E │  Browser verification        │
│                        /│     │\  (Playwright, Cypress)      │
│                       / └─────┘ \                            │
│                      /           \                           │
│                     / ┌─────────┐ \                          │
│                    /  │Integration│  API/Service tests       │
│                   /   │  Tests   │                           │
│                  /    └─────────┘  \                         │
│                 /                   \                        │
│                /   ┌─────────────┐   \                       │
│               /    │  Unit Tests │    \  Component/function  │
│              /     │             │     \                     │
│             /      └─────────────┘      \                    │
│            /                             \                   │
│           /      ┌─────────────────┐      \                  │
│          /       │  Static Checks  │       \  Typecheck,     │
│         /        │                 │        \ lint, build    │
│        └─────────└─────────────────┘─────────┘               │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### Layer 1: Static Checks (Every Task)

**Always run after each task:**
- Typecheck (`tsc --noEmit`, `mypy`, etc.)
- Lint (`eslint`, `ruff`, etc.)
- Build (`npm run build`)

**In tasks.md:**
```markdown
- [ ] 1.1.1 Add priority column to tasks table
  - Typecheck passes
  - Build succeeds
```

### Layer 2: Unit Tests (Per Task)

**Run for tasks with testable logic:**

```markdown
- [ ] 2.1.1 Add getPriorityTasks query function
  - Unit tests pass: `npm test src/queries/priority.test.ts`
```

### Layer 3: Integration Tests (Per Story)

**Run after all tasks in a story complete:**

```markdown
### Story 2.1: Task Service Updates
- [x] 2.1.1 Add priority field to TaskDTO
- [x] 2.1.2 Update createTask to accept priority
- [x] 2.1.3 Update updateTask to modify priority
- [x] 2.1.4 Add filterByPriority method

**Story verification:** `npm test src/services/task.integration.test.ts`
```

### Layer 4: E2E Tests (Per Epic or Change)

**Run after major milestones:**

```markdown
## 3. User Interface

**Epic verification:** `npm run test:e2e -- --spec cypress/e2e/priority.cy.ts`
```

---

## Scenario Specification Format

### Basic Scenario

```markdown
#### Scenario: [Descriptive name]
- GIVEN [precondition/context]
- WHEN [action/trigger]
- THEN [expected outcome]
- AND [additional outcome]
```

### Scenario with Test Mapping

```markdown
#### Scenario: Priority filter persists in URL
- GIVEN the task list page
- WHEN I select "High" from the priority filter
- THEN the URL contains "?priority=high"
- AND refreshing the page maintains the filter

**Test:** `cypress/e2e/priority.cy.ts:45` [e2e]
**Test:** `src/hooks/useUrlParams.test.ts:12` [unit]
```

### Scenario with Verification Command

```markdown
#### Scenario: Default priority assignment
- GIVEN a new task is being created
- WHEN no priority is specified
- THEN the task priority defaults to "medium"

**Verify:** `npm test -- --grep "default priority"`
```

---

## Verification Checkpoints

### Task-Level Verification

Each task has implicit and explicit checks:

```markdown
- [ ] 1.1.1 Add priority column to tasks table
```

**Implicit checks (always run):**
- `npm run typecheck`
- `npm run lint`
- `npm run build`

**Explicit checks (if specified):**
```markdown
- [ ] 1.1.1 Add priority column to tasks table
  - Migration runs without error
  - Column exists: `SELECT priority FROM tasks LIMIT 1`
```

### Story-Level Verification

After all tasks in a story complete, run story scenarios:

```markdown
### Story 1.2: Priority Queries
- [x] 1.2.1 Add getPriorityTasks query function
- [x] 1.2.2 Update existing task queries to include priority

**Story scenarios:**
- [ ] Default priority assignment (verify)
- [ ] Priority query returns correct tasks (verify)
```

The orchestrator can:
1. Detect story completion (all tasks checked)
2. Run associated scenario tests
3. Mark scenarios verified

### Change-Level Verification

Before archiving, run full verification:

```bash
openspec validate add-priority-feature
npm run test
npm run test:e2e
```

---

## Scenario-to-Test Generation

### Manual Test Mapping

In `design.md`, maintain a test mapping table:

```markdown
## Test Coverage

| Scenario | Test File | Type | Status |
|----------|-----------|------|--------|
| Default priority | `priority.test.ts:15` | unit | ✓ |
| Filter options | `priority.cy.ts:30` | e2e | ✓ |
| URL persistence | `priority.cy.ts:55` | e2e | ✓ |
| Visual indicator | `priority.cy.ts:80` | e2e | pending |
```

### Scenario Template for Test Generation

Scenarios written in a consistent format can be parsed to generate test skeletons:

```markdown
#### Scenario: Toggle from light to dark
- GIVEN the application is in light theme
- WHEN the user clicks the theme toggle
- THEN the theme switches to dark
- AND the preference is saved to localStorage
```

**Generated test skeleton:**

```typescript
describe('Theme Toggle', () => {
  it('should toggle from light to dark', () => {
    // GIVEN the application is in light theme
    // TODO: Setup light theme

    // WHEN the user clicks the theme toggle
    // TODO: Click toggle

    // THEN the theme switches to dark
    // TODO: Assert dark theme

    // AND the preference is saved to localStorage
    // TODO: Assert localStorage
  });
});
```

### Executable Scenario Format

For fully automated verification, use a structured format:

```yaml
# scenarios/priority-filter.yaml
scenario: Priority filter options
given:
  - action: navigate
    target: /tasks
when:
  - action: click
    target: "[data-testid='priority-filter']"
then:
  - assert: visible
    target: "[data-testid='filter-option-all']"
  - assert: visible
    target: "[data-testid='filter-option-high']"
  - assert: visible
    target: "[data-testid='filter-option-medium']"
  - assert: visible
    target: "[data-testid='filter-option-low']"
```

This could be processed by a scenario runner to generate and execute tests.

---

## Verification in the Ralph Loop

### Enhanced Orchestrator Logic

```bash
#!/bin/bash
# In openspec-ralph script

verify_task() {
  local task_text="$1"

  echo "Running task verification..."

  # Static checks (always)
  npm run typecheck || return 1
  npm run lint || return 1
  npm run build || return 1

  # Task-specific tests (if exist)
  # Extract test pattern from task metadata
  local test_pattern=$(extract_test_pattern "$task_text")
  if [[ -n "$test_pattern" ]]; then
    npm test -- --grep "$test_pattern" || return 1
  fi

  return 0
}

verify_story() {
  local story_name="$1"
  local change_name="$2"

  echo "Running story verification: $story_name"

  # Get scenario tests for this story
  local scenario_tests=$(get_story_scenarios "$change_name" "$story_name")

  if [[ -n "$scenario_tests" ]]; then
    npm test -- $scenario_tests || return 1
  fi

  return 0
}

check_story_completion() {
  local story_line="$1"
  local tasks_file="$2"

  # Get all tasks under this story
  # Check if all are [x]
  # If yes, run story verification
}
```

### AI Instructions for Verification

Include verification steps in the AI prompt:

```markdown
## After Implementing the Task

1. **Run static checks:**
   ```bash
   npm run typecheck
   npm run lint
   npm run build
   ```

2. **Run relevant unit tests:**
   ```bash
   npm test -- --grep "[related pattern]"
   ```

3. **If this completes a story, run story scenarios:**
   Check if all tasks under the current story heading are now `[x]`.
   If yes, find and run the story's scenario tests.

4. **Report verification results:**
   Include in your response:
   - Which checks passed/failed
   - Test output summary
   - Any issues discovered
```

---

## Verification Artifacts

### verification-report.md

After each story or epic completes, generate a verification report:

```markdown
# Verification Report: Story 2.1 - Task Service Updates

## Summary
- **Status:** PASSED
- **Date:** 2025-01-24
- **Tasks:** 4/4 complete

## Static Checks
| Check | Status | Duration |
|-------|--------|----------|
| Typecheck | ✓ | 2.3s |
| Lint | ✓ | 1.1s |
| Build | ✓ | 8.7s |

## Unit Tests
| Test Suite | Passed | Failed | Duration |
|------------|--------|--------|----------|
| task.service.test.ts | 12 | 0 | 0.8s |
| priority.test.ts | 5 | 0 | 0.3s |

## Scenarios Verified
| Scenario | Status |
|----------|--------|
| Default priority assignment | ✓ |
| Priority modification | ✓ |
| Priority query filtering | ✓ |

## Coverage
- Lines: 87%
- Branches: 72%
- Functions: 91%
```

### Scenario Verification Log

Track which scenarios have been verified:

```markdown
# Scenario Verification Log

## Change: add-priority-feature

### Epic 1: Database Layer
| Scenario | Verified | Date | Method |
|----------|----------|------|--------|
| Default priority assignment | ✓ | 2025-01-24 | unit test |
| Priority column exists | ✓ | 2025-01-24 | migration |

### Epic 2: Backend Logic
| Scenario | Verified | Date | Method |
|----------|----------|------|--------|
| Create task with priority | ✓ | 2025-01-24 | integration |
| Update task priority | ✓ | 2025-01-24 | integration |
| Filter by priority | ✓ | 2025-01-24 | integration |

### Epic 3: User Interface
| Scenario | Verified | Date | Method |
|----------|----------|------|--------|
| Priority badge display | ✓ | 2025-01-25 | e2e |
| Priority selector | ✓ | 2025-01-25 | e2e |
| Filter dropdown | ✓ | 2025-01-25 | e2e |
| URL persistence | ✓ | 2025-01-25 | e2e |
```

---

## Browser Verification Integration

### Automated Browser Tests

For UI scenarios, integrate with Playwright or Cypress:

```markdown
#### Scenario: Priority badge visibility
- GIVEN a task with high priority
- WHEN viewing the task list
- THEN a red badge labeled "High" is visible

**Test:** `cypress/e2e/priority.cy.ts:80`
```

```typescript
// cypress/e2e/priority.cy.ts
describe('Priority Badge', () => {
  it('displays red badge for high priority tasks', () => {
    // Create high priority task
    cy.createTask({ title: 'Urgent task', priority: 'high' });

    // Navigate to task list
    cy.visit('/tasks');

    // Verify badge
    cy.contains('Urgent task')
      .parent()
      .find('[data-testid="priority-badge"]')
      .should('have.class', 'bg-red-500')
      .and('contain', 'High');
  });
});
```

### Visual Regression Testing

For UI changes, add visual verification:

```markdown
#### Scenario: Theme toggle appearance
- GIVEN the settings page
- WHEN viewing the theme toggle
- THEN it matches the approved design

**Visual test:** `cypress/e2e/visual/theme-toggle.cy.ts`
```

```typescript
// Using Percy or similar
it('theme toggle matches design', () => {
  cy.visit('/settings');
  cy.percySnapshot('Theme Toggle - Light Mode');

  cy.get('[data-testid="theme-toggle"]').click();
  cy.percySnapshot('Theme Toggle - Dark Mode');
});
```

---

## Verification Configuration

### Project-Level Configuration

In `openspec/config.yaml`:

```yaml
verification:
  static:
    typecheck: "npm run typecheck"
    lint: "npm run lint"
    build: "npm run build"

  tests:
    unit: "npm test"
    integration: "npm run test:integration"
    e2e: "npm run test:e2e"

  thresholds:
    coverage:
      lines: 80
      branches: 70
      functions: 85

  story_completion:
    run_scenarios: true
    generate_report: true
```

### Change-Level Configuration

In `openspec/changes/add-priority/.openspec.yaml`:

```yaml
verification:
  skip_e2e: false
  additional_checks:
    - "npm run test:a11y"

  scenario_mapping:
    "Story 1.1": "src/tests/priority-db.test.ts"
    "Story 2.1": "src/tests/priority-service.test.ts"
    "Story 3.1": "cypress/e2e/priority-ui.cy.ts"
```

---

## Summary

### Verification Improvements Over Ralph

| Aspect | Ralph | OpenSpec-Ralph |
|--------|-------|----------------|
| Format | Plain text criteria | Structured Given/When/Then |
| Traceability | None | Scenario → Test mapping |
| Automation | Manual interpretation | Parseable, executable |
| Reporting | None | Verification reports |
| Layers | Just "typecheck passes" | Static → Unit → Integration → E2E |

### Key Principles

1. **Every task has static checks** - Typecheck, lint, build
2. **Stories have scenarios** - Structured verification criteria
3. **Scenarios map to tests** - Explicit traceability
4. **Story completion triggers verification** - Automatic scenario testing
5. **Reports document status** - Audit trail of verification

### Implementation Priority

1. **Phase 1:** Add static checks to every task
2. **Phase 2:** Write scenarios in delta specs
3. **Phase 3:** Map scenarios to existing tests
4. **Phase 4:** Generate verification reports
5. **Phase 5:** Scenario-to-test generation tooling
