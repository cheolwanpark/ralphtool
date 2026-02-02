## Context

현재 `src/agent/` 모듈:
- `CodingAgent` trait: `fn run(&self, prompt: &str, config: &AgentConfig) -> Result<AgentOutput>`
- 동기적 실행, 완료까지 blocking
- `AgentConfig`: max_turns, timeout, env (더 이상 필요 없음)
- `AgentOutput`: result, session_id, usage

TUI에서 에이전트 실행 중 실시간 피드백이 필요. Claude CLI가 `--output-format stream-json`으로 NDJSON 스트리밍을 지원함.

## Goals / Non-Goals

**Goals:**
- Iterator 기반 스트리밍 인터페이스 제공
- 중간 메시지와 최종 결과(비용/토큰/턴) 구분
- 간단하고 직관적인 API

**Non-Goals:**
- 비동기(async) 지원 - 현재 불필요
- 다른 에이전트 백엔드 구현 - ClaudeAgent만 있음
- 세션 재개 기능 - 향후 필요시 추가

## Decisions

### 1. Iterator 기반 스트리밍

**결정**: `AgentStream`이 `Iterator<Item = StreamEvent>` 구현

**대안들**:
- Callback 기반: 간단하지만 caller가 blocking됨
- Channel 기반: 복잡도 증가
- Async Stream: 런타임 의존성 필요

**근거**: 코드 간단함 우선, std Iterator만으로 충분

### 2. StreamEvent enum

```rust
pub enum StreamEvent {
    Message(String),      // 중간 메시지
    Done(Response),       // 최종 결과
}
```

**근거**: finish() 별도 메서드 없이 Iterator만으로 완결

### 3. CLI 플래그

```bash
claude -p "<user>" \
  --append-system-prompt "<system>" \
  --output-format stream-json \
  --verbose
```

**결정**: `--append-system-prompt` 사용 (기본 프롬프트에 추가)

**대안**: `--system-prompt` (완전 교체) - Claude Code 기본 기능 손실 위험

### 4. JSON 이벤트 매핑

| CLI 이벤트 | StreamEvent |
|-----------|-------------|
| `type: "system"` | 무시 |
| `type: "assistant"` | `Message(content[0].text)` |
| `type: "result"` | `Done(Response)` |

### 5. 기존 코드 마이그레이션

- `PromptBuilder::for_story()` → `Prompt { system: "", user: ... }` 리턴
- 향후 system prompt 활용 가능하도록 구조만 준비

## Risks / Trade-offs

**[Blocking Iterator]** → Iterator::next()가 blocking이라 UI가 멈출 수 있음
- Mitigation: TUI에서 별도 thread로 실행

**[JSON 파싱 실패]** → CLI 출력 포맷 변경시 깨질 수 있음
- Mitigation: 에러 타입으로 graceful 처리

**[Child process 관리]** → 비정상 종료시 좀비 프로세스
- Mitigation: Drop impl에서 kill 처리
