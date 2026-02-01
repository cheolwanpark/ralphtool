## Context

The ClaudeAgent invokes the Claude CLI to execute coding tasks. Currently, there's no way to skip permission prompts, which blocks fully autonomous execution in trusted environments like CI/CD pipelines or local development with Ralph loop.

## Goals / Non-Goals

**Goals:**
- Enable autonomous agent execution without permission prompts
- Maintain safe defaults (permissions required by default)
- Simple, minimal change to existing code

**Non-Goals:**
- Fine-grained permission control (beyond all-or-nothing)
- Persisting permission preferences across sessions

## Decisions

### Decision 1: Add boolean field to AgentConfig

Add `dangerously_skip_permissions: bool` to `AgentConfig` with default `false`.

**Rationale**: Simple, explicit field name that conveys the risk. Follows the CLI's naming convention (`--dangerously-skip-permissions`).

**Alternatives considered**:
- Enum for permission modes → Overkill for current needs, can extend later
- Separate config struct → Adds complexity for a single field

### Decision 2: Pass flag conditionally in ClaudeAgent

When `config.dangerously_skip_permissions` is `true`, add `--dangerously-skip-permissions` to the CLI command.

**Rationale**: Direct mapping from config to CLI flag. No transformation or validation needed.

## Risks / Trade-offs

- [Risk] Flag enables all tool execution without review → Mitigation: Clear naming, default false, only use in trusted contexts
- [Risk] User enables flag without understanding implications → Mitigation: Name includes "dangerously" to signal risk
