## Context

Currently the orchestrator spawns a single agent with a prompt to complete ALL stories. The agent:
1. Gets a prompt with change location and generic workflow instructions
2. Reads all files, works through stories autonomously
3. Marks tasks complete by editing tasks.md
4. Exits when done (or decides to stop)

Problems with this approach:
- No visibility into progress until agent exits
- Can't resume from a specific story if interrupted
- All scenarios dumped into prompt, no story relevance
- No verification gate between stories

## Goals / Non-Goals

**Goals:**
- Orchestrator iterates per-story, spawning agent for each story
- Track completion status and refresh between iterations
- Generate story-specific prompts with relevant context
- Agent signals completion with `<promise>COMPLETE</promise>` after verification
- SpecAdapter provides tool-specific prompt (how to use OpenSpec files)

**Non-Goals:**
- Automatic story-scenario mapping (use Option C: prompt agent to verify relevant ones)
- Parallel story execution
- Session persistence across ralph runs

## Decisions

### 1. Iteration model: Story loop in orchestrator

**Choice**: Orchestrator runs a loop: `while next_incomplete_story { spawn_agent(story); refresh_state(); }`

**Alternatives considered**:
- Single agent, multiple prompts (complex, requires session)
- Task-level iteration (too granular, context switching overhead)

**Rationale**: Story is the natural unit - contains related tasks, maps to feature slice.

### 2. Completion signal: `<promise>COMPLETE</promise>`

**Choice**: Agent must output `<promise>COMPLETE</promise>` when story is done AND verified.

```
Agent prompt includes:
"After completing all tasks in this story:
1. Run verification commands
2. If all pass, output: <promise>COMPLETE</promise>
3. If any fail, fix issues and re-verify before signaling"
```

**Alternatives considered**:
- Poll tasks.md for checkbox state (agent could mark done without verifying)
- Structured JSON response (harder to parse from streaming output)

**Rationale**: Explicit signal is unambiguous and ties completion to verification.

### 3. Scenario injection: Option C (all scenarios, prompt to filter)

**Choice**: Include all scenarios in prompt, instruct agent: "Focus on scenarios relevant to Story N tasks. You don't need to verify unrelated scenarios."

**Alternatives considered**:
- Explicit story-scenario linking in tasks.md (OpenSpec doesn't support natively)
- Heuristic matching (fragile, hard to debug)

**Rationale**: Agent is smart enough to determine relevance. Keeps format simple.

### 4. Prompt location: `src/agent/prompt.rs`

**Choice**: Move prompt generation from `src/spec/prompt.rs` to `src/agent/prompt.rs`.

**Rationale**: Prompt building is an agent concern, not a spec concern. The spec module provides data, agent module builds prompts.

### 5. SpecAdapter.tool_prompt()

**Choice**: Add `fn tool_prompt(&self) -> String` to SpecAdapter trait.

```rust
trait SpecAdapter {
    // existing methods...
    fn tool_prompt(&self) -> String;  // spec tool usage instructions
}
```

For OpenSpec, returns instructions on:
- File locations (proposal.md, design.md, tasks.md, specs/)
- How to mark tasks complete (edit tasks.md, change `[ ]` to `[x]`)
- Verification commands

**Rationale**: Each spec tool (OpenSpec, SpecKit, etc.) has different file formats and conventions.

## Risks / Trade-offs

**[Risk] Agent may not output completion signal** → Add timeout, check tasks.md as fallback

**[Risk] Story boundaries unclear in tasks.md** → Parser already handles `## N. Title` format, well-defined

**[Trade-off] More agent spawns = more latency** → Acceptable for better progress tracking

**[Trade-off] All scenarios in prompt = larger context** → Agent filters; scenarios are compact

## Module Changes

```
src/
├── agent/
│   ├── mod.rs           # export PromptBuilder
│   ├── claude.rs        # unchanged
│   └── prompt.rs        # NEW: story-specific prompt building
│
├── spec/
│   ├── mod.rs           # add tool_prompt() to SpecAdapter
│   ├── types.rs         # unchanged
│   └── openspec.rs      # implement tool_prompt()
│
└── ralph_loop/
    └── orchestrator.rs  # story iteration loop
```
