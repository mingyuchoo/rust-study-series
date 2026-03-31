// =============================================================================
// @trace SPEC-001
// @trace PRD: PRD-001
// @trace FR: FR-1
// @trace file-type: test
// =============================================================================

use chrono::TimeZone;
use chrono::Utc;
use world_clock::clock::{ClockDisplay, format_clocks, get_clock_display};
use world_clock::error::AppError;

/// @trace TC: SPEC-001/TC-1
/// @trace FR: PRD-001/FR-1
/// @trace scenario: 유효한 타임존의 시간 표시
#[test]
fn test_tc1_valid_timezone_display() {
    let now = Utc.with_ymd_and_hms(2026, 3, 31, 6, 30, 0).unwrap();
    let display = get_clock_display("Seoul", "Asia/Seoul", now).unwrap();

    assert_eq!(display.city, "Seoul");
    assert_eq!(display.timezone, "Asia/Seoul");
    assert_eq!(display.time, "2026-03-31 15:30:00");
    assert_eq!(display.utc_offset, "+09:00");
}

/// @trace TC: SPEC-001/TC-2
/// @trace FR: PRD-001/FR-1
/// @trace scenario: 잘못된 타임존 에러 처리
#[test]
fn test_tc2_invalid_timezone_error() {
    let now = Utc.with_ymd_and_hms(2026, 3, 31, 6, 30, 0).unwrap();
    let result = get_clock_display("Test", "Invalid/Zone", now);

    assert!(result.is_err());
    assert!(matches!(result, Err(AppError::UnknownTimezone(_))));
}

/// @trace TC: SPEC-001/TC-3
/// @trace FR: PRD-001/FR-1
/// @trace scenario: 여러 도시의 시간 포맷 출력
#[test]
fn test_tc3_format_multiple_clocks() {
    let displays = vec![
        ClockDisplay {
            city: "Seoul".to_string(),
            timezone: "Asia/Seoul".to_string(),
            time: "2026-03-31 15:30:00".to_string(),
            utc_offset: "+09:00".to_string(),
        },
        ClockDisplay {
            city: "New York".to_string(),
            timezone: "America/New_York".to_string(),
            time: "2026-03-31 02:30:00".to_string(),
            utc_offset: "-04:00".to_string(),
        },
    ];

    let output = format_clocks(&displays);
    assert!(output.contains("Seoul"));
    assert!(output.contains("Asia/Seoul"));
    assert!(output.contains("15:30:00"));
    assert!(output.contains("+09:00"));
    assert!(output.contains("New York"));
    assert!(output.contains("America/New_York"));
    assert!(output.contains("02:30:00"));
    assert!(output.contains("-04:00"));
}

/// @trace TC: SPEC-001/TC-4
/// @trace FR: PRD-001/FR-1
/// @trace scenario: 빈 도시 목록 시 안내 메시지
#[test]
fn test_tc4_format_empty_clocks() {
    let output = format_clocks(&[]);
    assert!(output.contains("No cities configured."));
}
