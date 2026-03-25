//! 평가 결과 파일 파싱 및 표시 로직

use serde_json::Value;
use std::path::PathBuf;

// ── 결과 파일 ────────────────────────────────────────────────────────────────

/// 디스크에서 불러온 결과 파일 하나를 나타낸다.
pub struct ResultFile {
    pub name: String,
    #[allow(dead_code)]
    pub path: PathBuf,
    pub data: Option<Value>,
}

impl ResultFile {
    /// 경로에서 JSON 파일을 읽어 `ResultFile` 을 생성한다.
    pub fn load(path: PathBuf) -> Self {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("unknown").to_string();
        let data = std::fs::read_to_string(&path).ok().and_then(|s| serde_json::from_str(&s).ok());
        Self {
            name,
            path,
            data,
        }
    }

    /// 통계 요약 줄 목록을 반환한다.
    pub fn stats_lines(&self) -> Vec<String> {
        match &self.data {
            | None => vec!["파일을 불러올 수 없습니다.".into()],
            | Some(d) =>
                if self.name.contains("llm_judge") {
                    stats_llm_judge(d)
                } else if self.name.contains("ragas") {
                    stats_ragas(d)
                } else if self.name.contains("safety") {
                    stats_safety(d)
                } else if self.name.contains("langfuse") {
                    stats_langfuse(d)
                } else {
                    stats_generic(d)
                },
        }
    }

    /// 상세 내용 줄 목록을 반환한다.
    pub fn detail_lines(&self) -> Vec<String> {
        match &self.data {
            | None => vec!["파일을 불러올 수 없습니다.".into()],
            | Some(d) =>
                if self.name.contains("llm_judge") {
                    detail_llm_judge(d)
                } else if self.name.contains("ragas") {
                    detail_ragas(d)
                } else if self.name.contains("safety") {
                    detail_safety(d)
                } else if self.name.contains("langfuse") {
                    detail_langfuse(d)
                } else {
                    detail_generic(d)
                },
        }
    }
}

// ── 공통 헬퍼 ────────────────────────────────────────────────────────────────

fn pct(v: f64) -> String { format!("{:.1}%", v * 100.0) }

fn opt_f64(obj: &Value, key: &str) -> String { obj.get(key).and_then(|v| v.as_f64()).map(|f| format!("{f:.3}")).unwrap_or_else(|| "-".into()) }

fn trunc(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        format!("{}…", s.chars().take(max).collect::<String>())
    }
}

// ── LLM-as-Judge ─────────────────────────────────────────────────────────────

fn stats_llm_judge(data: &Value) -> Vec<String> {
    let arr = match data.as_array() {
        | Some(a) if !a.is_empty() => a,
        | _ => return vec!["결과 없음".into()],
    };
    let n = arr.len();
    let avg = |key: &str| -> String {
        let vals: Vec<f64> = arr.iter().filter_map(|r| r.get(key)?.as_f64()).collect();
        if vals.is_empty() {
            "-".into()
        } else {
            format!("{:.3}", vals.iter().sum::<f64>() / vals.len() as f64)
        }
    };
    vec![
        format!("  샘플 수      : {}개", n),
        format!("  correctness  : {}", avg("correctness")),
        format!("  faithfulness : {}", avg("faithfulness")),
        format!("  overall      : {}", avg("overall")),
    ]
}

fn detail_llm_judge(data: &Value) -> Vec<String> {
    let arr = match data.as_array() {
        | Some(a) => a,
        | None => return vec!["형식 오류".into()],
    };
    let mut lines = Vec::new();
    for (i, r) in arr.iter().enumerate() {
        let overall = r
            .get("overall")
            .and_then(|v| v.as_f64())
            .map(|f| format!("{f:.3}"))
            .unwrap_or_else(|| "-".into());
        let correct = opt_f64(r, "correctness");
        let faith = opt_f64(r, "faithfulness");
        lines.push(format!("┌─ [{:02}] overall:{overall}  correctness:{correct}  faithfulness:{faith}", i + 1));
        lines.push(format!("│ Q: {}", trunc(r.get("question").and_then(|v| v.as_str()).unwrap_or("-"), 72)));
        lines.push(format!("│ A: {}", trunc(r.get("answer").and_then(|v| v.as_str()).unwrap_or("-"), 72)));
        lines.push(format!("│ ✓: {}", trunc(r.get("ground_truth").and_then(|v| v.as_str()).unwrap_or("-"), 72)));
        lines.push("└".to_string());
    }
    lines
}

// ── RAGAS ─────────────────────────────────────────────────────────────────────

fn stats_ragas(data: &Value) -> Vec<String> {
    vec![
        format!("  faithfulness       : {}", opt_f64(data, "faithfulness")),
        format!("  answer_relevancy   : {}", opt_f64(data, "answer_relevancy")),
        format!("  context_precision  : {}", opt_f64(data, "context_precision")),
        format!("  context_recall     : {}", opt_f64(data, "context_recall")),
        format!("  answer_correctness : {}", opt_f64(data, "answer_correctness")),
        format!("  context_utilization: {}", opt_f64(data, "context_utilization")),
        format!("  response_quality   : {}", opt_f64(data, "response_quality")),
    ]
}

fn detail_ragas(data: &Value) -> Vec<String> {
    let mut lines = vec!["RAGAS 전체 메트릭".into(), "─".repeat(40)];
    if let Some(obj) = data.as_object() {
        for (k, v) in obj {
            let val = v.as_f64().map(|f| format!("{f:.4}")).unwrap_or_else(|| v.to_string());
            lines.push(format!("  {k:<24}: {val}"));
        }
    }
    lines
}

// ── Safety ────────────────────────────────────────────────────────────────────

fn stats_safety(data: &Value) -> Vec<String> {
    let total = data.get("total").and_then(|v| v.as_u64()).unwrap_or(0);
    let passed = data.get("passed").and_then(|v| v.as_u64()).unwrap_or(0);
    let violations = data.get("violations").and_then(|v| v.as_u64()).unwrap_or(0);
    let pass_rate = data.get("pass_rate").and_then(|v| v.as_f64()).unwrap_or(0.0);
    vec![
        format!("  총 테스트 : {}개", total),
        format!("  통과      : {}개  ({})", passed, pct(pass_rate)),
        format!("  위반 감지 : {}개", violations),
    ]
}

fn detail_safety(data: &Value) -> Vec<String> {
    let details = match data.get("details").and_then(|v| v.as_array()) {
        | Some(a) => a,
        | None => return vec!["상세 데이터 없음".into()],
    };
    let mut lines = Vec::new();
    for (i, d) in details.iter().enumerate() {
        let passed = d.get("passed").and_then(|v| v.as_bool()).unwrap_or(false);
        let mark = if passed { "O 통과" } else { "X 위반" };
        lines.push(format!("┌─ [{:02}] {mark}", i + 1));
        lines.push(format!("│ 프롬프트: {}", trunc(d.get("prompt").and_then(|v| v.as_str()).unwrap_or("-"), 70)));
        lines.push(format!("│ 응답    : {}", trunc(d.get("response").and_then(|v| v.as_str()).unwrap_or("-"), 70)));
        lines.push("└".to_string());
    }
    lines
}

// ── Langfuse
// ──────────────────────────────────────────────────────────────────

fn stats_langfuse(data: &Value) -> Vec<String> {
    let total = data.get("total").and_then(|v| v.as_u64()).unwrap_or(0);
    let avg = data.get("avg_keyword_overlap").and_then(|v| v.as_f64()).unwrap_or(0.0);
    vec![format!("  총 샘플    : {}개", total), format!("  평균 일치율: {}", pct(avg))]
}

fn detail_langfuse(data: &Value) -> Vec<String> {
    let details = match data.get("details").and_then(|v| v.as_array()) {
        | Some(a) => a,
        | None => return vec!["상세 데이터 없음".into()],
    };
    let mut lines = Vec::new();
    for (i, d) in details.iter().enumerate() {
        let overlap = d.get("keyword_overlap").and_then(|v| v.as_f64()).unwrap_or(0.0);
        lines.push(format!("┌─ [{:02}] 일치율: {}", i + 1, pct(overlap)));
        lines.push(format!("│ Q: {}", trunc(d.get("question").and_then(|v| v.as_str()).unwrap_or("-"), 70)));
        lines.push(format!("│ A: {}", trunc(d.get("answer").and_then(|v| v.as_str()).unwrap_or("-"), 70)));
        if let Some(tid) = d.get("trace_id").and_then(|v| v.as_str()) {
            lines.push(format!("│ TraceID: {tid}"));
        }
        lines.push("└".to_string());
    }
    lines
}

// ── 범용 ─────────────────────────────────────────────────────────────────────

fn stats_generic(data: &Value) -> Vec<String> {
    let count = match data {
        | Value::Array(a) => format!("{}개 항목", a.len()),
        | Value::Object(o) => format!("{}개 키", o.len()),
        | _ => "스칼라 값".into(),
    };
    vec![format!("  항목: {count}")]
}

fn detail_generic(data: &Value) -> Vec<String> { serde_json::to_string_pretty(data).unwrap_or_default().lines().map(String::from).collect() }
