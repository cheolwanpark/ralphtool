## 1. Update AgentConfig

- [x] 1.1 Add `dangerously_skip_permissions: bool` field to AgentConfig struct in `src/agent/mod.rs`
- [x] 1.2 Set default value to `false` in AgentConfig::default()

## 2. Update ClaudeAgent

- [x] 2.1 Add conditional `--dangerously-skip-permissions` flag in ClaudeAgent::run() in `src/agent/claude.rs`

## 3. Tests

- [x] 3.1 Add test verifying flag is passed when dangerously_skip_permissions is true
- [x] 3.2 Add test verifying flag is NOT passed when dangerously_skip_permissions is false
