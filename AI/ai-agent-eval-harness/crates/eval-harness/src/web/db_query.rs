// =============================================================================
// @trace SPEC-021
// @trace PRD: PRD-021
// @trace FR: PRD-021/FR-4
// @trace file-type: impl
// =============================================================================
//
// 동기 컨텍스트(파일 read 함수, 테스트 헬퍼 등)에서 SqliteStore 의 SPEC-021
// 조회 메서드를 호출하기 위한 얇은 어댑터. 전역 install 된 store 가 있을
// 때만 동작하며, 없으면 None 을 반환한다.

use data_scenarios::sqlite_store::{EvaluationListRow,
                                   TrajectoryListRow};

fn block_on<F: std::future::Future>(fut: F) -> F::Output {
    match tokio::runtime::Handle::try_current() {
        | Ok(handle) => tokio::task::block_in_place(|| handle.block_on(fut)),
        | Err(_) => tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("failed to build tokio runtime")
            .block_on(fut),
    }
}

/// trajectories 행 메타데이터 목록. 전역 store 가 없으면 None.
pub fn list_trajectories() -> Option<Vec<TrajectoryListRow>> {
    let store = data_scenarios::loader::try_installed_store()?;
    match block_on(async move { store.list_trajectory_ids().await }) {
        | Ok(rows) => Some(rows),
        | Err(e) => {
            eprintln!("[warn] db_query.list_trajectories 실패: {e}");
            None
        },
    }
}

/// 단건 trajectory JSON. 미존재 시 None.
pub fn get_trajectory(task_id: &str) -> Option<serde_json::Value> {
    let store = data_scenarios::loader::try_installed_store()?;
    let task_id = task_id.to_string();
    match block_on(async move { store.get_trajectory_json(&task_id).await }) {
        | Ok(v) => v,
        | Err(e) => {
            eprintln!("[warn] db_query.get_trajectory 실패: {e}");
            None
        },
    }
}

/// evaluations 행 메타데이터 목록.
pub fn list_evaluations() -> Option<Vec<EvaluationListRow>> {
    let store = data_scenarios::loader::try_installed_store()?;
    match block_on(async move { store.list_evaluation_ids().await }) {
        | Ok(rows) => Some(rows),
        | Err(e) => {
            eprintln!("[warn] db_query.list_evaluations 실패: {e}");
            None
        },
    }
}

/// 단건 evaluation JSON.
pub fn get_evaluation(task_id: &str) -> Option<serde_json::Value> {
    let store = data_scenarios::loader::try_installed_store()?;
    let task_id = task_id.to_string();
    match block_on(async move { store.get_evaluation_json(&task_id).await }) {
        | Ok(v) => v,
        | Err(e) => {
            eprintln!("[warn] db_query.get_evaluation 실패: {e}");
            None
        },
    }
}

/// 파일명에서 task_id(36자 UUID) 추출. 형식:
/// `(trajectory|evaluation)_<task_id>_<YYYYMMDD_HHMMSS>.json`
pub fn parse_task_id_from_filename(name: &str) -> Option<String> {
    let stripped = name.strip_suffix(".json")?;
    // prefix 제거
    let body = stripped.strip_prefix("trajectory_").or_else(|| stripped.strip_prefix("evaluation_"))?;
    // body = "<task_id>_<YYYYMMDD_HHMMSS>"
    // task_id 는 UUID(36자 + 4 hyphens) 이므로 길이 36 자르기
    if body.len() < 36 {
        return None;
    }
    let id = &body[.. 36];
    if id.chars().all(|c| c.is_ascii_hexdigit() || c == '-') {
        Some(id.to_string())
    } else {
        None
    }
}

/// 파일명 형식으로 trajectory list row 를 표현. 기존 클라이언트가 파일명을
/// 기대하므로 호환을 위해 같은 패턴을 만든다.
pub fn trajectory_row_to_filename(row: &TrajectoryListRow) -> String {
    // started_at 은 RFC3339 (예: 2026-04-11T10:23:45Z). YYYYMMDD_HHMMSS 로 정리.
    let ts = compact_ts(&row.started_at);
    format!("trajectory_{}_{}.json", row.task_id, ts)
}

pub fn evaluation_row_to_filename(row: &EvaluationListRow) -> String {
    let ts = compact_ts(&row.created_at);
    format!("evaluation_{}_{}.json", row.task_id, ts)
}

fn compact_ts(s: &str) -> String {
    // RFC3339 또는 SQLite "YYYY-MM-DD HH:MM:SS" 모두 처리.
    let cleaned: String = s.chars().filter(|c| c.is_ascii_digit()).take(14).collect();
    if cleaned.len() == 14 {
        format!("{}_{}", &cleaned[.. 8], &cleaned[8 ..])
    } else {
        cleaned
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @trace TC: SPEC-021/TC-8
    #[test]
    fn parse_task_id_valid() {
        let name = "trajectory_550e8400-e29b-41d4-a716-446655440000_20260411_102345.json";
        assert_eq!(parse_task_id_from_filename(name).as_deref(), Some("550e8400-e29b-41d4-a716-446655440000"));
    }

    /// @trace TC: SPEC-021/TC-8
    #[test]
    fn parse_task_id_evaluation_prefix() {
        let name = "evaluation_550e8400-e29b-41d4-a716-446655440000_20260411_102345.json";
        assert_eq!(parse_task_id_from_filename(name).as_deref(), Some("550e8400-e29b-41d4-a716-446655440000"));
    }

    #[test]
    fn parse_task_id_invalid_returns_none() {
        assert!(parse_task_id_from_filename("not_a_log.json").is_none());
        assert!(parse_task_id_from_filename("trajectory_short_20260411.json").is_none());
        assert!(parse_task_id_from_filename("evaluation_report_20260411.json").is_none());
    }

    #[test]
    fn compact_ts_rfc3339() {
        assert_eq!(compact_ts("2026-04-11T10:23:45Z"), "20260411_102345");
    }

    #[test]
    fn compact_ts_sqlite_format() {
        assert_eq!(compact_ts("2026-04-11 10:23:45"), "20260411_102345");
    }
}
