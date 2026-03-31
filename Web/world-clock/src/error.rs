// =============================================================================
// @trace SPEC-001
// @trace PRD: PRD-001
// @trace FR: FR-1, FR-2, FR-3, FR-4
// @trace file-type: impl
// =============================================================================

use thiserror::Error;

/// 애플리케이션 에러 타입.
///
/// @trace SPEC: SPEC-001
/// @trace FR: PRD-001/FR-1, PRD-001/FR-2, PRD-001/FR-3
#[derive(Debug, Error)]
pub enum AppError {
    /// 알 수 없는 타임존 문자열.
    #[error("알 수 없는 타임존: {0}")]
    UnknownTimezone(String),

    /// 이미 존재하는 도시를 추가하려고 시도.
    #[error("이미 존재하는 도시: {0}")]
    DuplicateCity(String),

    /// 존재하지 않는 도시를 삭제하려고 시도.
    #[error("존재하지 않는 도시: {0}")]
    CityNotFound(String),

    /// 설정 파일 I/O 에러.
    #[error("설정 파일 에러: {0}")]
    Config(#[from] std::io::Error),

    /// JSON 파싱 에러.
    #[error("JSON 파싱 에러: {0}")]
    Json(#[from] serde_json::Error),
}
