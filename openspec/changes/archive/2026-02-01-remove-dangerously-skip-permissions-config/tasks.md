## 1. Simplify AgentConfig

- [x] 1.1 Remove `allowed_tools` field from `AgentConfig` struct in `src/agent/mod.rs`
- [x] 1.2 Remove `dangerously_skip_permissions` field from `AgentConfig` struct in `src/agent/mod.rs`
- [x] 1.3 Update `Default` impl for `AgentConfig` to only set `max_turns` and `timeout`

## 2. Update Claude Agent Implementation

- [x] 2.1 Remove `--allowedTools` logic from `build_command_args` in `src/agent/claude.rs`
- [x] 2.2 Change `--dangerously-skip-permissions` to always be added (remove conditional)

## 3. Update Tests

- [x] 3.1 Remove test `skip_permissions_flag_passed_when_enabled` (no longer conditional)
- [x] 3.2 Remove test `skip_permissions_flag_not_passed_when_disabled` (no longer conditional)
- [x] 3.3 Add test verifying `--dangerously-skip-permissions` is always present

## 4. Update Spec

- [x] 4.1 Update `openspec/specs/coding-agent/spec.md` to reflect simplified configuration
