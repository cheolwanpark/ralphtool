## Context

Ralphtool has three layers: Agent CLI (for machines), Agent Layer (session/command logic), and Spec Layer (abstraction over OpenSpec/SpecKit). The current implementation has 5 traits, 2 type hierarchies, and ~2000 lines across agent + spec modules. The goal is to reduce this to ~800-1000 lines while maintaining functionality.

Current pain points:
- Agent layer directly imports `OpenSpecAdapter` instead of using abstractions
- `Story` (from tasks.md) and `UserStory` (from specs) are parallel hierarchies that need complex mapping
- Session buffers learnings/patterns with timestamps and references - overkill for simple append-to-file
- Separate verification phase adds complexity for marginal benefit

## Goals / Non-Goals

**Goals:**
- Single `SpecAdapter` trait that agent layer uses exclusively
- One type hierarchy: `Story > Task`, plus flat `Scenario` list
- Simple session state: just `{id, change, story_id, learnings[]}`
- Custom error type with machine-readable codes
- Agent layer has zero imports from `spec::openspec` module
- Reduce total lines by ~50%

**Non-Goals:**
- Changing the CLI interface (commands stay the same)
- Adding new features
- Supporting multiple concurrent sessions
- Async/parallel execution

## Decisions

### Decision 1: Single unified trait

```rust
pub trait SpecAdapter {
    fn stories(&self) -> Result<Vec<Story>>;
    fn scenarios(&self) -> Result<Vec<Scenario>>;
    fn context(&self, story_id: &str) -> Result<Context>;
    fn mark_done(&mut self, task_id: &str) -> Result<()>;
    fn append_learnings(&mut self, learnings: &[String]) -> Result<()>;
}
```

**Rationale**: All 5 current traits are always implemented together. No use case for partial implementation. One trait is simpler.

**Alternative considered**: Keep read/write split (2 traits). Rejected because they're always used together.

### Decision 2: Remove UserStory, keep Scenario flat

```rust
struct Scenario {
    name: String,
    story_id: String,  // links to Story.id for grouping
    given: Vec<String>,
    when: String,
    then: Vec<String>,
}
```

**Rationale**: `UserStory` was only used to group scenarios. A simple `story_id` field achieves the same. UI layer can group by this field.

### Decision 3: Verification via AI token, not state

Remove `VerifyCommand`, `StorySource::mark_passed`, `UserStory.passed`. Instead, AI outputs `<promise>VERIFIED</promise>` when story is verified.

**Rationale**: Simpler. The orchestrator already looks for `<promise>COMPLETE</promise>`. Same pattern.

### Decision 4: Factory function for adapter creation

```rust
pub fn create_adapter(change: &str) -> Result<Box<dyn SpecAdapter>>
```

**Rationale**: Agent layer imports only `spec::create_adapter` and `spec::SpecAdapter`. Zero coupling to concrete types. When SpecKit comes, just update the factory.

### Decision 5: Custom error enum

```rust
pub enum Error {
    ChangeNotFound(String),
    TaskNotFound(String),
    StoryNotFound(String),
    SessionRequired,
    StoryRequired,
    ChangeLocked(String),
    Io(std::io::Error),
    Json(serde_json::Error),
    Command { cmd: String, stderr: String },
    Parse(String),
}
```

**Rationale**: Machine-readable error codes for agent CLI. `e.code()` returns strings like `"SESSION_REQUIRED"`. Better than parsing anyhow error messages.

### Decision 6: Learnings as simple strings

```rust
struct Session {
    learnings: Vec<String>,  // not Vec<SessionLearning>
}
```

**Rationale**: Timestamps and task references add complexity. Just store the text, write all on flush. Simple.

## Risks / Trade-offs

- **[Breaking changes]** → Internal refactor only, CLI interface unchanged. No external impact.
- **[Loss of learning metadata]** → Task/story context lost. Acceptable - learnings are still useful as text.
- **[Scenario parsing still needed for TUI]** → Keep parsing but simplify storage. UI groups by story_id.

## Learnings

### 2026-02-01
- Verified CLI commands work correctly
