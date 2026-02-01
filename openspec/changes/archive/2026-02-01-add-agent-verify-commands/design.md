## Context

The agent CLI provides commands for the Ralph Loop implementation phase. A verification agent needs different context: all requirements and scenarios to verify the implementation is complete and correct. Currently:
- `ScenarioSource::scenarios_for(story_id)` returns empty because Story IDs (from tasks.md: "1", "2") don't match UserStory IDs (from specs: "session-init")
- No CLI command exposes UserStory/requirements data
- `StorySource::mark_passed()` exists but has no CLI command

## Goals / Non-Goals

**Goals:**
- Provide verification agent with all requirements and scenarios
- Allow marking stories as verified
- Keep implementation simple - reuse existing traits

**Non-Goals:**
- Linking tasks.md stories to specs (complex, not needed)
- Automated test execution (orchestrator's job)
- Verification reports (handled by opsx:verify skill)

## Decisions

### Decision: Return all scenarios in context (Option B)
Instead of fixing the ID mapping between Story and UserStory, return ALL scenarios in the context response. The verification agent can determine relevance.

**Rationale**: Simpler than adding explicit linking. The number of scenarios per change is small.

### Decision: Separate verify subcommand group
Add `ralphtool agent verify` with subcommands rather than adding to existing commands.

**Rationale**: Clean separation between implementation and verification concerns.

### Decision: Reuse existing traits
Use `StorySource::list_stories()` and `ScenarioSource::list_scenarios()` for verify context. Use `StorySource::mark_passed()` for verify pass.

**Rationale**: Traits already exist, just need CLI exposure.

## Risks / Trade-offs

- **All scenarios returned** → Slightly larger context payload. Acceptable for typical change sizes.
- **No story-scenario linking** → Verification agent must infer relevance. Acceptable given AI capability.
