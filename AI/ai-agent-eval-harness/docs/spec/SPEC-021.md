# SPEC-021: 궤적/평가 로그 SQLite 저장소 이전

## 메타데이터
- SPEC ID: SPEC-021
- 관련 PRD: PRD-021
- 작성일: 2026-04-11
- 상태: Draft

## 개요
PRD-021 의 7개 FR 을 충족시키기 위해, 다음 설계를 적용한다:
1. SqliteStore 에 `trajectories` / `evaluations` 두 테이블을 추가하는 v4 마이그레이션
2. `TrajectoryLogger` 트레이트화 + `FileLogger` / `SqliteLogger` / `MultiLogger` 세 구현체
3. 읽기 경로 함수가 DB 우선 + 파일 폴백
4. `compare` CLI 의 입력 모드 확장
5. `backfill-results` CLI 서브커맨드

## 데이터 모델

### 신규 테이블

```sql
-- v4: agent 실행 결과(궤적). steps/final_state 는 JSON BLOB 으로 저장.
CREATE TABLE IF NOT EXISTS trajectories (
    task_id           TEXT PRIMARY KEY,
    task_description  TEXT NOT NULL DEFAULT '',
    agent_name        TEXT NOT NULL DEFAULT '',
    domain            TEXT,
    scenario_id       TEXT,
    success           INTEGER NOT NULL DEFAULT 0,
    total_iterations  INTEGER NOT NULL DEFAULT 0,
    started_at        TEXT NOT NULL,
    ended_at          TEXT,
    steps_json        TEXT NOT NULL,
    final_state_json  TEXT,
    created_at        TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_trajectories_agent     ON trajectories(agent_name);
CREATE INDEX IF NOT EXISTS idx_trajectories_domain    ON trajectories(domain);
CREATE INDEX IF NOT EXISTS idx_trajectories_started   ON trajectories(started_at);

-- v4: 평가 점수. trajectory 1:1 매핑.
CREATE TABLE IF NOT EXISTS evaluations (
    task_id                  TEXT PRIMARY KEY,
    agent_name               TEXT NOT NULL DEFAULT '',
    domain                   TEXT,
    scenario_id              TEXT,
    success                  INTEGER NOT NULL DEFAULT 0,
    criteria_score           REAL,
    tool_sequence_score      REAL,
    domain_routing_score     REAL,
    overall_score            REAL,
    metrics_json             TEXT NOT NULL DEFAULT '{}',
    golden_set_result_json   TEXT,
    created_at               TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (task_id) REFERENCES trajectories(task_id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_evaluations_agent     ON evaluations(agent_name);
CREATE INDEX IF NOT EXISTS idx_evaluations_domain    ON evaluations(domain);
CREATE INDEX IF NOT EXISTS idx_evaluations_created   ON evaluations(created_at);
```

`SCHEMA_VERSION` 상수를 4로 올린다. 마이그레이션은 `CREATE TABLE IF NOT EXISTS` 만 실행하면 충분(파괴적 변경 없음).

### Rust 설계

```rust
// crates/reporting/src/logger.rs
pub trait TrajectoryLog: Send + Sync {
    fn save_trajectory(&self, t: &Trajectory) -> Result<()>;
    fn save_evaluation(&self, e: &EvaluationResult) -> Result<()>;
}

pub struct FileLogger { /* 기존 구현 */ }
impl TrajectoryLog for FileLogger { ... }

pub struct SqliteLogger { /* SqliteStore Arc 보유 */ }
impl TrajectoryLog for SqliteLogger { ... }

pub struct MultiLogger { children: Vec<Box<dyn TrajectoryLog>> }
impl TrajectoryLog for MultiLogger { /* fan-out, 각 child 실패는 stderr 경고 */ }
```

기존 `TrajectoryLogger` (struct) 는 **하위호환 facade** 로 남겨둔다 — 내부에서 `MultiLogger { FileLogger, SqliteLogger? }` 를 들고 동일 메서드 시그니처를 노출한다. 이렇게 하면 `HarnessRunner::new(output_dir)` 같은 기존 호출부는 변경 없이 dual-write 가 활성화된다.

### task_id ↔ 파일명 매핑

기존 파일명 형식: `trajectory_<task_id>_<YYYYMMDD_HHMMSS>.json`, `evaluation_<task_id>_<YYYYMMDD_HHMMSS>.json`.

읽기 측에서 파일명 → task_id 추출은 다음 정규 표현으로:
```
^(trajectory|evaluation)_([0-9a-f-]{36})_(\d{8}_\d{6})\.json$
```

DB 행을 파일명으로 표현할 때는 `created_at` 을 `%Y%m%d_%H%M%S` 로 포맷한다(원본과 동일).

## 테스트 계획

| TC ID | 시나리오 | 입력 | 기대 | 유형 | FR |
|-------|--------|------|------|------|-----|
| TC-1 | v4 마이그레이션 멱등 | 빈 DB → init_schema 2회 | trajectories/evaluations 테이블 1개씩, 에러 없음 | unit | NFR-1 |
| TC-2 | trajectory insert | sample Trajectory → SqliteLogger.save_trajectory | trajectories 테이블 1행, steps_json valid JSON | unit | FR-1 |
| TC-3 | evaluation insert + FK | 동일 task_id 의 trajectory + evaluation | evaluations 테이블 1행, FK 유효 | unit | FR-2 |
| TC-4 | upsert (동일 task_id 재기록) | save_trajectory 2회 동일 task_id | 행 1개만 존재 (REPLACE) | unit | FR-1 |
| TC-5 | dual-write file + db | MultiLogger.save_trajectory | 파일 1개 + DB 행 1개 | unit | FR-3 |
| TC-6 | DB 우선 read, 파일 폴백 | DB 채워짐, 파일 비어있음 | list_trajectories_impl 가 DB 행 반환 | unit | FR-4 |
| TC-7 | DB 비어있을 때 파일 폴백 | DB 비어있음, 파일 1개 | list_trajectories_impl 가 파일명 반환 | unit | FR-4 |
| TC-8 | get_trajectory_impl 파일명 → task_id 변환 | 정상 파일명 입력 | DB 행 발견 시 본문 반환 | unit | FR-4 |
| TC-9 | compare --baseline-task / --current-task | 두 task_id | EvaluationReport 비교 결과 | integration | FR-6 |
| TC-10 | compare --agent --since --until | 시간 범위 + agent | 평균 메트릭 비교 | integration | FR-6 |
| TC-11 | backfill-results | dir 안 trajectory_*.json N개 | DB 에 N행 INSERT, 부분 실패 무시 | integration | FR-7 |
| TC-12 | dual-write 실패 격리 | DB 에러 시뮬레이트 | 파일은 성공, stderr 경고 | unit | FR-3, NFR-2 |

본 SPEC 의 TC-1 ~ TC-8 은 단위 테스트로 우선 구현, TC-9 ~ TC-12 는 후속 통합 테스트 단계에서 보강.

## 구현 단계

1. **Stage 1 (스키마)**: `SqliteStore::init_schema` 에 v4 테이블 추가, `SCHEMA_VERSION = 4`.
2. **Stage 1b (CRUD)**: `insert_trajectory`, `insert_evaluation`, `list_trajectories`, `get_trajectory_by_id`, `list_evaluations`, `get_evaluation_by_id`, `compare_agent_window` (시간 범위 평균).
3. **Stage 2 (트레이트)**: `TrajectoryLog` 트레이트, `FileLogger`, `SqliteLogger`, `MultiLogger`. 기존 struct `TrajectoryLogger` 를 facade 로 재작성.
4. **Stage 3 (Dual-write 진입점)**: `HarnessRunner::new` 가 `ScenarioLoader::resolve()` 로 store 를 얻어 `MultiLogger` 구성. 실패 시 FileLogger 단독으로 폴백.
5. **Stage 4 (읽기 전환)**: `web/api_exec.rs::list_trajectories_impl`, `get_trajectory_impl`, `web/handlers.rs::list_reports_impl`, `get_report_impl` 가 DB 우선 + 파일 폴백.
6. **Stage 5 (Backfill)**: `cargo run -- backfill-results --from <dir>` 신규 CLI.
7. **Stage 6 (Compare 확장)**: `compare` 가 `--baseline-task`, `--current-task`, `--agent`, `--since`, `--until` 옵션 추가.

각 단계 종료 후 `cargo test --workspace` 통과 확인.
