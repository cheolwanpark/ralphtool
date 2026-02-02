## Purpose

Iterator-based streaming interface for coding agents, enabling real-time output processing during agent execution.

## Requirements

### Requirement: Agent returns streaming iterator

CodingAgent::run() SHALL return an AgentStream that implements Iterator<Item = StreamEvent>.

#### Scenario: Iterating over stream events
- **WHEN** caller iterates over AgentStream
- **THEN** each call to next() returns Some(StreamEvent) until completion, then None

### Requirement: StreamEvent distinguishes messages from completion

StreamEvent enum SHALL have two variants: Message(String) for intermediate output and Done(Response) for final result.

#### Scenario: Receiving intermediate message
- **WHEN** agent outputs intermediate text
- **THEN** iterator yields StreamEvent::Message containing the text

#### Scenario: Receiving final result
- **WHEN** agent completes execution
- **THEN** iterator yields StreamEvent::Done containing Response with content, turns, tokens, and cost

### Requirement: Prompt contains system and user components

Prompt struct SHALL have system (String) and user (String) fields for separate prompt components.

#### Scenario: Creating prompt with both components
- **WHEN** caller creates Prompt with system="You are helpful" and user="Fix the bug"
- **THEN** agent receives both prompts appropriately

### Requirement: Response contains execution metadata

Response struct SHALL include content (String), turns (u32), tokens (u32), and cost (f64) fields.

#### Scenario: Response with complete metadata
- **WHEN** agent completes after 3 turns using 1000 tokens costing $0.01
- **THEN** Response contains turns=3, tokens=1000, cost=0.01

### Requirement: ClaudeAgent uses streaming JSON output

ClaudeAgent SHALL invoke claude CLI with --output-format stream-json and --append-system-prompt flags.

#### Scenario: CLI invocation with correct flags
- **WHEN** ClaudeAgent::run() is called with Prompt
- **THEN** claude CLI is invoked with -p, --append-system-prompt, --output-format stream-json, --verbose

### Requirement: AgentStream parses NDJSON events

AgentStream SHALL parse newline-delimited JSON from claude CLI stdout and convert to StreamEvent.

#### Scenario: Parsing assistant message
- **WHEN** CLI outputs {"type":"assistant","message":{"content":[{"type":"text","text":"Hello"}]}}
- **THEN** iterator yields StreamEvent::Message("Hello")

#### Scenario: Parsing result event
- **WHEN** CLI outputs {"type":"result","result":"Done","num_turns":1,"total_cost_usd":0.01,"usage":{"input_tokens":100,"output_tokens":50}}
- **THEN** iterator yields StreamEvent::Done(Response{content:"Done",turns:1,tokens:150,cost:0.01})

### Requirement: AgentStream handles process lifecycle

AgentStream SHALL manage child process lifecycle and clean up on drop.

#### Scenario: Process cleanup on drop
- **WHEN** AgentStream is dropped before completion
- **THEN** child process is terminated
