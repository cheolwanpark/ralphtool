## Context

Ralph Iteration runs Stories sequentially, but each Story's Agent starts fresh without knowledge of what previous Stories learned. The file system is already used for Story-to-Story information passing (tasks.md state). We extend this pattern with a dedicated learnings file.

## Goals / Non-Goals

**Goals:**
- Enable cumulative knowledge sharing across Stories within an Iteration
- Persist learnings across multiple Iteration runs for the same change
- Keep prompt additions minimal when no learnings exist

**Non-Goals:**
- Structured parsing of learnings content (free-form markdown)
- Automatic summarization or pruning of old learnings
- Cross-change learnings sharing

## Decisions

### Decision: File location `/tmp/ralphtool/{change}-learnings.md`

Use system temp directory with a dedicated `ralphtool` subdirectory.

**Rationale:**
- Keeps learnings out of git and project directory
- `ralphtool` subdirectory groups all Ralph temp files
- Change name in filename enables parallel work on different changes
- `/tmp` is standard and cleared on reboot

**Alternatives considered:**
- `openspec/changes/{change}/learnings.md` - Would be tracked by git, adds noise to change artifacts
- `.ralph/learnings.md` - Requires .gitignore management, project pollution

### Decision: Create file at iteration start only if missing

The Orchestrator ensures the learnings file exists before the first Story runs, but does not overwrite existing content.

**Rationale:**
- Simple lifecycle: create once, never delete
- Allows learnings to accumulate across iteration failures and retries
- No complex state management needed

### Decision: Free-form markdown with guidance

Learnings are free-form markdown. The initial file template provides guidance on what to record (discoveries, decisions, gotchas) via a comment block.

**Rationale:**
- Agents write naturally without format constraints
- No parsing overhead - content is included verbatim in prompts
- Guidance via comment doesn't pollute actual content

**Alternatives considered:**
- Structured YAML/JSON - Harder for Agent to write, requires parsing
- Story-headed sections - Adds complexity without clear benefit

### Decision: Include learnings in prompt only when content exists

The PromptBuilder checks if learnings file has content beyond the initial template. If empty or missing, the learnings section is omitted entirely.

**Rationale:**
- Story 1 of a fresh change doesn't need a "no learnings yet" message
- Keeps prompts focused and avoids noise
- Simple content check (non-whitespace after template header)

### Decision: Learnings section placement in prompt

Add learnings section after story context but before verification instructions.

**Rationale:**
- Context flows naturally: what to do → what we know → how to verify
- Not buried at the end where it might be missed

## Risks / Trade-offs

**Risk: Learnings file grows unbounded**
→ Mitigation: `/tmp` is cleared on reboot. For long-running work, user can manually clear the file. Future: add pruning if needed.

**Risk: Agent doesn't write useful learnings**
→ Mitigation: Provide clear guidance in prompt about what to record. Trust the Agent to self-improve over iterations.

**Risk: Stale learnings from failed iterations mislead later attempts**
→ Mitigation: Agent can overwrite or correct learnings. The persistence is intentional - even failed attempts contain useful information.
