## Context

The ralphtool currently has:
- `src/spec/` - SpecAdapter trait with OpenSpecAdapter implementation (stories, tasks, scenarios, context)
- `src/agent/` - Session management and agent CLI commands (context, task done, learn, status)
- `src/ui/` - TUI for change selection and preview

Missing: an orchestrator that ties these together into an autonomous loop.

## Goals / Non-Goals

**Goals:**
- Implement the Ralph loop as a Rust module integrated with TUI
- Abstract coding agents behind a trait for future extensibility
- Reorganize modules to separate concerns (session vs agent)
- Show real-time loop progress in TUI

**Non-Goals:**
- Shell script orchestrator (doing it in Rust for TUI integration)
- Multiple AI backends in initial implementation (Claude Code only)
- Parallel story execution (sequential for simplicity)
- Verification phase as separate step (inline with task completion)

## Decisions

### Decision 1: Module structure

```
src/
├── session/          # Renamed from src/agent/
│   ├── mod.rs        # Session state + CLI command handlers
│   ├── cli.rs        # Clap definitions
│   ├── session.rs    # Session struct, load/save
│   └── instructions.rs # Prompt generation for agents
│
├── agent/            # NEW - coding agent abstraction
│   ├── mod.rs        # CodingAgent trait
│   └── claude.rs     # ClaudeAgent implementation
│
├── ralph_loop/       # NEW - orchestration (named to avoid Rust keyword 'loop')
│   ├── mod.rs        # Loop state, events
│   └── orchestrator.rs  # Main loop logic
│
├── spec/             # Unchanged
└── ui/               # Enhanced with loop screen
    ├── loop_screen.rs   # Loop execution display
    └── result_screen.rs # Post-loop result review
```

**Rationale:** Clear separation between session management (CLI interface for agents) and the agent abstraction (how to spawn/communicate with AI).

### Decision 2: CodingAgent trait

```rust
pub trait CodingAgent {
    /// Spawn agent with prompt, return output
    fn run(&self, prompt: &str, config: &AgentConfig) -> Result<AgentOutput>;
}

pub struct AgentConfig {
    pub allowed_tools: Vec<String>,
    pub max_turns: u32,
    pub timeout: Duration,
}

pub struct AgentOutput {
    pub result: String,
    pub session_id: String,
    pub usage: TokenUsage,
}
```

**Rationale:** Minimal trait - just spawn and get output. Config handles Claude-specific flags. Future agents (Amp, Cursor) implement the same trait.

### Decision 3: Prompt generation in session module

The session module generates AI instructions using spec layer data:

```rust
// session/mod.rs
pub fn generate_instructions(adapter: &dyn SpecAdapter, story_id: &str) -> Result<String> {
    let context = adapter.context(story_id)?;
    // Build markdown prompt from context.proposal, context.design,
    // context.story, context.tasks, context.verify
}
```

**Rationale:** Session module already has access to spec layer and understands the context structure. Keeps agent module simple (just spawn/output).

### Decision 4: Loop orchestration flow

```
┌─────────────────────────────────────────────────────────────────┐
│  Orchestrator::run(change_name)                                  │
│                                                                  │
│  1. Create adapter via spec::create_adapter(change_name)         │
│  2. Get stories via adapter.stories()                            │
│  3. For each incomplete story:                                   │
│     a. Generate instructions via session::generate_instructions  │
│     b. Spawn agent via CodingAgent::run(instructions)            │
│     c. Parse output for task completions                         │
│     d. Update tasks.md via adapter.mark_done()                   │
│     e. Emit LoopEvent for TUI                                    │
│  4. Flush learnings via adapter.append_learnings()               │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

**Rationale:** Simple sequential flow. Each story = one agent spawn. Agent is expected to complete all tasks in a story before returning.

### Decision 5: TUI loop screen

Use async channel for loop events:
- `LoopEvent::StoryStarted { story_id, title }`
- `LoopEvent::TaskCompleted { task_id }`
- `LoopEvent::StoryCompleted { story_id }`
- `LoopEvent::AgentOutput { line }` (streaming)
- `LoopEvent::Error { message }`
- `LoopEvent::Complete`

TUI subscribes to channel and updates display. User can press `q` to stop.

**Rationale:** Decouples loop logic from TUI rendering. Events are the interface.

### Decision 6: Claude Code integration

```rust
// agent/claude.rs
impl CodingAgent for ClaudeAgent {
    fn run(&self, prompt: &str, config: &AgentConfig) -> Result<AgentOutput> {
        let output = Command::new("claude")
            .arg("-p")
            .arg(prompt)
            .arg("--output-format").arg("json")
            .arg("--allowedTools").arg(config.allowed_tools.join(","))
            .arg("--max-turns").arg(config.max_turns.to_string())
            .output()?;

        // Parse JSON output
        let response: ClaudeResponse = serde_json::from_slice(&output.stdout)?;
        Ok(AgentOutput { ... })
    }
}
```

**Rationale:** Direct CLI invocation. No streaming initially - wait for full output. Simple and reliable.

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Agent hangs or takes too long | `--max-turns` limit + timeout in spawn |
| Agent produces invalid output | Parse gracefully, log errors, continue to next story |
| TUI blocks during agent run | Run loop in separate thread, communicate via channel |
| Claude CLI not installed | Check on startup, show helpful error |
| Story too large for context | Trust agent to handle; future: split stories |

## Open Questions

- Should we support resuming a partially completed loop?
- How to handle agent errors mid-story (retry? skip? ask user?)?
