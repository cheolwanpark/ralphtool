## Context

Ralph is an autonomous AI development tool that uses Claude Code as its coding agent. The current `AgentConfig` struct includes `dangerously_skip_permissions` and `allowed_tools` fields that were added for potential flexibility, but in practice:

1. Ralph always needs full permissions to operate autonomously
2. Tool restrictions would break the autonomous workflow
3. The configuration adds complexity without providing value

The goal is to simplify by removing these fields and hardcoding the always-skip-permissions behavior.

## Goals / Non-Goals

**Goals:**
- Simplify `AgentConfig` by removing unused configuration options
- Always enable `--dangerously-skip-permissions` for autonomous operation
- Remove tool restriction capability entirely
- Reduce configuration surface area

**Non-Goals:**
- Adding new configuration options
- Changing the fundamental agent execution flow
- Modifying timeout or max_turns behavior

## Decisions

### Decision 1: Remove fields from AgentConfig rather than deprecate

**Choice**: Remove `dangerously_skip_permissions` and `allowed_tools` fields entirely.

**Rationale**: These fields have no current use case in Ralph's autonomous operation model. Deprecation would add complexity without benefit. A clean removal is simpler.

**Alternatives considered**:
- Deprecate with warnings: Adds noise without value since the codebase is young
- Keep as no-op: Confusing for future developers

### Decision 2: Hardcode --dangerously-skip-permissions in build_command_args

**Choice**: Always include `--dangerously-skip-permissions` flag without condition.

**Rationale**: Ralph requires autonomous operation. Making this unconditional removes a potential source of misconfiguration.

### Decision 3: Remove --allowedTools logic entirely

**Choice**: Never pass `--allowedTools` to Claude CLI.

**Rationale**: Tool restrictions would prevent Ralph from completing autonomous tasks. The agent needs full tool access.

## Risks / Trade-offs

**[Reduced flexibility]** → Acceptable for Ralph's use case. If future needs arise for restricted operation, the fields can be re-added.

**[Breaking change for any code using these fields]** → Compile-time error will catch all usages. No runtime surprises.

**[Tests need updating]** → Tests for conditional flag behavior should be removed entirely, not modified.
