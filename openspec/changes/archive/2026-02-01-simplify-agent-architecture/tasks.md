## 1. Remove Session Module

- [x] 1.1 Delete `src/session/state.rs` (session file management)
- [x] 1.2 Delete `src/session/scoped.rs` (ScopedSession wrapper)
- [x] 1.3 Delete `src/session/cli.rs` (agent CLI definitions)
- [x] 1.4 Delete agent command handlers from `src/session/mod.rs` (context, task, status, learn commands)
- [x] 1.5 Update `src/session/mod.rs` to only export instructions module
- [x] 1.6 Remove `RootCommand::Agent` from main CLI and update `src/main.rs`

## 2. Create Agent Prompt Template

- [x] 2.1 Rewrite `src/session/instructions.rs` to generate simple prompt with change location
- [x] 2.2 Include workflow instructions (read files, implement, mark tasks done by editing tasks.md)
- [x] 2.3 Include verification commands from spec adapter
- [x] 2.4 Remove dependency on session state (no env vars, no session ID)

## 3. Simplify Orchestrator

- [x] 3.1 Remove ScopedSession usage from `src/ralph_loop/orchestrator.rs`
- [x] 3.2 Remove session environment variable passing
- [x] 3.3 Simplify to single agent spawn with generated prompt
- [x] 3.4 Remove task completion parsing (agent edits files directly)
- [x] 3.5 Emit only AgentOutput and Complete events

## 4. Update TUI

- [x] 4.1 Remove session-related state from `src/app.rs` (loop_state task/story counters)
- [x] 4.2 Update loop execution screen to show agent output only
- [x] 4.3 Simplify LoopEvent enum (remove StoryStarted, TaskCompleted, StoryCompleted)
- [x] 4.4 Update loop result screen for simplified state

## 5. Cleanup

- [x] 5.1 Remove `.ralph/locks/` directory handling code
- [x] 5.2 Remove lock file path constants and functions
- [x] 5.3 Remove unused error variants (SessionRequired, StoryRequired, ChangeLocked)
- [x] 5.4 Remove unused dependencies from Cargo.toml if applicable (fs2, uuid check)
- [x] 5.5 Run `cargo check && cargo clippy -- -D warnings && cargo test` to verify
