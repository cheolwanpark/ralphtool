## Why

현재 `CodingAgent` trait 인터페이스가 동기적이고 스트리밍을 지원하지 않음. TUI에서 에이전트 실행 중 중간 메시지를 실시간으로 표시하고, 최종 결과에 비용/토큰/턴 정보를 포함해야 함.

## What Changes

- **BREAKING**: `CodingAgent::run()` 시그니처 변경 - `AgentConfig` 제거, `Prompt` 구조체 도입
- **BREAKING**: `AgentOutput` → `Response` 로 교체, 새 필드 구조 (content, turns, tokens, cost)
- `AgentStream` 구체 타입 추가 - Iterator 기반 스트리밍
- `StreamEvent` enum 추가 - `Message(String)` | `Done(Response)`
- `Prompt` 구조체 추가 - system prompt와 user prompt 분리
- `AgentConfig`, `TokenUsage` 타입 제거
- Claude CLI 호출 방식 변경: `--append-system-prompt`, `--output-format stream-json`

## Capabilities

### New Capabilities

- `agent-streaming`: Iterator 기반 스트리밍 인터페이스로 에이전트 출력을 실시간 수신

### Modified Capabilities

(없음 - 기존 spec 없음)

## Impact

- `src/agent/mod.rs`: trait 정의 및 타입 변경
- `src/agent/claude.rs`: ClaudeAgent 구현 전면 수정
- `src/agent/prompt.rs`: `PromptBuilder`가 새 `Prompt` 타입 사용하도록 수정
- 향후 TUI 코드에서 새 스트리밍 인터페이스 사용
