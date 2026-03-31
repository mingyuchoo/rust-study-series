// =============================================================================
// @trace SPEC-001
// @trace PRD: PRD-001
// @trace FR: FR-1
// @trace file-type: impl
// =============================================================================

use std::fmt::Write;

use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use serde::Serialize;

use crate::error::AppError;

/// 단일 도시의 시계 표시 정보.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ClockDisplay {
    pub city: String,
    pub timezone: String,
    pub time: String,
    pub utc_offset: String,
}

/// 주어진 도시와 타임존에 대해 시계 표시 정보를 생성한다.
///
/// # Arguments
/// * `city` - 도시 이름
/// * `timezone` - IANA 타임존 문자열 (예: "Asia/Seoul")
/// * `now` - 기준 UTC 시각
///
/// # Returns
/// 해당 도시의 시계 표시 정보, 타임존이 유효하지 않으면 에러
///
/// @trace SPEC: SPEC-001
/// @trace TC: SPEC-001/TC-1, SPEC-001/TC-2
/// @trace FR: PRD-001/FR-1
pub fn get_clock_display(
    city: &str,
    timezone: &str,
    now: DateTime<Utc>,
) -> Result<ClockDisplay, AppError> {
    let tz: Tz = timezone
        .parse()
        .map_err(|_| AppError::UnknownTimezone(timezone.to_string()))?;
    let local_time = now.with_timezone(&tz);

    Ok(ClockDisplay {
        city: city.to_string(),
        timezone: timezone.to_string(),
        time: local_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        utc_offset: local_time.format("%:z").to_string(),
    })
}

/// 여러 도시의 시계 정보를 테이블 형식 문자열로 포맷한다.
///
/// @trace SPEC: SPEC-001
/// @trace TC: SPEC-001/TC-3, SPEC-001/TC-4
/// @trace FR: PRD-001/FR-1
pub fn format_clocks(displays: &[ClockDisplay]) -> String {
    if displays.is_empty() {
        return "No cities configured.\n".to_string();
    }

    let city_width = displays
        .iter()
        .map(|d| d.city.len())
        .max()
        .unwrap_or(4)
        .max(4);
    let tz_width = displays
        .iter()
        .map(|d| d.timezone.len())
        .max()
        .unwrap_or(8)
        .max(8);

    let mut output = String::new();
    writeln!(
        output,
        "  {:<city_width$}  {:<tz_width$}  {:<19}  UTC Offset",
        "City", "Timezone", "Time",
    )
    .unwrap();

    let line_len = city_width + tz_width + 19 + 10 + 8;
    writeln!(output, "  {}", "─".repeat(line_len)).unwrap();

    for d in displays {
        writeln!(
            output,
            "  {:<city_width$}  {:<tz_width$}  {:<19}  {}",
            d.city, d.timezone, d.time, d.utc_offset,
        )
        .unwrap();
    }

    output
}
