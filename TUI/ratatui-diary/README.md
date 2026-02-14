# Ratatui Diary

터미널 기반 다이어리 애플리케이션 (Rust + Ratatui)

## 기능

- 📅 월간 달력 뷰
- ✍️ Vi 모드 텍스트 에디터
- 💾 Markdown 파일 자동 저장
- 🎨 다이어리 작성 유무 시각적 표시

## 설치

```bash
cargo build --release
cargo install --path .
```

## 사용법

```bash
ratatui-diary
```

### 달력 화면

| 키 | 동작 |
|---|---|
| `h/j/k/l` | 날짜 이동 |
| `H/L` | 연도 이동 |
| `Enter` | 다이어리 작성/편집 |
| `q` | 종료 |

### 에디터 화면

**Normal 모드:**
- `i`: Insert 모드
- `:w`: 저장
- `:q`: 나가기
- `:wq`: 저장 후 나가기
- `dd`: 다이어리 삭제
- `Esc`: 달력으로 돌아가기

**Insert 모드:**
- 텍스트 입력
- `Esc`: Normal 모드

## 데이터 저장

다이어리는 `~/.local/share/ratatui-diary/entries/` 디렉토리에 Markdown 파일로 저장됩니다.

파일명 형식: `YYYY-MM-DD.md`

## 아키텍처

ELM (Model-Update-View) 패턴 기반

- **Model**: 앱 상태
- **Update**: 상태 업데이트 로직
- **View**: UI 렌더링

## 개발

```bash
# 테스트 실행
cargo test

# 개발 모드 실행
cargo run
```

## 라이선스

MIT
