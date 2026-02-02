## 1. Type Definitions

- [x] 1.1 Add Prompt struct (system, user fields) to mod.rs
- [x] 1.2 Add Response struct (content, turns, tokens, cost) to mod.rs
- [x] 1.3 Add StreamEvent enum (Message, Done variants) to mod.rs
- [x] 1.4 Remove AgentConfig and TokenUsage types from mod.rs

## 2. AgentStream Implementation

- [x] 2.1 Add AgentStream struct with Child process and BufReader
- [x] 2.2 Add JSON event parsing types (ClaudeSystemEvent, ClaudeAssistantEvent, ClaudeResultEvent)
- [x] 2.3 Implement Iterator<Item = StreamEvent> for AgentStream
- [x] 2.4 Implement Drop for AgentStream to kill child process

## 3. CodingAgent Trait Update

- [x] 3.1 Change CodingAgent::run() signature to take Prompt, return Result<AgentStream>
- [x] 3.2 Update ClaudeAgent::run() to spawn process with stream-json flags
- [x] 3.3 Update ClaudeAgent::run() to use --append-system-prompt for system prompt

## 4. PromptBuilder Migration

- [x] 4.1 Change PromptBuilder::for_story() to return Prompt instead of String
- [x] 4.2 Set system="" and user=<generated prompt> for now

## 5. Cleanup and Tests

- [x] 5.1 Remove old AgentConfig, AgentOutput, TokenUsage types
- [x] 5.2 Update/add unit tests for JSON parsing
- [x] 5.3 Verify cargo check, cargo clippy, cargo test pass
