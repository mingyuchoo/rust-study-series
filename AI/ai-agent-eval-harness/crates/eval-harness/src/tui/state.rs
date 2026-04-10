// =============================================================================
// @trace SPEC-001
// @trace PRD: PRD-001
// @trace FR: FR-2, FR-3, FR-4
// @trace file-type: impl
// =============================================================================

use crossterm::event::KeyCode;
use std::{fs,
          io,
          path::Path};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Scenarios,
    Reports,
}

#[derive(Debug)]
pub struct TuiState {
    pub scenarios: Vec<String>,
    pub reports: Vec<String>,
    pub scenario_idx: usize,
    pub report_idx: usize,
    pub focus: Focus,
    pub should_quit: bool,
}

impl TuiState {
    /// 시나리오/리포트 디렉토리를 스캔해 초기 상태를 만든다.
    ///
    /// @trace SPEC: SPEC-001
    /// @trace TC: TC-2, TC-4
    /// @trace FR: PRD-001/FR-2, PRD-001/FR-3
    pub fn new(scenarios_dir: &Path, reports_dir: &Path) -> io::Result<Self> {
        let scenarios = load_files(scenarios_dir, "yaml").unwrap_or_default();
        let reports = load_files(reports_dir, "json").unwrap_or_default();
        Ok(Self {
            scenarios,
            reports,
            scenario_idx: 0,
            report_idx: 0,
            focus: Focus::Scenarios,
            should_quit: false,
        })
    }

    /// 포커스된 리스트에서 다음 항목으로 이동.
    ///
    /// @trace SPEC: SPEC-001
    /// @trace TC: TC-3
    /// @trace FR: PRD-001/FR-2
    pub fn next(&mut self) {
        let (idx, len) = self.focused_list();
        if len == 0 {
            return;
        }
        let new_idx = (idx + 1) % len;
        self.set_focused_idx(new_idx);
    }

    /// 포커스된 리스트에서 이전 항목으로 이동.
    ///
    /// @trace SPEC: SPEC-001
    /// @trace TC: TC-3
    /// @trace FR: PRD-001/FR-2
    pub fn prev(&mut self) {
        let (idx, len) = self.focused_list();
        if len == 0 {
            return;
        }
        let new_idx = if idx == 0 { len - 1 } else { idx - 1 };
        self.set_focused_idx(new_idx);
    }

    /// 키 이벤트를 상태에 반영한다.
    ///
    /// @trace SPEC: SPEC-001
    /// @trace TC: TC-5, TC-6
    /// @trace FR: PRD-001/FR-3, PRD-001/FR-4
    pub fn handle_key(&mut self, key: KeyCode) {
        match key {
            | KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            | KeyCode::Tab => {
                self.focus = match self.focus {
                    | Focus::Scenarios => Focus::Reports,
                    | Focus::Reports => Focus::Scenarios,
                };
            },
            | KeyCode::Down | KeyCode::Char('j') => self.next(),
            | KeyCode::Up | KeyCode::Char('k') => self.prev(),
            | _ => {},
        }
    }

    fn focused_list(&self) -> (usize, usize) {
        match self.focus {
            | Focus::Scenarios => (self.scenario_idx, self.scenarios.len()),
            | Focus::Reports => (self.report_idx, self.reports.len()),
        }
    }

    fn set_focused_idx(&mut self, idx: usize) {
        match self.focus {
            | Focus::Scenarios => self.scenario_idx = idx,
            | Focus::Reports => self.report_idx = idx,
        }
    }
}

fn load_files(dir: &Path, ext: &str) -> io::Result<Vec<String>> {
    let mut out = Vec::new();
    if !dir.exists() {
        return Ok(out);
    }
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some(ext) {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                out.push(name.to_string());
            }
        }
    }
    out.sort();
    Ok(out)
}

#[cfg(test)]
mod tests {
    // =============================================================================
    // @trace SPEC-001
    // @trace PRD: PRD-001
    // @trace FR: FR-2, FR-3, FR-4
    // @trace file-type: test
    // =============================================================================

    use super::*;
    use std::{fs::File,
              io::Write};
    use tempfile::tempdir;

    /// @trace TC: SPEC-001/TC-2
    /// @trace FR: PRD-001/FR-2
    /// @trace scenario: 시나리오 목록 로딩
    #[test]
    fn test_tc_2_load_scenarios() {
        let scen = tempdir().unwrap();
        let reps = tempdir().unwrap();
        File::create(scen.path().join("cs.yaml")).unwrap().write_all(b"x").unwrap();
        File::create(scen.path().join("fin.yaml")).unwrap().write_all(b"x").unwrap();
        let state = TuiState::new(scen.path(), reps.path()).unwrap();
        assert_eq!(state.scenarios.len(), 2);
        assert!(state.scenarios.contains(&"cs.yaml".to_string()));
    }

    /// @trace TC: SPEC-001/TC-3
    /// @trace FR: PRD-001/FR-2
    /// @trace scenario: 선택 인덱스 이동
    #[test]
    fn test_tc_3_next_prev_moves_index() {
        let scen = tempdir().unwrap();
        let reps = tempdir().unwrap();
        for n in ["a.yaml", "b.yaml", "c.yaml"] {
            File::create(scen.path().join(n)).unwrap().write_all(b"x").unwrap();
        }
        let mut state = TuiState::new(scen.path(), reps.path()).unwrap();
        assert_eq!(state.scenario_idx, 0);
        state.next();
        assert_eq!(state.scenario_idx, 1);
        state.next();
        assert_eq!(state.scenario_idx, 2);
        state.next();
        assert_eq!(state.scenario_idx, 0, "wraps around");
        state.prev();
        assert_eq!(state.scenario_idx, 2);
    }

    /// @trace TC: SPEC-001/TC-4
    /// @trace FR: PRD-001/FR-3
    /// @trace scenario: 리포트 파일 목록 로딩
    #[test]
    fn test_tc_4_load_reports() {
        let scen = tempdir().unwrap();
        let reps = tempdir().unwrap();
        File::create(reps.path().join("r1.json")).unwrap().write_all(b"{}").unwrap();
        File::create(reps.path().join("ignore.txt")).unwrap().write_all(b"x").unwrap();
        let state = TuiState::new(scen.path(), reps.path()).unwrap();
        assert_eq!(state.reports, vec!["r1.json".to_string()]);
    }

    /// @trace TC: SPEC-001/TC-5
    /// @trace FR: PRD-001/FR-3
    /// @trace scenario: Tab으로 포커스 전환
    #[test]
    fn test_tc_5_tab_toggles_focus() {
        let scen = tempdir().unwrap();
        let reps = tempdir().unwrap();
        let mut state = TuiState::new(scen.path(), reps.path()).unwrap();
        assert_eq!(state.focus, Focus::Scenarios);
        state.handle_key(KeyCode::Tab);
        assert_eq!(state.focus, Focus::Reports);
        state.handle_key(KeyCode::Tab);
        assert_eq!(state.focus, Focus::Scenarios);
    }

    /// @trace TC: SPEC-001/TC-6
    /// @trace FR: PRD-001/FR-4
    /// @trace scenario: q/Esc 입력 시 should_quit=true
    #[test]
    fn test_tc_6_quit_keys_set_should_quit() {
        let scen = tempdir().unwrap();
        let reps = tempdir().unwrap();
        let mut state = TuiState::new(scen.path(), reps.path()).unwrap();
        assert!(!state.should_quit);
        state.handle_key(KeyCode::Char('q'));
        assert!(state.should_quit);

        let mut state2 = TuiState::new(scen.path(), reps.path()).unwrap();
        state2.handle_key(KeyCode::Esc);
        assert!(state2.should_quit);
    }
}
