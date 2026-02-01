## 1. ScopedSession Implementation

- [x] 1.1 Create `ScopedSession` struct with session_id, change, and story_id fields
- [x] 1.2 Implement `ScopedSession::init(change: &str)` that creates session and acquires lock
- [x] 1.3 Implement `env(&self) -> HashMap<String, String>` returning RALPH_SESSION and RALPH_STORY
- [x] 1.4 Implement `next_story(&mut self) -> Result<Option<String>>` for story iteration
- [x] 1.5 Implement `flush(self, learnings: &[String]) -> Result<()>` for explicit cleanup
- [x] 1.6 Implement `Drop` trait to release lock and remove session file

## 2. Orchestrator Migration

- [x] 2.1 Update orchestrator to create `ScopedSession` at start of `run()`
- [x] 2.2 Replace direct adapter creation with session-based story iteration
- [x] 2.3 Update agent spawning to pass `session.env()` to AgentConfig

## 3. Testing

- [x] 3.1 Add unit tests for `ScopedSession::env()` output
- [x] 3.2 Add integration test verifying subprocess receives env vars
