// =============================================================================
// @trace SPEC-001
// @trace PRD: PRD-001
// @trace FR: FR-1, FR-2, FR-3, FR-4
// @trace file-type: impl
// =============================================================================

use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// 여러 타임존의 현재 시간을 표시하는 세계 시계 CLI 앱.
///
/// @trace SPEC: SPEC-001
/// @trace FR: PRD-001/FR-1, PRD-001/FR-2, PRD-001/FR-3, PRD-001/FR-4
#[derive(Parser)]
#[command(name = "world-clock", about = "여러 타임존의 현재 시간을 표시합니다")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// 설정 파일 경로 (기본값 대신 사용)
    #[arg(long, global = true)]
    pub config: Option<PathBuf>,
}

/// CLI 하위 명령어.
///
/// @trace SPEC: SPEC-001, SPEC-002
/// @trace FR: PRD-001/FR-1, PRD-001/FR-2, PRD-001/FR-3, PRD-001/FR-4, PRD-002/FR-5
#[derive(Subcommand)]
pub enum Commands {
    /// 도시를 추가한다
    Add {
        /// 도시 이름 (예: "Seoul")
        name: String,
        /// IANA 타임존 (예: "Asia/Seoul")
        timezone: String,
    },
    /// 도시를 삭제한다
    Remove {
        /// 삭제할 도시 이름
        name: String,
    },
    /// 저장된 도시 목록을 조회한다
    List,
    /// 웹 서버를 시작한다
    Serve {
        /// 서버 포트 (기본값: 3000)
        #[arg(long, default_value = "3000")]
        port: u16,
    },
}
