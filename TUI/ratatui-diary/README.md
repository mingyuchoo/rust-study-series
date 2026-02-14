# Ratatui Diary

터미널 기반 다이어리 애플리케이션 (Rust + Ratatui)

## 기능

- 📅 월간 달력 뷰
- ✍️ Helix 스타일 모달 에디터 (Selection-first)
- 💾 Markdown 파일 자동 저장
- 🎨 다이어리 작성 유무 시각적 표시
- 👁️ 실시간 Markdown 미리보기 (달력 & 에디터)
- 🎯 Selection 하이라이트 (Helix 스타일)
- 🔍 검색 매치 하이라이트

### 미리보기 기능

- **달력 화면**: 선택된 날짜의 다이어리 내용을 오른쪽에 실시간으로 표시
- **에디터 화면**: 작성 중인 Markdown 문서를 렌더링하여 오른쪽에 표시
- **화면 분할**: 50:50 레이아웃으로 원본과 미리보기를 동시에 확인
- **고급 Markdown 지원**: 헤더, 굵게, 기울임, 코드 블록, 리스트, 인용구, 표, 링크 등

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
| `Space` | 명령 모드 |
| `Enter` | 다이어리 작성/편집 |
| `q` | 종료 |

**Space 명령 모드:**
- `n`: 다음 달
- `p`: 이전 달
- `y`: 다음 연도
- `Y`: 이전 연도
- `q`: 종료
- `Esc`: 명령 모드 취소

### 에디터 화면

**Normal 모드:**

*이동:*
- `h/j/k/l`: 좌/하/상/우 이동
- `w`: 다음 단어 시작
- `b`: 이전 단어 시작
- `e`: 단어 끝으로 이동

*Goto 모드 (g):*
- `gg`: 문서 시작
- `ge`: 문서 끝
- `gh`: 줄 시작
- `gl`: 줄 끝

*Insert 모드 진입:*
- `i`: 커서 위치에 삽입
- `a`: 커서 다음에 삽입
- `o`: 아래 줄 삽입
- `O`: 위 줄 삽입

*Selection (Helix 스타일):*
- `v`: Selection 토글
- `x`: 현재 줄 선택
- 이동 키로 Selection 확장

*편집:*
- `d`: 선택 영역 삭제 (selection 없으면 현재 줄)
- `c`: 선택 영역 삭제 후 Insert 모드
- `y`: 선택 영역 복사
- `p`: 커서 다음에 붙여넣기
- `P`: 커서 위치에 붙여넣기

*Undo/Redo:*
- `u`: Undo
- `U`: Redo

*검색:*
- `/`: 검색 모드
- `n`: 다음 매치
- `N`: 이전 매치

*Space 명령:*
- `Space w`: 저장
- `Space q`: 나가기
- `Space x`: 저장 후 나가기

*기타:*
- `Esc`: 달력으로 돌아가기

**Insert 모드:**
- 텍스트 입력
- `Esc`: Normal 모드
- `Enter`: 새 줄
- `Backspace`: 삭제

**시각적 피드백:**
- Selection 영역: 회색 배경으로 하이라이트
- 검색 매치: 노란색 배경으로 표시
- 현재 매치: 밝은 노란색 + 굵게
- Submode 표시: 상태바에 [GOTO], [SPACE], /검색어 표시

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
