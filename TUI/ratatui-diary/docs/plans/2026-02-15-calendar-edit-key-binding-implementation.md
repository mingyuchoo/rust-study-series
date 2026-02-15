# 달력 편집 키 바인딩 변경 Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 달력 화면에서 편집 모드 진입 키를 Enter에서 'e'로 변경합니다.

**Architecture:** `src/main.rs`의 `handle_calendar_key` 함수에서 단일 라인 키 바인딩을 변경하는 간단한 수정입니다. 메시지 처리 로직이나 다른 컴포넌트는 변경하지 않습니다.

**Tech Stack:** Rust, Crossterm (키 이벤트 처리)

---

## Task 1: 키 바인딩 변경

**Files:**
- Modify: `src/main.rs:109`

**Step 1: 현재 코드 확인**

파일 위치: `src/main.rs:104-113`

현재 코드:
```rust
// Normal 키
match key.code {
    | KeyCode::Char('h') => Some(Msg::CalendarMoveLeft),
    | KeyCode::Char('l') => Some(Msg::CalendarMoveRight),
    | KeyCode::Char('k') => Some(Msg::CalendarMoveUp),
    | KeyCode::Char('j') => Some(Msg::CalendarMoveDown),
    | KeyCode::Enter => Some(Msg::CalendarSelectDate),
    | KeyCode::Char(' ') => Some(Msg::CalendarEnterSpaceMode),
    | KeyCode::Char('q') => Some(Msg::Quit),
    | _ => None,
}
```

**Step 2: 키 바인딩 변경**

109번째 줄을 다음과 같이 수정:

변경 전:
```rust
| KeyCode::Enter => Some(Msg::CalendarSelectDate),
```

변경 후:
```rust
| KeyCode::Char('e') => Some(Msg::CalendarSelectDate),
```

**Step 3: 빌드 확인**

Run: `cargo build`
Expected: 빌드 성공, 경고 없음

**Step 4: 수동 테스트 - 'e' 키 동작 확인**

Run: `cargo run`

테스트 시나리오:
1. 애플리케이션 실행
2. 달력 화면에서 날짜 선택 (h/j/k/l 키 사용)
3. 'e' 키 입력
4. **Expected:** 에디터 화면으로 전환, 선택한 날짜의 일기 편집 모드 진입

**Step 5: 수동 테스트 - Enter 키 반응 없음 확인**

테스트 시나리오:
1. Esc 키로 달력 화면으로 돌아가기
2. 달력 화면에서 Enter 키 입력
3. **Expected:** 아무 반응 없음, 여전히 달력 화면 유지

**Step 6: 수동 테스트 - 에디터에서 'e' 키 동작 확인**

테스트 시나리오:
1. 'e' 키로 에디터 진입
2. Normal 모드에서 'e' 키 입력
3. **Expected:** 커서가 현재 단어의 끝으로 이동 (기존 기능 유지)

**Step 7: 커밋**

```bash
git add src/main.rs
git commit -m "feat: change calendar edit key from Enter to 'e'

달력 화면에서 편집 모드 진입 키 변경
- Enter 키 제거
- 'e' 키로 편집 시작 (Vim/Helix 스타일)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 2: 전체 테스트 실행

**Files:**
- Test: 기존 테스트 슈트 전체

**Step 1: 모든 테스트 실행**

Run: `cargo test`
Expected: 모든 테스트 통과

**Step 2: 테스트 결과 확인**

- 기존 integration 테스트들이 여전히 통과하는지 확인
- 키 바인딩 변경이 다른 기능에 영향을 주지 않았는지 확인

**Step 3: 커밋 (필요시)**

만약 테스트 수정이 필요했다면:
```bash
git add tests/
git commit -m "test: update tests for new calendar key binding"
```

---

## Completion Checklist

- [ ] 'e' 키로 편집 모드 진입 작동
- [ ] Enter 키는 달력에서 반응 없음
- [ ] 에디터의 'e' 키 기능 정상 작동 (단어 끝 이동)
- [ ] 모든 기존 테스트 통과
- [ ] 빌드 경고 없음
- [ ] 변경사항 커밋 완료

---

## Notes

- 이 변경은 매우 간단한 키 바인딩 수정이므로 TDD보다는 직접 변경 후 검증 방식이 적합합니다.
- `handle_calendar_key` 함수는 private이므로 직접적인 유닛 테스트 추가가 어렵습니다.
- 수동 테스트로 충분히 검증 가능합니다.
- 기존 integration 테스트들이 키 바인딩에 의존하지 않으므로 테스트 수정이 불필요합니다.
