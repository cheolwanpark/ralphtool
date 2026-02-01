## 1. Module Restructure

- [x] 1.1 Rename src/agent/ to src/session/
- [x] 1.2 Update all imports referencing agent module to session
- [x] 1.3 Create new src/agent/ directory for coding agent abstraction
- [x] 1.4 Create new src/loop/ directory for orchestration

## 2. Coding Agent Abstraction

- [x] 2.1 Define CodingAgent trait with run method in src/agent/mod.rs
- [x] 2.2 Define AgentConfig struct (allowed_tools, max_turns, timeout)
- [x] 2.3 Define AgentOutput struct (result, session_id, usage)
- [x] 2.4 Implement ClaudeAgent in src/agent/claude.rs
- [x] 2.5 Add Claude CLI invocation with -p and --output-format json
- [x] 2.6 Parse Claude JSON response into AgentOutput

## 3. Instruction Generation

- [x] 3.1 Add generate_instructions function to session module
- [x] 3.2 Build markdown prompt from Context (proposal, design, story, tasks, verify)
- [x] 3.3 Include available ralphtool commands in instructions

## 4. Loop Orchestration

- [x] 4.1 Define LoopEvent enum (StoryStarted, TaskCompleted, StoryCompleted, AgentOutput, Error, Complete)
- [x] 4.2 Define LoopState struct for tracking progress
- [x] 4.3 Implement Orchestrator with event channel
- [x] 4.4 Implement story iteration loop using SpecAdapter
- [x] 4.5 Parse agent output for task completions
- [x] 4.6 Call adapter.mark_done for completed tasks
- [x] 4.7 Flush learnings on loop completion

## 5. TUI Integration

- [x] 5.1 Create LoopScreen widget for displaying loop progress
- [x] 5.2 Subscribe to LoopEvent channel and update display
- [x] 5.3 Show current story, iteration count, and progress bar
- [x] 5.4 Handle 'q' key to stop loop
- [x] 5.5 Create ResultScreen widget for reviewing changes
- [x] 5.6 Display changed files and verification status in result screen
