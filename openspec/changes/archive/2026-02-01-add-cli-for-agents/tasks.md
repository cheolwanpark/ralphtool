# Tasks

## 1. Project Setup

- [x] 1.1 Add clap dependency to Cargo.toml with derive feature
- [x] 1.2 Create `src/agent/mod.rs` module structure
- [x] 1.3 Update `src/main.rs` to dispatch between TUI and agent subcommand

## 2. CLI Framework

- [x] 2.1 Define CLI structure with clap derive in `src/agent/cli.rs`
- [x] 2.2 Add agent subcommand with helpful --help text (warn: not for human use)
- [x] 2.3 Implement session, context, task, status, learn subcommands skeleton

## 3. Session Management

- [x] 3.1 Create `src/agent/session.rs` with Session struct and state file format
- [x] 3.2 Implement `session init` command (create session, acquire lock)
- [x] 3.3 Implement `session next-story` command (get next incomplete story)
- [x] 3.4 Implement `session flush` command (write learnings, release lock)
- [x] 3.5 Add RALPH_SESSION environment variable validation
- [x] 3.6 Add RALPH_STORY environment variable handling

## 4. Context Retrieval

- [x] 4.1 Create `src/agent/context.rs` module
- [x] 4.2 Implement `context` command returning story-scoped context JSON
- [x] 4.3 Include proposal, design, scenarios, learnings in context response
- [x] 4.4 Include verification commands (infer from Cargo.toml)

## 5. Task Operations

- [x] 5.1 Create `src/agent/tasks.rs` module
- [x] 5.2 Implement `task done` command with task ID validation
- [x] 5.3 Add story scope validation (task must be in current story)
- [x] 5.4 Implement atomic task update with file locking
- [x] 5.5 Implement `status` command returning progress JSON

## 6. Progress Tracking

- [x] 6.1 Create `src/agent/progress.rs` module
- [x] 6.2 Implement `learn` command storing learning in session state
- [x] 6.3 Add --task flag support for task-referenced learnings
- [x] 6.4 Implement learning flush to design.md in session flush

## 7. Integration and Polish

- [x] 7.1 Add JSON output formatting for all commands
- [x] 7.2 Add consistent error response format
- [x] 7.3 Write integration tests for happy path
- [x] 7.4 Test session isolation with multiple concurrent sessions
